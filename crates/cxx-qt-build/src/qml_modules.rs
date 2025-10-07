// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This Rust module contains structs for registering QML modules.

use std::path::{Path, PathBuf};

/// This is a description of a QML module for building by the [crate::CxxQtBuilder].
///
/// It allows registering QML files that will be included in the QML module.
/// For further resources such as images, these can be added to the Qt resources
/// system via the appropriate CxxQtBuilder functions.
#[must_use = "The QML module only does anything if it is passed to CxxQtBuilder::qml_module"]
pub struct QmlModule {
    pub(crate) uri: String,
    pub(crate) version_major: usize,
    pub(crate) version_minor: usize,
    pub(crate) qml_files: Vec<PathBuf>,
}

impl QmlModule {
    /// Create a new [QmlModule] with the given URI.
    ///
    /// The default version is 1.0.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            version_major: 1,
            version_minor: 0,
            qml_files: Vec::new(),
        }
    }

    /// Add a version to the QML module.
    pub fn version(mut self, version_major: usize, version_minor: usize) -> Self {
        self.version_major = version_major;
        self.version_minor = version_minor;
        self
    }

    /// Add a single QML file to the module.
    ///
    /// The [crate::CxxQtBuilder] will register the file with the [Qt Resource System](https://doc.qt.io/qt-6/resources.html) in
    /// the [default QML import path](https://doc.qt.io/qt-6/qtqml-syntax-imports.html#qml-import-path) `qrc:/qt/qml/uri/of/module/`.
    ///
    /// When using Qt 6, the [crate::CxxQtBuilder] will [run qmlcachegen](https://doc.qt.io/qt-6/qtqml-qtquick-compiler-tech.html)
    /// to compile the specified `.qml` file ahead-of-time.
    ///
    /// Additional resources such as images can be added to the Qt resources for the QML module by specifying
    /// the `qrc_files` field.
    pub fn qml_file(self, file: impl AsRef<Path>) -> Self {
        self.qml_files([file])
    }

    /// Add multiple QML files to the module, see [Self::qml_file].
    pub fn qml_files(mut self, files: impl IntoIterator<Item = impl AsRef<Path>>) -> Self {
        self.qml_files
            .extend(files.into_iter().map(|p| p.as_ref().to_path_buf()));
        self
    }
}
