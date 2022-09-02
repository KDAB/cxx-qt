// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod fragment;

use fragment::CppFragmentPair;

/// Representation of the generated C++ code for a QObject
pub struct GeneratedCppBlocks {
    /// Stem of the CXX header to include
    pub cxx_stem: String,
    /// Ident of the C++ QObject
    pub ident: String,
    /// Ident of the Rust object
    pub rust_ident: String,
    /// Ident of the CxxQtThread object
    pub cxx_qt_thread_ident: String,
    /// Ident of the namespace of the QObject
    pub namespace: String,
    /// Ident of the namespace for CXX-Qt internals of the QObject
    pub namespace_internals: String,
    /// Base class of the QObject
    pub base_class: String,
    /// List of Qt Meta Object items (eg Q_PROPERTY)
    pub metaobjects: Vec<String>,
    /// List of public methods for the QObject
    pub methods: Vec<CppFragmentPair>,
    /// List of public Q_SLOTS for the QObject
    pub slots: Vec<CppFragmentPair>,
    /// List of public Q_SIGNALS for the QObject
    pub signals: Vec<String>,
}
