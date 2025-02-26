// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod attribute;
pub mod cfg;
pub mod expr;
pub mod foreignmod;
pub mod lifetimes;
pub mod path;
mod qtfile;
mod qtitem;
pub mod types;

pub use qtfile::{parse_qt_file, CxxQtFile};
pub use qtitem::CxxQtItem;
