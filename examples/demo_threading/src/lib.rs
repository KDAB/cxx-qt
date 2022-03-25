// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt::make_qobject;

use serde::{Deserialize, Serialize};
use std::{sync::mpsc::TrySendError, time::SystemTime};
use uuid::Uuid;

/// The size of the network thread to update thread queue
const CHANNEL_NETWORK_COUNT: usize = 1_024;
/// The size of the update thread to Qt queue
const CHANNEL_QT_COUNT: usize = 16;
/// After how many milliseconds should a sensor be disconnected and considered missing
const SENSOR_TIMEOUT_MILLIS: u128 = 10_000;
/// How often should the timeout thread poll sensors
const SENSOR_TIMEOUT_POLL_RATE_MILLIS: u64 = 256;
/// The maximum number of sensors we will manage
const SENSOR_MAXIMUM_COUNT: usize = 1000;
/// The maximum power a sensor can report
const SENSOR_MAXIMUM_POWER: f64 = 1000.0;

// Network Serialisation definition
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RequestCommand {
    Disconnect,
    Power { value: f64 },
}

#[derive(Deserialize, Serialize)]
struct Request {
    command: RequestCommand,
    uuid: Uuid,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    Ok,
    ErrorInvalidPower,
    ErrorInvalidRequest,
    ErrorServerQueueFull,
    ErrorServerDisconnected,
}

#[derive(Deserialize, Serialize)]
struct Response {
    status: Status,
}

impl From<Status> for Response {
    fn from(status: Status) -> Self {
        Response { status }
    }
}

impl From<Result<(), std::sync::mpsc::TrySendError<NetworkChannel>>> for Response {
    fn from(result: Result<(), std::sync::mpsc::TrySendError<NetworkChannel>>) -> Self {
        match result {
            Ok(_) => Response { status: Status::Ok },
            Err(TrySendError::Full { .. }) => Response {
                status: Status::ErrorServerQueueFull,
            },
            Err(TrySendError::Disconnected { .. }) => Response {
                status: Status::ErrorServerDisconnected,
            },
        }
    }
}

// Channel definition
enum NetworkChannel {
    Disconnect { uuid: Uuid },
    Power { uuid: Uuid, value: f64 },
    TimeoutUpdate,
}

#[derive(Clone)]
struct SensorData {
    power: f64,
    last_seen: SystemTime,
}

impl Default for SensorData {
    fn default() -> Self {
        Self {
            power: 0.0,
            last_seen: SystemTime::now(),
        }
    }
}

#[make_qobject]
mod energy_usage {
    use super::{NetworkChannel, Request, RequestCommand, Response, SensorData, Status};
    use async_std::{
        net::{TcpListener, TcpStream},
        prelude::*,
        task::spawn,
    };
    use futures::{executor::block_on, stream::StreamExt};
    use futures_timer::Delay;
    use std::{
        collections::HashMap,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        thread::JoinHandle,
        time::{Duration, SystemTime},
    };
    use uuid::Uuid;

    pub struct Data {
        average_use: f64,
        sensors: u32,
        total_use: f64,
    }

    impl Default for Data {
        fn default() -> Self {
            Self {
                average_use: 0.0,
                sensors: 0,
                total_use: 0.0,
            }
        }
    }

    struct RustObj {
        qt_rx: Receiver<Data>,
        qt_tx: SyncSender<Data>,
        join_handle_network: Option<JoinHandle<()>>,
        join_handle_timeout: Option<JoinHandle<()>>,
        join_handle_update: Option<JoinHandle<()>>,
    }

    impl Default for RustObj {
        fn default() -> Self {
            let (qt_tx, qt_rx) = sync_channel(super::CHANNEL_QT_COUNT);
            Self {
                qt_rx,
                qt_tx,
                join_handle_network: None,
                join_handle_timeout: None,
                join_handle_update: None,
            }
        }
    }

    impl RustObj {
        async fn handle_connection(mut stream: TcpStream, network_tx: SyncSender<NetworkChannel>) {
            let mut buf = vec![0u8; 1024];
            let _ = stream.read(&mut buf).await.unwrap();
            let trimmed = std::str::from_utf8(&buf)
                .unwrap()
                .trim_matches(|c| c == ' ' || c == '\n' || c == '\r' || c == '\0');

            let response: Response = match serde_json::from_str::<Request>(trimmed) {
                Ok(request) => {
                    match request.command {
                        RequestCommand::Power { value } => {
                            // Validate that our power is within the expected range
                            if (0.0..=super::SENSOR_MAXIMUM_POWER).contains(&value) {
                                network_tx
                                    .try_send(NetworkChannel::Power {
                                        uuid: request.uuid,
                                        value,
                                    })
                                    .into()
                            } else {
                                Status::ErrorInvalidPower.into()
                            }
                        }
                        RequestCommand::Disconnect => network_tx
                            .try_send(NetworkChannel::Disconnect { uuid: request.uuid })
                            .into(),
                    }
                }
                Err(_) => Status::ErrorInvalidRequest.into(),
            };

            stream
                .write(serde_json::to_string(&response).unwrap().as_bytes())
                .await
                .unwrap();
            stream.flush().await.unwrap();
        }

