// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This Rust module contains structs for registering QML modules.

use std::path::{Path, PathBuf};

/// Metadata for registering a QML module with [crate::CxxQtBuilder::qml_module]
pub struct QmlModule<'a, A, B>
where
    A: AsRef<Path>,
    // Use a separate generic to allow using different types that impl AsRef<Path>
    B: AsRef<Path>,
{
    /// The URI of the QML module
    pub uri: &'a str,
    /// The major version of the QML module
    pub version_major: usize,
    /// The minor version of the QML module
    pub version_minor: usize,
    /// The `.rs` files containing a `#[cxx_qt::bridge]` module with at least one QObject type annotated with `#[qml_element]`
    pub rust_files: &'a [A],
    /// `.qml` files included in the module
    pub qml_files: &'a [B],
    /// Other QRC resources (such as images) included in the module
    //
    // Reuse the `A` generic from rust_files to allow the compiler to infer the
    // type when constructing the struct with Default::default. Using a separate
    // generic for this field would be more flexible, but it would require users
    // to explicitly specify the type even for an empty slice (like `&[] as &[&str; 0]`)
    // and an empty slice is likely desired in most cases; most users probably don't
    // care about this field.
    pub qrc_files: &'a [A],
}

impl<A, B> Default for QmlModule<'_, A, B>
where
    A: AsRef<Path>,
    B: AsRef<Path>,
{
    fn default() -> Self {
        QmlModule {
            uri: "com.example.cxx_qt_module",
            version_major: 1,
            version_minor: 0,
            rust_files: &[],
            qml_files: &[],
            qrc_files: &[],
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
    pub rust_files: Vec<PathBuf>,
    pub qml_files: Vec<PathBuf>,
    pub qrc_files: Vec<PathBuf>,
}

fn collect_pathbuf_vec(asref: &[impl AsRef<Path>]) -> Vec<PathBuf> {
    asref.iter().map(|p| p.as_ref().to_path_buf()).collect()
}

impl<A: AsRef<Path>, B: AsRef<Path>> From<QmlModule<'_, A, B>> for OwningQmlModule {
    fn from(other: QmlModule<'_, A, B>) -> Self {
        OwningQmlModule {
            uri: other.uri.to_string(),
            version_major: other.version_major,
            version_minor: other.version_minor,
            rust_files: collect_pathbuf_vec(other.rust_files),
            qml_files: collect_pathbuf_vec(other.qml_files),
            qrc_files: collect_pathbuf_vec(other.qrc_files),
        }
    }
}
