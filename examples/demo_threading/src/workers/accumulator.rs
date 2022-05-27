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

pub struct AccumulatorWorker;

impl AccumulatorWorker {
    pub async fn run(
        qt_tx: SyncSender<QtSync>,
        sensors: Arc<Mutex<Arc<SensorHashMap>>>,
        sensors_changed: Arc<AtomicBool>,
        update_requester: cxx_qt_lib::UpdateRequester,
    ) {
        loop {
            Delay::new(SENSOR_UPDATE_POLL_RATE).await;

            if sensors_changed
                .compare_exchange_weak(true, false, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                // If there is new sensor info then build average, count, total and inform Qt
                let sensors = SensorsWorker::read_sensors(&sensors);
                let total_use = sensors.values().fold(0.0, |acc, x| acc + x.power);
                let sensors = sensors.len() as u32;
                let average_use = if sensors > 0 {
                    total_use / (sensors as f64)
                } else {
                    0.0
                };

                qt_tx
                    .send(QtSync::DataChange(Data {
                        average_use,
                        sensors,
                        total_use,
                    }))
                    .unwrap();

                update_requester.request_update();
            }
        }
    }
}
