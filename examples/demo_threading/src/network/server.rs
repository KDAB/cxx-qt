// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use async_std::{
    net::{TcpListener, TcpStream},
    prelude::*,
    task::spawn,
};
use futures::stream::StreamExt;
use std::sync::mpsc::{SyncSender, TrySendError};
use uuid::Uuid;

use super::{
    request::{Request, RequestCommand},
    response::{Response, Status},
};
use crate::constants::SENSOR_MAXIMUM_POWER;

/// Definition of the network channel
pub enum NetworkChannel {
    /// The given sensor should be disconnected
    Disconnect { uuid: Uuid },
    /// The given sensor has a new power value
    Power { uuid: Uuid, value: f64 },
}

/// Define converting from a channel error to a network reponse
impl From<Result<(), std::sync::mpsc::TrySendError<NetworkChannel>>> for Response {
    fn from(result: Result<(), std::sync::mpsc::TrySendError<NetworkChannel>>) -> Self {
        match result {
            Ok(_) => Status::Ok,
            Err(TrySendError::Full { .. }) => Status::ErrorServerQueueFull,
            Err(TrySendError::Disconnected { .. }) => Status::ErrorServerDisconnected,
        }
        .into()
    }
}

/// Define a network server for managing the TCP connections
pub struct NetworkServer;

impl NetworkServer {
    /// Read from a TCP stream and create a Request
    ///
    /// This deserialises the request into our Rust objects
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

    /// Handle a network connection
    ///
    /// This reads the TCP streams, checks the request is valid, and then triggers
    /// packets on the network channel
    async fn handle_connection(mut stream: TcpStream, network_tx: SyncSender<NetworkChannel>) {
        let response: Response = match Self::build_request(&mut stream).await {
            Ok(request) => {
                // The network request was deserialised successfully so process the request
                match request.command {
                    RequestCommand::Power { value } => {
                        // Validate that our power is within the expected range
                        if (0.0..=SENSOR_MAXIMUM_POWER).contains(&value) {
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

        // Write our response to the TCP stream
        stream
            .write(serde_json::to_string(&response).unwrap().as_bytes())
            .await
            .ok();
        stream.flush().await.unwrap();
    }

    /// Start a server which binds to the given address
    pub async fn listen(address: &str, network_tx: SyncSender<NetworkChannel>) {
        let listener = TcpListener::bind(address).await.unwrap();
        listener
            .incoming()
            .map(|stream| (stream, network_tx.clone()))
            .for_each_concurrent(/* limit */ None, |(stream, network_tx)| async move {
                let stream = stream.unwrap();
                spawn(Self::handle_connection(stream, network_tx));
            })
            .await;
    }
}
