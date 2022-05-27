// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::Serialize;

/// Definition of the network response status
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Status {
    /// The sensor request was accepted
    Ok,
    /// The errors that can occur
    ErrorFailedToRead,
    ErrorFailedToParseAsUtf8,
    ErrorFailedToParseJSONRequest,
    ErrorInvalidReadSize,
    ErrorInvalidPower,
    ErrorServerQueueFull,
    ErrorServerDisconnected,
}

/// Definition of the network response
#[derive(Serialize)]
pub(crate) struct Response {
    /// The status of the response
    status: Status,
}

/// When a serde error occurs convert to the relevant status
impl From<serde_json::Error> for Status {
    fn from(_: serde_json::Error) -> Self {
        Status::ErrorFailedToParseJSONRequest
    }
}

/// When a UTF8 parse error occurs convert to the relevant status
impl From<std::str::Utf8Error> for Status {
    fn from(_: std::str::Utf8Error) -> Self {
        Status::ErrorFailedToParseAsUtf8
    }
}

/// Define converting from Status to Response
impl From<Status> for Response {
    fn from(status: Status) -> Self {
        Response { status }
    }
}
