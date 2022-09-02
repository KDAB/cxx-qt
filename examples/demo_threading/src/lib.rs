// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use serde::{Deserialize, Serialize};
use std::{
    sync::mpsc::TrySendError,
    time::{Duration, SystemTime},
};
use uuid::Uuid;

/// The size of the network thread to update thread queue
const CHANNEL_NETWORK_COUNT: usize = 1_024;
/// The size of the update thread to Qt queue
const CHANNEL_QT_COUNT: usize = 250;
/// After how many milliseconds should a sensor be disconnected and considered missing
const SENSOR_TIMEOUT: Duration = Duration::from_millis(10_000);
/// How often should the timeout thread poll sensors
const SENSOR_TIMEOUT_POLL_RATE: Duration = Duration::from_millis(256);
/// How often should the update thread poll sensors
const SENSOR_UPDATE_POLL_RATE: Duration = Duration::from_millis(128);
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
    ErrorFailedToRead,
    ErrorFailedToParseAsUtf8,
    ErrorFailedToParseJSONRequest,
    ErrorInvalidReadSize,
    ErrorInvalidPower,
    ErrorServerQueueFull,
    ErrorServerDisconnected,
}

#[derive(Deserialize, Serialize)]
struct Response {
    status: Status,
}

impl From<serde_json::Error> for Status {
    fn from(_: serde_json::Error) -> Self {
        Status::ErrorFailedToParseJSONRequest
    }
}

impl From<std::str::Utf8Error> for Status {
    fn from(_: std::str::Utf8Error) -> Self {
        Status::ErrorFailedToParseAsUtf8
    }
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
    Update,
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

struct QtData {
    average_use: f64,
    sensors: u32,
    total_use: f64,
}

#[cxx_qt::bridge(namespace = "cxx_qt::energy_usage")]
mod ffi {
    use super::{NetworkChannel, QtData, Request, RequestCommand, Response, SensorData, Status};
    use async_std::{
        net::{TcpListener, TcpStream},
        prelude::*,
        task::spawn,
    };
    use futures::{executor::block_on, stream::StreamExt};
    use futures_timer::Delay;
    use std::{
        collections::HashMap,
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::{sync_channel, Receiver, SyncSender},
            Arc,
        },
        thread::JoinHandle,
        time::SystemTime,
    };
    use uuid::Uuid;

    #[cxx_qt::qobject]
    pub struct EnergyUsage {
        #[qproperty]
        average_use: f64,
        #[qproperty]
        sensors: u32,
        #[qproperty]
        total_use: f64,

        qt_rx: Receiver<QtData>,
        qt_tx: SyncSender<QtData>,
        join_handles: Option<[JoinHandle<()>; 4]>,
    }

    impl Default for EnergyUsage {
        fn default() -> Self {
            let (qt_tx, qt_rx) = sync_channel(super::CHANNEL_QT_COUNT);
            Self {
                average_use: 0.0,
                sensors: 0,
                total_use: 0.0,

                qt_rx,
                qt_tx,
                join_handles: None,
            }
        }
    }

    impl EnergyUsage {
        /// Read from a TCP stream and create a Request
        async fn build_request(stream: &mut TcpStream) -> Result<Request, Status> {
            let mut buf = vec![0u8; 128];
            if let Ok(size) = stream.read(&mut buf).await {
                if size > buf.len() {
                    Err(Status::ErrorInvalidReadSize)
                } else {
                    let trimmed = std::str::from_utf8(&buf)?
                        .trim_matches(|c| c == ' ' || c == '\n' || c == '\r' || c == '\0');
                    serde_json::from_str::<Request>(trimmed).map_err(|e| e.into())
                }
            } else {
                Err(Status::ErrorFailedToRead)
            }
        }

        async fn handle_connection(mut stream: TcpStream, network_tx: SyncSender<NetworkChannel>) {
            let response: Response = match EnergyUsage::build_request(&mut stream).await {
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
                Err(err) => err.into(),
            };

            stream
                .write(serde_json::to_string(&response).unwrap().as_bytes())
                .await
                .ok();
            stream.flush().await.unwrap();
        }
    }

