// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::time::Duration;

/// The size of the network thread to update thread queue
pub const CHANNEL_NETWORK_COUNT: usize = 1_024;
/// The size of the update thread to Qt queue
pub const CHANNEL_QT_COUNT: usize = 250;
/// After how many milliseconds should a sensor be disconnected and considered missing
pub const SENSOR_TIMEOUT: Duration = Duration::from_millis(10_000);
/// How often should the timeout thread poll sensors
pub const SENSOR_TIMEOUT_POLL_RATE: Duration = Duration::from_millis(256);
/// How often should the update thread poll sensors
pub const SENSOR_UPDATE_POLL_RATE: Duration = Duration::from_millis(128);
/// The maximum number of sensors we will manage
pub const SENSOR_MAXIMUM_COUNT: usize = 1000;
/// The maximum power a sensor can report
pub const SENSOR_MAXIMUM_POWER: f64 = 1000.0;
