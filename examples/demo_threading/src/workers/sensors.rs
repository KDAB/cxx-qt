// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    constants::SENSOR_MAXIMUM_COUNT,
    ffi::{EnergyUsageCxxQtThread, EnergyUsageQt},
    network::NetworkChannel,
    EnergySignals,
};
use cxx_qt_lib::QString;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, SyncSender},
        Arc, Mutex,
    },
    time::SystemTime,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct SensorData {
    pub power: f64,
    pub last_seen: SystemTime,
}

impl Default for SensorData {
    fn default() -> Self {
        Self {
            power: 0.0,
            last_seen: SystemTime::now(),
        }
    }
}

pub type SensorHashMap = HashMap<Uuid, SensorData>;

pub enum SensorChanged {
    Added(String),
    Changed(String),
    Removed(String),
}

pub struct SensorsWorker;

impl SensorsWorker {
    pub fn read_sensors(sensors: &Arc<Mutex<Arc<SensorHashMap>>>) -> Arc<SensorHashMap> {
        let sensors_lock = sensors.lock().unwrap();
        let sensors = Arc::clone(&*sensors_lock);
        drop(sensors_lock);

        sensors
    }

    pub async fn run(
        network_rx: Receiver<NetworkChannel>,
        qt_signals_tx: SyncSender<SensorChanged>,
        sensors: Arc<Mutex<Arc<SensorHashMap>>>,
        sensors_changed: Arc<AtomicBool>,
        qt_thread: cxx::UniquePtr<EnergyUsageCxxQtThread>,
    ) {
        // Emit the signal event on the Qt thread
        let queue_process_signal_change =
            |mut qobject_energy_usage: std::pin::Pin<&mut EnergyUsageQt>| {
                // TODO: for now we use the unsafe rust_mut() API
                // later there will be getters and setters for the properties
                unsafe {
                    // Process the new data from the background thread
                    let signals = qobject_energy_usage
                        .as_mut()
                        .rust_mut()
                        .qt_signals_rx
                        .try_iter()
                        .map(|packet| match packet {
                            SensorChanged::Added(uuid) => EnergySignals::SensorAdded {
                                uuid: QString::from_str(&uuid),
                            },
                            SensorChanged::Changed(uuid) => EnergySignals::SensorChanged {
                                uuid: QString::from_str(&uuid),
                            },
                            SensorChanged::Removed(uuid) => EnergySignals::SensorRemoved {
                                uuid: QString::from_str(&uuid),
                            },
                        })
                        .collect::<Vec<EnergySignals>>();

                    // TODO: once emit_queued is not a mut then this can be in the same loop?
                    for signal in signals {
                        qobject_energy_usage.as_mut().emit_queued(signal);
                    }
                }
            };

        loop {
            if let Ok(command) = network_rx.recv() {
                match command {
                    NetworkChannel::Disconnect { uuid } => {
                        {
                            let mut sensors_lock = sensors.lock().unwrap();
                            let sensors = Arc::make_mut(&mut *sensors_lock);
                            sensors.remove(&uuid);
                        }
                        sensors_changed.store(true, Ordering::SeqCst);
                        qt_signals_tx
                            .send(SensorChanged::Removed(uuid.to_string()))
                            .unwrap();
                        qt_thread.queue(queue_process_signal_change).unwrap();
                    }
                    NetworkChannel::Power { uuid, value } => {
                        let mut sensors_lock = sensors.lock().unwrap();
                        let sensors = Arc::make_mut(&mut *sensors_lock);
                        // Validate that we would still be below the sensors max count
                        let sensors_len = sensors.len();
                        let entry = sensors.entry(uuid);
                        let is_occupied =
                            matches!(entry, std::collections::hash_map::Entry::Occupied(..));
                        if sensors_len < SENSOR_MAXIMUM_COUNT || is_occupied {
                            let mut sensor = entry.or_default();
                            sensor.power = value;
                            sensor.last_seen = SystemTime::now();
                            drop(sensors_lock);

                            sensors_changed.store(true, Ordering::SeqCst);

                            if is_occupied {
                                qt_signals_tx
                                    .send(SensorChanged::Changed(uuid.to_string()))
                                    .unwrap();
                                qt_thread.queue(queue_process_signal_change).unwrap();
                            } else {
                                qt_signals_tx
                                    .send(SensorChanged::Added(uuid.to_string()))
                                    .unwrap();
                                qt_thread.queue(queue_process_signal_change).unwrap();
                            }
                        } else {
                            println!("Maximum sensor count reached!");
                        }
                    }
                }
            }
        }
    }
}
