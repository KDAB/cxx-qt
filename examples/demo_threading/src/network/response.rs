// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Status {
    Ok,
    ErrorFailedToRead,
    ErrorFailedToParseAsUtf8,
    ErrorFailedToParseJSONRequest,
    ErrorInvalidReadSize,
    ErrorInvalidPower,
    ErrorServerQueueFull,
    ErrorServerDisconnected,
}

#[derive(Serialize)]
pub(crate) struct Response {
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
