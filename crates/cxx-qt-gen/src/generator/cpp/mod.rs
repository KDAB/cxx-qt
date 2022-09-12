// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod fragment;
pub mod qobject;

/// Representation of the generated C++ code for a group of QObjects
pub struct GeneratedCppBlocks {
    /// Stem of the CXX header to include
    pub cxx_stem: String,
    /// Ident of the common namespace of the QObjects
    pub namespace: String,
    /// Generated QObject blocks
    pub qobjects: Vec<qobject::GeneratedCppQObjectBlocks>,
}
