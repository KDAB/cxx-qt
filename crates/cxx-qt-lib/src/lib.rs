// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(not(any(cxxqt_qt_version_major = "5", cxxqt_qt_version_major = "6")))]
compile_error!("cxxqt_qt_version_major must be either \"5\" or \"6\"");

mod core;

pub use crate::core::*;

#[cfg(feature = "qt_gui")]
mod gui;
#[cfg(feature = "qt_gui")]
pub use crate::gui::*;

#[cfg(feature = "qt_qml")]
mod qml;
#[cfg(feature = "qt_qml")]
pub use crate::qml::*;

#[cfg(feature = "qt_quickcontrols")]
mod quickcontrols;
#[cfg(feature = "qt_quickcontrols")]
pub use crate::quickcontrols::*;