    impl cxx_qt::QObject<EnergyUsage> {
        #[qinvokable]
        pub fn start_server(self: Pin<&mut Self>) {
            if self.rust().join_handles.is_some() {
                println!("Already running a server!");
                return;
            }

            let (network_tx, network_rx) = sync_channel(super::CHANNEL_NETWORK_COUNT);
            let (timeout_tx, timeout_rx) = sync_channel::<HashMap<Uuid, SensorData>>(0);
            let (update_tx, update_rx) = sync_channel::<HashMap<Uuid, SensorData>>(0);
            let sensors_changed = Arc::new(AtomicBool::new(false));

            // Prepare our timeout thread, if a sensor is not seen for N seconds we remove it
            let timeout_network_tx = network_tx.clone();
            let run_timeout = async move {
                loop {
                    Delay::new(super::SENSOR_TIMEOUT_POLL_RATE).await;

                    timeout_network_tx
                        .send(NetworkChannel::TimeoutUpdate)
                        .unwrap();

                    if let Ok(mut sensors) = timeout_rx.recv() {
                        for uuid in sensors
                            .drain()
                            // Find sensors that have expired
                            .filter(|(_, sensor)| {
                                if let Ok(duration) = sensor.last_seen.elapsed() {
                                    duration > super::SENSOR_TIMEOUT
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

            // Prepare our update thread
            //
            // When values change this then requests an update to Qt
            let qt_tx = self.rust().qt_tx.clone();
            let update_network_tx = network_tx.clone();
            let qt_thread = self.qt_thread();
            let update_sensors_changed = sensors_changed.clone();
            let run_update = async move {
                loop {
                    Delay::new(super::SENSOR_UPDATE_POLL_RATE).await;

                    if update_sensors_changed
                        .compare_exchange_weak(true, false, Ordering::SeqCst, Ordering::SeqCst)
                        .is_ok()
                    {
                        update_network_tx.send(NetworkChannel::Update).unwrap();

                        // If there is new sensor info then build average, count, total and inform Qt
                        if let Ok(sensors) = update_rx.recv() {
                            let total_use = sensors.values().fold(0.0, |acc, x| acc + x.power);
                            let sensors = sensors.len() as u32;
                            let average_use = if sensors > 0 {
                                total_use / (sensors as f64)
                            } else {
                                0.0
                            };

                            qt_tx
                                .send(QtData {
                                    average_use,
                                    sensors,
                                    total_use,
                                })
                                .unwrap();

                            qt_thread
                                .queue(|mut qobject_energy_usage| {
                                    // TODO: for now we use the unsafe rust_mut() API
                                    // later there will be getters and setters for the properties
                                    unsafe {
                                        // Process the new data from the background thread
                                        if let Some(data) = qobject_energy_usage
                                            .as_mut()
                                            .rust_mut()
                                            .qt_rx
                                            .try_iter()
                                            .last()
                                        {
                                            // Here we have constructed a new Data struct so can consume it's values
                                            // for other uses we could have passed an Enum across the channel
                                            // and then process the required action here
                                            qobject_energy_usage
                                                .as_mut()
                                                .set_average_use(data.average_use);
                                            qobject_energy_usage.as_mut().set_sensors(data.sensors);
                                            qobject_energy_usage
                                                .as_mut()
                                                .set_total_use(data.total_use);
                                        }
                                    }
                                })
                                .unwrap();
                        }
                    }
                }
            };

            // Prepare our sensors thread, which reads from the network channel and collates
            // the commands into a hashmap.
            //
            // The timeout and update thread can request snapshots of the sensors data
            let run_sensors = async move {
                let mut sensors =
                    HashMap::<Uuid, SensorData>::with_capacity(super::SENSOR_MAXIMUM_COUNT);

                loop {
                    if let Ok(command) = network_rx.recv() {
                        match command {
                            NetworkChannel::Disconnect { uuid } => {
                                sensors.remove(&uuid);
                                sensors_changed.store(true, Ordering::SeqCst);
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
                                    sensors_changed.store(true, Ordering::SeqCst);
                                } else {
                                    println!("Maximum sensor count reached!");
                                }
                            }
                            NetworkChannel::TimeoutUpdate => {
                                timeout_tx.send(sensors.clone()).unwrap();
                            }
                            NetworkChannel::Update => {
                                update_tx.send(sensors.clone()).unwrap();
                            }
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
                        spawn(EnergyUsage::handle_connection(stream, network_tx));
                    })
                    .await;
            };

            // Start our threads
            unsafe {
                self.rust_mut().join_handles = Some([
                    std::thread::spawn(move || block_on(run_timeout)),
                    std::thread::spawn(move || block_on(run_update)),
                    std::thread::spawn(move || block_on(run_sensors)),
                    std::thread::spawn(move || block_on(run_server)),
                ]);
            }
        }
    }
}
