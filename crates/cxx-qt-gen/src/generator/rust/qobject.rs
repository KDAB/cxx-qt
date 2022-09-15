// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{Ident, Item};

pub struct GeneratedRustQObjectBlocks {
    /// Module for the CXX bridge
    pub cxx_mod_contents: Vec<Item>,
    /// Items for the CXX-Qt module
    pub cxx_qt_mod_contents: Vec<Item>,
    /// Ident of the Rust name for the C++ object
    pub cpp_struct_ident: Ident,
    /// Ident of the CxxQtThread object
    pub cxx_qt_thread_ident: Ident,
    /// Ident of the namespace for CXX-Qt internals of the QObject
    pub namespace_internals: String,
    /// Ident of the Rust name for the Rust object
    pub rust_struct_ident: Ident,
}
