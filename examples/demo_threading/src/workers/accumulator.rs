// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::sensors::{SensorHashMap, SensorsWorker};
use crate::{
    constants::SENSOR_UPDATE_POLL_RATE,
    energy_usage::{Data, QtSync},
};
use futures_timer::Delay;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::SyncSender,
    Arc, Mutex,
};

/// Define a worker which accumulates sensor values
pub struct AccumulatorWorker;

impl AccumulatorWorker {
    /// Start our accumulator thread
    ///
    /// It polls for sensors changing via a given AtomicBool, then if values
    /// have changed it accumulates the data (eg into total, average etc) and
    /// then requests an update Qt via the channel
    pub async fn run(
        qt_tx: SyncSender<QtSync>,
        sensors: Arc<Mutex<Arc<SensorHashMap>>>,
        sensors_changed: Arc<AtomicBool>,
        update_requester: cxx_qt_lib::UpdateRequester,
    ) {
        loop {
            // Wait at the given poll rate
            Delay::new(SENSOR_UPDATE_POLL_RATE).await;

            // Check the sensors have changed and if we need to do any work
            if sensors_changed
                .compare_exchange_weak(true, false, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                // If there is new sensor info then build average, count, and total
                let sensors = SensorsWorker::read_sensors(&sensors);
                let total_use = sensors.values().fold(0.0, |acc, x| acc + x.power);
                let sensors = sensors.len() as u32;
                let average_use = if sensors > 0 {
                    total_use / (sensors as f64)
                } else {
                    0.0
                };

                // Send the new data into the Qt channel
                qt_tx
                    .send(QtSync::DataChange(Data {
                        average_use,
                        sensors,
                        total_use,
                    }))
                    .unwrap();
                // Send a request to Qt that it should update
                update_requester.request_update();
            }
        }
    }
}