        #[invokable]
        fn start_server(&mut self, cpp: &mut CppObj) {
            if self.join_handle_network.is_some()
                || self.join_handle_timeout.is_some()
                || self.join_handle_update.is_some()
            {
                println!("Already running a thread!");
                return;
            }

            let (network_tx, network_rx) = sync_channel(super::CHANNEL_NETWORK_COUNT);
            let (timeout_tx, timeout_rx) = sync_channel::<HashMap<Uuid, SensorData>>(0);

            // Prepare our timeout thread, if a sensor is not seen for N seconds we remove it
            let timeout_network_tx = network_tx.clone();
            let run_timeout = async move {
                loop {
                    Delay::new(Duration::from_millis(
                        super::SENSOR_TIMEOUT_POLL_RATE_MILLIS,
                    ))
                    .await;

                    timeout_network_tx
                        .send(NetworkChannel::TimeoutUpdate)
                        .unwrap();

                    if let Ok(mut sensors) = timeout_rx.recv() {
                        for uuid in sensors
                            .drain()
                            // Find sensors that have expired
                            .filter(|(_, sensor)| {
                                if let Ok(duration) = sensor.last_seen.elapsed() {
                                    duration.as_millis() > super::SENSOR_TIMEOUT_MILLIS
                                } else {
                                    true
                                }
                            })
                            .map(|(uuid, _)| uuid)
                        {
                            timeout_network_tx
                                .send(NetworkChannel::Disconnect { uuid })
                                .unwrap();
                        }
                    }
                }
            };

            // Prepare our update thread, which reads from the network channel and collates
            // the commands into a hashmap.
            // When values change this then requests an update to Qt
            let update_requester = cpp.update_requester();
            let qt_tx = self.qt_tx.clone();
            let run_update = async move {
                let mut sensors =
                    HashMap::<Uuid, SensorData>::with_capacity(super::SENSOR_MAXIMUM_COUNT);

                loop {
                    if let Ok(command) = network_rx.recv() {
                        let mut changed = false;

                        match command {
                            NetworkChannel::Disconnect { uuid } => {
                                sensors.remove(&uuid);
                                changed = true;
                            }
                            NetworkChannel::Power { uuid, value } => {
                                // Validate that we would still be below the sensors max count
                                let sensors_len = sensors.len();
                                let entry = sensors.entry(uuid);
                                if sensors_len < super::SENSOR_MAXIMUM_COUNT
                                    || matches!(
                                        entry,
                                        std::collections::hash_map::Entry::Occupied(..)
                                    )
                                {
                                    let mut sensor = entry.or_default();
                                    sensor.power = value;
                                    sensor.last_seen = SystemTime::now();
                                    changed = true;
                                } else {
                                    println!("Maximum sensor count reached!");
                                }
                            }
                            NetworkChannel::TimeoutUpdate => {
                                timeout_tx.send(sensors.clone()).unwrap();
                            }
                        }

                        // If there is new sensor info then build average, count, total and inform Qt
                        //
                        // Note that this part could be in a separate thread like the timeout
                        // then it could poll for a snapshot of the sensors every N milliseconds
                        if changed {
                            let total_use = sensors.values().fold(0.0, |acc, x| acc + x.power);
                            let sensors = sensors.len() as u32;
                            let average_use = if sensors > 0 {
                                total_use / (sensors as f64)
                            } else {
                                0.0
                            };

                            qt_tx
                                .send(Data {
                                    average_use,
                                    sensors,
                                    total_use,
                                })
                                .unwrap();

                            update_requester.request_update();
                        }
                    }
                }
            };

            // Prepare our Tcp server which listens for sensors
            let run_server = async move {
                let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
                listener
                    .incoming()
                    .map(|stream| (stream, network_tx.clone()))
                    .for_each_concurrent(/* limit */ None, |(stream, network_tx)| async move {
                        let stream = stream.unwrap();
                        spawn(RustObj::handle_connection(stream, network_tx));
                    })
                    .await;
            };

            // Start our threads
            self.join_handle_timeout = Some(std::thread::spawn(move || block_on(run_timeout)));
            self.join_handle_network = Some(std::thread::spawn(move || block_on(run_server)));
            self.join_handle_update = Some(std::thread::spawn(move || block_on(run_update)));
        }
    }

    impl UpdateRequestHandler<CppObj<'_>> for RustObj {
        fn handle_update_request(&mut self, cpp: &mut CppObj) {
            // Process the new data from the background thread
            if let Some(data) = self.qt_rx.try_iter().last() {
                // Here we have constructed a new Data struct so can consume it's values
                // for other uses we could have passed an Enum across the channel
                // and then process the required action here
                cpp.grab_values_from_data(data);
            }
        }
    }
}
