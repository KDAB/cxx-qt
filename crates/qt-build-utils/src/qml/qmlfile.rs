// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::{Path, PathBuf};

/// A QML file to be included in a QML module (e.g. by ways of [super::QmlDirBuilder]).
#[derive(Clone, Debug, Hash)]
pub struct QmlFile {
    path: PathBuf,
    singleton: bool,
    version: Option<(usize, usize)>,
}

impl<T: AsRef<Path>> From<T> for QmlFile {
    /// Create a new QmlFile from the given path.
    ///
    /// By default, no version is set for the QML file, and the file is not a singleton.
    fn from(path: T) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            singleton: false,
            version: None,
        }
    }
}

impl QmlFile {
    /// Set whether the component inside this QML file is a singleton.
    pub fn singleton(mut self, singleton: bool) -> Self {
        self.singleton = singleton;
        self
    }

    /// Returns whether the component inside this QML file is a singleton.
    pub fn is_singleton(&self) -> bool {
        self.singleton
    }

    /// Returns the path to the QML file.
    pub fn get_path(&self) -> &Path {
        &self.path
    }

    /// Assign a version to the QML file.
    pub fn version(mut self, major: usize, minor: usize) -> Self {
        self.version = Some((major, minor));
        self
    }

    /// Returns the version assigned to the QML file, if any.
    pub fn get_version(&self) -> Option<(usize, usize)> {
        self.version
    }
}
