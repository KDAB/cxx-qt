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
    use std::{collections::HashMap, thread::JoinHandle};
    use uuid::Uuid;

    enum EventArrived {
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
        event_sender: Sender<EventArrived>,
        event_receiver: Receiver<EventArrived>,
        join_handle: Option<JoinHandle<()>>,
        sensors: HashMap<Uuid, f64>,
    }

    impl Default for RustObj {
        fn default() -> Self {
            let (event_sender, event_receiver) = channel(4096);

            Self {
                event_sender,
                event_receiver,
                join_handle: None,
                sensors: HashMap::new(),
            }
        }
    }

    impl RustObj {
        async fn handle_connection(
            mut stream: TcpStream,
            mut event_sender: Sender<EventArrived>,
            update_requester: cxx_qt_lib::update_requester::UpdateRequester,
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
                        event_sender.try_send(EventArrived::Connect(uuid)).unwrap();
                        update_requester.request_update();

                        Response {
                            status: Status::Ok,
                            uuid: Some(uuid),
                        }
                    }
                    RequestCommand::Disconnect => {
                        if let Some(uuid) = request.uuid {
                            event_sender
                                .try_send(EventArrived::Disconnect(uuid))
                                .unwrap();
                            update_requester.request_update();

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
                            event_sender
                                .try_send(EventArrived::Power(uuid, value))
                                .unwrap();
                            update_requester.request_update();

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
            if self.join_handle.is_some() {
                println!("Already running a thread!");
                return;
            }

            // Prepare for moving into the thread
            let update_requester = cpp.update_requester();
            let event_sender = self.event_sender.clone();

            let run_server = async move {
                let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
                listener
                    .incoming()
                    .map(|stream| (stream, event_sender.clone(), update_requester.clone()))
                    .for_each_concurrent(
                        /* limit */ None,
                        |(stream, event_sender, update_requester)| async move {
                            let stream = stream.unwrap();
                            spawn(RustObj::handle_connection(
                                stream,
                                event_sender,
                                update_requester,
                            ));
                        },
                    )
                    .await;
            };

            // Start our thread
            self.join_handle = Some(std::thread::spawn(move || block_on(run_server)));
        }
    }

    impl UpdateRequestHandler<CppObj<'_>> for RustObj {
        fn handle_update_request(&mut self, cpp: &mut CppObj) {
            // Process each of the update requests from the background thread
            while let Ok(event) = self.event_receiver.try_next() {
                if let Some(event) = event {
                    match event {
                        // TODO: should this happen here or in the background thread?
                        EventArrived::Connect(uuid) => {
                            self.sensors.insert(uuid, 0.0);
                        }
                        EventArrived::Disconnect(uuid) => {
                            self.sensors.remove(&uuid);
                        }
                        EventArrived::Power(uuid, value) => {
                            *self.sensors.entry(uuid).or_default() = value;
                        }
                    }
                }
            }

            // TODO: should this happen on the "GUI" thread?
            let total = self.sensors.values().fold(0.0, |acc, x| acc + x);
            let count = self.sensors.len() as u32;
            cpp.set_total_use(total);
            if count != 0 {
                cpp.set_average_use(total / (count as f64));
            } else {
                cpp.set_average_use(0.0);
            }
            cpp.set_sensors(count);
        }
    }
}
