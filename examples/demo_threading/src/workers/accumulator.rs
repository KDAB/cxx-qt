// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::sensors::{SensorHashMap, SensorsWorker};
use crate::{constants::SENSOR_UPDATE_POLL_RATE, ffi::EnergyUsageCxxQtThread, Data};
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
        qt_data_tx: SyncSender<Data>,
        sensors: Arc<Mutex<Arc<SensorHashMap>>>,
        sensors_changed: Arc<AtomicBool>,
        qt_thread: cxx::UniquePtr<EnergyUsageCxxQtThread>,
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
                qt_data_tx
                    .send(Data {
                        average_use,
                        sensors,
                        total_use,
                    })
                    .unwrap();

                // Send a request to Qt that it should update
                qt_thread
                    .queue(|mut qobject_energy_usage| {
                        // TODO: for now we use the unsafe rust_mut() API
                        // later there will be getters and setters for the properties
                        unsafe {
                            // Process the new data from the background thread
                            let datas = qobject_energy_usage
                                .as_mut()
                                .rust_mut()
                                .qt_data_rx
                                .try_iter()
                                .collect::<Vec<Data>>();

                            // TODO: can we do this in the same loop?
                            for data in datas {
                                // Here we have constructed a new Data struct so can consume it's values
                                // for other uses we could have passed an Enum across the channel
                                // and then process the required action here
                                //
                                // The Q_PROPERTYs have changed, so load all the values from Data again
                                //
                                // Note that all the Q_PROPERTYs always change in this example, we could
                                // also have an enum per Q_PROPERTY and use the individual setters
                                qobject_energy_usage.as_mut().grab_values_from_data(data);
                            }
                        }
                    })
                    .unwrap();
            }
        }
    }
}
