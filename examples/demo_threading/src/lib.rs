// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt::make_qobject;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RequestCommand {
    // TODO: should we have connect + disconnect + power?
    // or just power + disconnect and accept any new uuid as a new sensor,
    // then have graceful disconnect?
    Connect,
    Disconnect,
    Power { value: f64 },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    Ok,
    Error,
    ErrorNoUuid,
    ErrorInvalidPower,
}

#[derive(Deserialize, Serialize)]
struct Request {
    command: RequestCommand,
    uuid: Option<Uuid>,
}

#[derive(Deserialize, Serialize)]
struct Response {
    status: Status,
    uuid: Option<Uuid>,
}

#[make_qobject]
mod energy_usage {
    use super::{Request, RequestCommand, Response, Status};
    use async_std::{
        net::{TcpListener, TcpStream},
        prelude::*,
        task::spawn,
    };
    use futures::{
        channel::mpsc::{channel, Receiver, Sender},
        executor::block_on,
        stream::StreamExt,
    };
    use futures_timer::Delay;
    use std::{collections::HashMap, thread::JoinHandle, time::Duration};
    use uuid::Uuid;

    enum QtValueArrived {
        AverageUseChanged(f64),
        SensorsChanged(u32),
        TotalUsageChanged(f64),
    }

    enum NetworkDataArrived {
        Connect(Uuid),
        Disconnect(Uuid),
        Power(Uuid, f64),
    }

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
        qt_sender: Sender<QtValueArrived>,
        qt_receiver: Receiver<QtValueArrived>,
        join_handle_network: Option<JoinHandle<()>>,
        join_handle_processing: Option<JoinHandle<()>>,
    }

    impl Default for RustObj {
        fn default() -> Self {
            let (qt_sender, qt_receiver) = channel(4096);

            Self {
                qt_sender,
                qt_receiver,
                join_handle_network: None,
                join_handle_processing: None,
            }
        }
    }

    impl RustObj {
        async fn handle_connection(
            mut stream: TcpStream,
            mut event_sender: Sender<NetworkDataArrived>,
        ) {
            let mut buf = vec![0u8; 1024];
            let _ = stream.read(&mut buf).await.unwrap();
            let trimmed = std::str::from_utf8(&buf)
                .unwrap()
                .trim_matches(|c| c == ' ' || c == '\n' || c == '\r' || c == '\0');

            let response = match serde_json::from_str::<Request>(trimmed) {
                Ok(request) => match request.command {
                    RequestCommand::Connect => {
                        let uuid = Uuid::new_v4();
                        event_sender
                            .try_send(NetworkDataArrived::Connect(uuid))
                            .unwrap();

                        Response {
                            status: Status::Ok,
                            uuid: Some(uuid),
                        }
                    }
                    RequestCommand::Disconnect => {
                        if let Some(uuid) = request.uuid {
                            event_sender
                                .try_send(NetworkDataArrived::Disconnect(uuid))
                                .unwrap();

                            Response {
                                status: Status::Ok,
                                uuid: Some(uuid),
                            }
                        } else {
                            Response {
                                status: Status::ErrorNoUuid,
                                uuid: None,
                            }
                        }
                    }
                    RequestCommand::Power { value } => {
                        if let Some(uuid) = request.uuid {
                            // Validate that our power is within the expected range
                            if value < 0.0 || value > 1000.0 {
                                Response {
                                    status: Status::ErrorInvalidPower,
                                    uuid: None,
                                }
                            } else {
                                event_sender
                                    .try_send(NetworkDataArrived::Power(uuid, value))
                                    .unwrap();

                                Response {
                                    status: Status::Ok,
                                    uuid: Some(uuid),
                                }
                            }
                        } else {
                            Response {
                                status: Status::ErrorNoUuid,
                                uuid: None,
                            }
                        }
                    }
                },
                Err(_) => Response {
                    status: Status::Error,
                    uuid: None,
                },
            };

            stream
                .write(serde_json::to_string(&response).unwrap().as_bytes())
                .await
                .unwrap();
            stream.flush().await.unwrap();
        }

        #[invokable]
        fn start_server(&mut self, cpp: &mut CppObj) {
            if self.join_handle_network.is_some() || self.join_handle_processing.is_some() {
                println!("Already running a thread!");
                return;
            }

            let (network_sender, mut network_receiver) = channel(4096);
            let mut qt_sender = self.qt_sender.clone();
            let update_requester = cpp.update_requester();

            // Prepare our processing thread which builds average, count, total
            let run_processing = async move {
                let mut sensors: HashMap<Uuid, f64> = HashMap::new();

                loop {
                    let mut changed = false;

                    Delay::new(Duration::from_millis(8)).await;

                    // Read our channel of sensor data from the network thread
                    while let Ok(event) = network_receiver.try_next() {
                        if let Some(event) = event {
                            changed = true;

                            match event {
                                NetworkDataArrived::Connect(uuid) => {
                                    sensors.insert(uuid, 0.0);
                                }
                                NetworkDataArrived::Disconnect(uuid) => {
                                    sensors.remove(&uuid);
                                }
                                NetworkDataArrived::Power(uuid, value) => {
                                    *sensors.entry(uuid).or_default() = value;
                                }
                            }
                        }
                    }

                    // If there is new sensor info then build average, count, total and inform Qt
                    if changed {
                        let total = sensors.values().fold(0.0, |acc, x| acc + x);
                        let count = sensors.len() as u32;
                        let average = if count > 0 {
                            total / (count as f64)
                        } else {
                            0.0
                        };

                        qt_sender
                            .try_send(QtValueArrived::TotalUsageChanged(total))
                            .unwrap();
                        qt_sender
                            .try_send(QtValueArrived::SensorsChanged(count))
                            .unwrap();
                        qt_sender
                            .try_send(QtValueArrived::AverageUseChanged(average))
                            .unwrap();

                        update_requester.request_update();
                    }
                }
            };

            // Prepare our Tcp server which listens for sensors
            let run_server = async move {
                let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
                listener
                    .incoming()
                    .map(|stream| (stream, network_sender.clone()))
                    .for_each_concurrent(
                        /* limit */ None,
                        |(stream, network_sender)| async move {
                            let stream = stream.unwrap();
                            spawn(RustObj::handle_connection(stream, network_sender));
                        },
                    )
                    .await;
            };

            // Start our threads
            self.join_handle_processing =
                Some(std::thread::spawn(move || block_on(run_processing)));
            self.join_handle_network = Some(std::thread::spawn(move || block_on(run_server)));
        }
    }

    impl UpdateRequestHandler<CppObj<'_>> for RustObj {
        fn handle_update_request(&mut self, cpp: &mut CppObj) {
            // Process each of the update requests from the background thread
            while let Ok(event) = self.qt_receiver.try_next() {
                if let Some(event) = event {
                    match event {
                        QtValueArrived::AverageUseChanged(average) => cpp.set_average_use(average),
                        QtValueArrived::SensorsChanged(sensors) => cpp.set_sensors(sensors),
                        QtValueArrived::TotalUsageChanged(total) => cpp.set_total_use(total),
                    }
                }
            }
        }
    }
}
