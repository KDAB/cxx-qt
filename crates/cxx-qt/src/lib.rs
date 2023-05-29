// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(missing_docs)]

//! This crate and its associated crates provide a framework for generating QObjects from Rust.
//!
//! See the [book](https://kdab.github.io/cxx-qt/book/) for more information.

pub use cxx_qt_macro::bridge;
pub use cxx_qt_macro::inherit;
pub use cxx_qt_macro::qobject;
pub use cxx_qt_macro::qsignals;
