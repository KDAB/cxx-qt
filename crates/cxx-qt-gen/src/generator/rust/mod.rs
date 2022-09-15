// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::ItemMod;

pub mod qobject;

/// Representation of the generated Rust code for a QObject
pub struct GeneratedRustBlocks {
    /// Module for the CXX bridge with passthrough items
    pub cxx_mod: ItemMod,
    /// Ident of the namespace of the QObject
    pub namespace: String,
    /// Generated QObject blocks
    pub qobjects: Vec<qobject::GeneratedRustQObjectBlocks>,
}
