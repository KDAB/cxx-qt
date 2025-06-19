// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This modules contains utilities for specifying interfaces with cxx-qt-build.

use core::panic;
use std::collections::HashSet;

use crate::{dir, Dependency, Manifest};

/// When generating a library with cxx-qt-build, the library may need to export certain flags or headers.
/// These are all specified by this Interface struct.
pub struct Interface {
    // The name of the links keys, whose CXX-Qt dependencies to reexport
    pub(crate) reexport_links: HashSet<String>,
    pub(crate) exported_include_prefixes: Vec<String>,
    pub(crate) manifest: Manifest,
    pub(crate) dependencies: Vec<Dependency>,
}

impl Default for Interface {
    fn default() -> Self {
        Self {
            reexport_links: HashSet::new(),
            // TODO: This doesn't currently match the include_prefix that is specified by e.g.
            // cxx-qt-lib build script.
            // In this case this is a happy accident, as we don't want to actually export the
            // `include_prefix` in cxx-qt-lib (which is "private/").
            // But we do need to unify this.
            exported_include_prefixes: vec![super::crate_name()],
            manifest: Manifest::default(),
            dependencies: Vec::new(),
        }
    }
}

impl Interface {
    /// Export all headers with the given prefix to downstream dependencies
    ///
    /// Note: This will overwrite any previously specified header_prefixes, including the default
    /// header_prefix of this crate.
    pub fn export_include_prefixes<'a>(
        mut self,
        prefixes: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        let prefixes = prefixes.into_iter().map(|item| item.to_string()).collect();

        self.exported_include_prefixes = prefixes;
        self
    }

    /// Reexport the dependency with the given link name.
    /// This will make the dependency available to downstream dependencies.
    ///
    /// Specifically it will reexport all include_prefixes of the given dependency.
    ///
    /// Note that the link name may differ from the crate name.
    /// Check your dependencies Cargo.toml for the correct link name.
    pub fn reexport_dependency(mut self, link_name: &str) -> Self {
        self.reexport_links.insert(link_name.to_owned());
        self
    }

    /// Export the Interface for this crate so that it can be used by downstream
    /// crates.
    ///
    /// # Panics
    ///
    /// Currently it is only possible to export a single Interface per crate.
    /// If you try to call this method multiple times, it will panic.
    pub fn export(mut self) {
        use std::sync::atomic::{AtomicBool, Ordering};
        static HAS_EXPORTED: AtomicBool = AtomicBool::new(false);
        if HAS_EXPORTED
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            panic!("cxx-qt-build can only export a single Interface per crate.\nConsider splitting your project into multiple crates.");
        }

        // Ensure that a link name has been set
        if self.manifest.link_name.is_empty() {
            panic!("The links key must be set when exporting with CXX-Qt-build");
        }

        // We automatically reexport all qt_modules and downstream dependencies
        // as they will always need to be enabled in the final binary.
        // However, we only reexport the headers of libraries that
        // are marked as re-export.
        let dependencies = reexported_dependencies(&self, &self.dependencies);

        self.manifest.exported_include_prefixes = all_include_prefixes(&self, &dependencies);

        let manifest_path = dir::crate_target().join("manifest.json");
        let manifest_json = serde_json::to_string_pretty(&self.manifest)
            .expect("Failed to convert Manifest to JSON!");
        std::fs::write(&manifest_path, manifest_json).expect("Failed to write manifest.json!");
        println!(
            "cargo::metadata=CXX_QT_MANIFEST_PATH={}",
            manifest_path.to_string_lossy()
        );
    }
}

fn all_include_prefixes(interface: &Interface, dependencies: &[Dependency]) -> Vec<String> {
    interface
        .exported_include_prefixes
        .iter()
        .cloned()
        .chain(
            dependencies
                .iter()
                .flat_map(|dep| &dep.manifest.exported_include_prefixes)
                .cloned(),
        )
        .collect()
}

fn reexported_dependencies(interface: &Interface, dependencies: &[Dependency]) -> Vec<Dependency> {
    let mut exported_dependencies = Vec::new();
    for link_name in &interface.reexport_links {
        if let Some(dependency) = dependencies
            .iter()
            .find(|dep| &dep.manifest.link_name == link_name)
        {
            exported_dependencies.push(dependency.clone());
        } else {
            panic!("Could not find dependency with link name `{link_name}` to reexport!");
        }
    }
    exported_dependencies
}
