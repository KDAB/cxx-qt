// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod qobject;

use syn::{Item, ItemMod};

/// Representation of the generated Rust code for a QObject
pub struct GeneratedRustBlocks {
    /// Module for the CXX bridge with passthrough items
    pub cxx_mod: ItemMod,
    /// Any global passthrough items for the implementation
    pub cxx_qt_mod_contents: Vec<Item>,
    /// Ident of the namespace of the QObject
    pub namespace: String,
    /// Generated QObject blocks
    pub qobjects: Vec<qobject::GeneratedRustQObjectBlocks>,
}
