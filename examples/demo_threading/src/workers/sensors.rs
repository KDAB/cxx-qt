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

/// Define an individual sensors data
#[derive(Clone)]
pub struct SensorData {
    /// The power of the sensor
    pub power: f64,
    /// The last time that the sensor was seen
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

/// Define how we represent sensors as a hashmap of uuids to sensordata
pub type SensorHashMap = HashMap<Uuid, SensorData>;

pub enum SensorChanged {
    Added(String),
    Changed(String),
    Removed(String),
}

/// Define a worker which manages mutating the sensor hashmap
pub struct SensorsWorker;

impl SensorsWorker {
    /// Retrieve an Arc of the sensors hashmap that is used for reading
    ///
    /// This uses Arc::clone to provide a reference to the data with minimal locking time
    /// if the SensorWorker uses Arc::make_mut to mutate the data while we still hold
    /// onto this Arc then the data will be cloned on demand.
    pub fn read_sensors(sensors: &Arc<Mutex<Arc<SensorHashMap>>>) -> Arc<SensorHashMap> {
        let sensors_lock = sensors.lock().unwrap();
        let sensors = Arc::clone(&*sensors_lock);
        drop(sensors_lock);

        sensors
    }

    /// Start our SensorsWorker thread
    ///
    /// It listens for valid network requests, then when available it mutates
    /// the sensors hashmap to the new state. It then notifies Qt of any signal
    /// changes that need to occur and flips the AtomicBool for the AccumulatorWorker
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
                        // A Q_SIGNALS has been requested so emit it
                        qobject_energy_usage.as_mut().emit_queued(signal);
                    }
                }
            };

        loop {
            // Wait for a valid network request
            if let Ok(command) = network_rx.recv() {
                match command {
                    // A sensor should be remove from the hashmap
                    NetworkChannel::Disconnect { uuid } => {
                        // Remove the sensor from the hashmap
                        {
                            let mut sensors_lock = sensors.lock().unwrap();
                            let sensors = Arc::make_mut(&mut *sensors_lock);
                            sensors.remove(&uuid);
                        }
                        // Tell AccumulatorWorker that data has changed
                        sensors_changed.store(true, Ordering::SeqCst);

                        // Queue a Signal that the sensor has been removed into the Qt channel
                        qt_signals_tx
                            .send(SensorChanged::Removed(uuid.to_string()))
                            .unwrap();
                        // Send a request to Qt that it should update
                        qt_thread.queue(queue_process_signal_change).unwrap();
                    }
                    // A new or existing sensor has a power value
                    NetworkChannel::Power { uuid, value } => {
                        let mut sensors_lock = sensors.lock().unwrap();
                        let sensors = Arc::make_mut(&mut *sensors_lock);
                        let sensors_len = sensors.len();
                        let entry = sensors.entry(uuid);
                        let is_occupied =
                            matches!(entry, std::collections::hash_map::Entry::Occupied(..));

                        // Validate that we would still be below the sensors max count
                        if sensors_len < SENSOR_MAXIMUM_COUNT || is_occupied {
                            // Insert or modify the sensor entry to have the power and last seen
                            let mut sensor = entry.or_default();
                            sensor.power = value;
                            sensor.last_seen = SystemTime::now();
                            drop(sensors_lock);
                            // Tell AccumulatorWorker that data has changed
                            sensors_changed.store(true, Ordering::SeqCst);

                            // Queue a Signal that there is a new sensor or an existing sensor has
                            // changed into the Qt channel
                            if is_occupied {
                                qt_signals_tx
                                    .send(SensorChanged::Changed(uuid.to_string()))
                                    .unwrap();
                            } else {
                                qt_signals_tx
                                    .send(SensorChanged::Added(uuid.to_string()))
                                    .unwrap();
                            }

                            // Send a request to Qt that it should update
                            qt_thread.queue(queue_process_signal_change).unwrap();
                        } else {
                            println!("Maximum sensor count reached!");
                        }
                    }
                }
            }
        }
    }
}
