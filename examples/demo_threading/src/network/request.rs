// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::Deserialize;
use uuid::Uuid;

/// Definition of a command
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RequestCommand {
    /// The sensor should be disconnected
    Disconnect,
    /// The sensor has a new power value
    ///
    /// Note that in this example a sensor "connects" by using the power
    /// command with the new uuid. Real world applications would likely
    /// have authentication when connecting sensors.
    Power { value: f64 },
}

/// Definition of a network request
#[derive(Deserialize)]
pub(crate) struct Request {
    /// The command that has been requested
    pub command: RequestCommand,
    /// The uuid of the sensor
    pub uuid: Uuid,
}
