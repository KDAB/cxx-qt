// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Initialiser definitions for the builder to inject
pub struct CxxQtBuilderInit {
    pub(crate) method_name: String,
    pub(crate) source: String,
}

impl CxxQtBuilderInit {
    /// Create an initialisers with the given method name and source
    pub fn new(method_name: &str, source: &str) -> Self {
        Self {
            source: source.to_owned(),
            method_name: method_name.to_owned(),
        }
    }
}
