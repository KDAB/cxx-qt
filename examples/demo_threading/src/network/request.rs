// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::Deserialize;
use uuid::Uuid;

// Network definition
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RequestCommand {
    Disconnect,
    Power { value: f64 },
}

#[derive(Deserialize)]
pub(crate) struct Request {
    pub command: RequestCommand,
    pub uuid: Uuid,
}
