// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(not(any(qt_version_major = "5", qt_version_major = "6")))]
compile_error!("qt_version_major must be either \"5\" or \"6\"");

mod core;
use std::cmp::Ordering;

pub use crate::core::*;

fn get_ordering(ord: i8) -> Option<Ordering> {
    match ord {
        -1 => Some(Ordering::Less),
        0 => Some(Ordering::Equal),
        1 => Some(Ordering::Greater),
        _ => None,
    }
}

#[cfg(feature = "qt_gui")]
mod gui;
#[cfg(feature = "qt_gui")]
pub use crate::gui::*;

#[cfg(feature = "qt_qml")]
mod qml;
#[cfg(feature = "qt_qml")]
pub use crate::qml::*;
