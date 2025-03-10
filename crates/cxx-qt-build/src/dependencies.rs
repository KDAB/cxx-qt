// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This modules contains utilities for specifying dependencies with cxx-qt-build.

use serde::{Deserialize, Serialize};

use std::path::PathBuf;

#[derive(Clone, Default, Serialize, Deserialize)]
/// This struct is used by cxx-qt-build internally to propagate data through to downstream
/// dependencies
pub(crate) struct Manifest {
    pub(crate) name: String,
    pub(crate) link_name: String,
    pub(crate) qt_modules: Vec<String>,
    pub(crate) initializers: Vec<qt_build_utils::Initializer>,
    pub(crate) exported_include_prefixes: Vec<String>,
}

#[derive(Clone)]
/// A dependency that has been set up with [crate::CxxQtBuilder::library] and is available to
/// the crate that is currently being built.
pub(crate) struct Dependency {
    /// The path of the dependencies export directory
    pub(crate) path: PathBuf,
    /// The deserialized manifest of the dependency
    pub(crate) manifest: Manifest,
}

impl Dependency {
    /// This function will search the environment for all dependencies that have been set up with
    /// CxxQtBuilder::library.
    /// They export their manifest paths as metadata, which will be exposed to us as an environment
    /// variable.
    /// We extract those paths here, parse the manifest and make sure to set it up correctly as a
    /// dependency.
    ///
    /// See also the internals "build system" section of our book.
    pub(crate) fn find_all() -> Vec<Dependency> {
        std::env::vars_os()
            .map(|(var, value)| (var.to_string_lossy().to_string(), value))
            .filter(|(var, _)| var.starts_with("DEP_") && var.ends_with("_CXX_QT_MANIFEST_PATH"))
            .map(|(_, manifest_path)| {
                let manifest_path = PathBuf::from(manifest_path);
                let manifest: Manifest = serde_json::from_str(
                    &std::fs::read_to_string(&manifest_path)
                        .expect("Could not read dependency manifest file!"),
                )
                .expect("Could not deserialize dependency manifest file!");

                println!(
                    "cxx-qt-build: Discovered dependency `{}` at: {}",
                    manifest.name,
                    manifest_path.to_string_lossy()
                );
                Dependency {
                    path: manifest_path.parent().unwrap().to_owned(),
                    manifest,
                }
            })
            .collect()
    }
}

pub(crate) fn initializers(dependencies: &[Dependency]) -> Vec<qt_build_utils::Initializer> {
    dependencies
        .iter()
        .flat_map(|dep| dep.manifest.initializers.iter().cloned())
        .collect()
}
