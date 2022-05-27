// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    constants::SENSOR_MAXIMUM_COUNT,
    energy_usage::{QtSync, Signal},
    network::NetworkChannel,
};
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
        qt_tx: SyncSender<QtSync>,
        sensors: Arc<Mutex<Arc<SensorHashMap>>>,
        sensors_changed: Arc<AtomicBool>,
        update_requester: cxx_qt_lib::UpdateRequester,
    ) {
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
                        qt_tx
                            .send(QtSync::SignalChange(Signal::SensorRemoved {
                                uuid: uuid.to_string(),
                            }))
                            .unwrap();
                        update_requester.request_update();
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
                                qt_tx
                                    .send(QtSync::SignalChange(Signal::SensorChanged {
                                        uuid: uuid.to_string(),
                                    }))
                                    .unwrap();
                                update_requester.request_update();
                            } else {
                                qt_tx
                                    .send(QtSync::SignalChange(Signal::SensorAdded {
                                        uuid: uuid.to_string(),
                                    }))
                                    .unwrap();
                                update_requester.request_update();
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
