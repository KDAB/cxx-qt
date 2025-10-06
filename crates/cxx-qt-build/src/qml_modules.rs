// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This Rust module contains structs for registering QML modules.

use std::path::{Path, PathBuf};

/// Metadata for registering a QML module with [crate::CxxQtBuilder::qml_module]
pub struct QmlModule<'a, A>
where
    // Use a separate generic to allow using different types that impl AsRef<Path>
    A: AsRef<Path>,
{
    /// The URI of the QML module
    pub uri: &'a str,
    /// The major version of the QML module
    pub version_major: usize,
    /// The minor version of the QML module
    pub version_minor: usize,
    /// `.qml` files included in the module
    pub qml_files: &'a [A],
}

impl<A> Default for QmlModule<'_, A>
where
    A: AsRef<Path>,
{
    fn default() -> Self {
        QmlModule {
            uri: "com.example.cxx_qt_module",
            version_major: 1,
            version_minor: 0,
            qml_files: &[],
        }
    }
}

/// Same as [QmlModule], but this struct owns the data instead of referencing it.
/// This avoids needing to specify generics to instantiate a [crate::CxxQtBuilder], which
/// contains a `Vec<OwningQmlModule>` member.
pub(crate) struct OwningQmlModule {
    pub uri: String,
    pub version_major: usize,
    pub version_minor: usize,
    pub qml_files: Vec<PathBuf>,
}

fn collect_pathbuf_vec(asref: &[impl AsRef<Path>]) -> Vec<PathBuf> {
    asref.iter().map(|p| p.as_ref().to_path_buf()).collect()
}

impl<A: AsRef<Path>> From<QmlModule<'_, A>> for OwningQmlModule {
    fn from(other: QmlModule<'_, A>) -> Self {
        OwningQmlModule {
            uri: other.uri.to_owned(),
            version_major: other.version_major,
            version_minor: other.version_minor,
            qml_files: collect_pathbuf_vec(other.qml_files),
        }
    }
}
