// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{constants::SENSOR_MAXIMUM_COUNT, network::NetworkChannel, qobject};
use cxx_qt::CxxQtThread;
use cxx_qt_lib::QString;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Receiver,
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
        sensors: Arc<Mutex<Arc<SensorHashMap>>>,
        sensors_changed: Arc<AtomicBool>,
        qt_thread: CxxQtThread<qobject::EnergyUsage>,
    ) {
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

                        // Queue a Signal that the sensor has been removed to Qt
                        qt_thread
                            .queue(
                                move |qobject_energy_usage: std::pin::Pin<
                                    &mut qobject::EnergyUsage,
                                >| {
                                    qobject_energy_usage
                                        .sensor_removed(QString::from(&uuid.to_string()));
                                },
                            )
                            .unwrap();
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
                            let sensor = entry.or_default();
                            sensor.power = value;
                            sensor.last_seen = SystemTime::now();
                            drop(sensors_lock);
                            // Tell AccumulatorWorker that data has changed
                            sensors_changed.store(true, Ordering::SeqCst);

                            // Queue a Signal that there is a new sensor or an existing sensor has
                            // changed to Qt
                            if is_occupied {
                                qt_thread
                                    .queue(
                                        move |qobject_energy_usage: std::pin::Pin<
                                            &mut qobject::EnergyUsage,
                                        >| {
                                            qobject_energy_usage
                                                .sensor_changed(QString::from(&uuid.to_string()));
                                        },
                                    )
                                    .unwrap();
                            } else {
                                qt_thread
                                    .queue(
                                        move |qobject_energy_usage: std::pin::Pin<
                                            &mut qobject::EnergyUsage,
                                        >| {
                                            qobject_energy_usage
                                                .sensor_added(QString::from(&uuid.to_string()));
                                        },
                                    )
                                    .unwrap();
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
