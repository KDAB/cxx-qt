// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::sensors::{SensorHashMap, SensorsWorker};
use crate::{
    constants::{SENSOR_TIMEOUT, SENSOR_TIMEOUT_POLL_RATE},
    network::NetworkChannel,
};
use futures_timer::Delay;
use std::sync::{mpsc::SyncSender, Arc, Mutex};

/// Define a worker which manages disconnecting sensors that have timed out
pub struct TimeoutWorker;

impl TimeoutWorker {
    /// Start our TimeoutWorker
    ///
    /// It polls for sensors that have a last seen value larger than sensor
    /// timeout. Then it uses the network channel to create a disconnect request
    pub async fn run(
        network_tx: SyncSender<NetworkChannel>,
        sensors: Arc<Mutex<Arc<SensorHashMap>>>,
    ) {
        loop {
            // Wait at the given poll rate
            Delay::new(SENSOR_TIMEOUT_POLL_RATE).await;

            // Read through every sensor
            for uuid in SensorsWorker::read_sensors(&sensors)
                .iter()
                // Find sensors that have expired
                .filter(|(_, sensor)| {
                    if let Ok(duration) = sensor.last_seen.elapsed() {
                        duration > SENSOR_TIMEOUT
                    } else {
                        true
                    }
                })
                .map(|(uuid, _)| uuid)
            {
                // Create disconnect requests into the network channel
                network_tx
                    .send(NetworkChannel::Disconnect { uuid: *uuid })
                    .unwrap();
            }
        }
    }
}
