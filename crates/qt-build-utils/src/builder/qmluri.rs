// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// A builder for representing a QML uri
pub struct QmlUriBuilder {
    uri: Vec<String>,
}

impl QmlUriBuilder {
    /// Construct a [QmlUriBuilder] from a given string
    pub fn new(uri: Vec<String>) -> Self {
        Self { uri }
    }

    /// Retrieve the QML uri in directory form
    pub fn as_dirs(&self) -> String {
        self.uri.join("/")
    }

    /// Retrieve the QML uri in dot form
    pub fn as_dots(&self) -> String {
        self.uri.join(".")
    }

    /// Retrieve the QML uri in underscore form
    pub fn as_underscores(&self) -> String {
        self.uri.join("_")
    }
}
