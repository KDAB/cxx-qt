// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This modules contains utilities for specifying interfaces with cxx-qt-build.

use std::collections::HashSet;

use std::path::{Path, PathBuf};

use crate::{Dependency, Manifest};

/// When generating a library with cxx-qt-build, the library may need to export certain flags or headers.
/// These are all specified by this Interface struct, which should be passed to the [crate::CxxQtBuilder::library] function.
pub struct Interface {
    // The name of the links keys, whose CXX-Qt dependencies to reexport
    pub(crate) reexport_links: HashSet<String>,
    pub(crate) exported_include_prefixes: Vec<String>,
    pub(crate) exported_include_directories: Vec<(PathBuf, String)>,
    // TODO: In future, we want to also set up the include paths so that you can include anything
    // from the crates source directory.
    // Once this is done, this flag should indicate whether or not to export our own crates source
    // directory to downstream dependencies?
    // export_crate_directory: bool,
}

impl Default for Interface {
    fn default() -> Self {
        Self {
            reexport_links: HashSet::new(),
            exported_include_prefixes: vec![super::crate_name()],
            exported_include_directories: Vec::new(),
        }
    }
}

impl Interface {
    /// Export all headers with the given prefix to downstream dependencies
    ///
    /// Note: This will overwrite any previously specified header_prefixes, including the default
    /// header_prefix of this crate.
    ///
    /// This function will panic if any of the given prefixes are already exported through the
    /// [Self::export_include_directory] function.
    pub fn export_include_prefixes<'a>(
        mut self,
        prefixes: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        let prefixes = prefixes.into_iter().map(|item| item.to_string()).collect();

        let mut exported_prefixes = self
            .exported_include_directories
            .iter()
            .map(|(_path, prefix)| prefix);
        for prefix in &prefixes {
            if let Some(duplicate) =
                exported_prefixes.find(|exported_prefix| exported_prefix.starts_with(prefix))
            {
                panic!("Duplicate export_prefix! Cannot export `{prefix}`, as `{duplicate}` is already exported as an export_include_directory!");
            }
        }

        self.exported_include_prefixes = prefixes;
        self
    }

    /// Add a directory that will be added as an include directory under the given prefix.
    ///
    /// The prefix will automatically be exported (see also: [Self::export_include_prefixes])
    ///
    /// This function will panic if the given prefix is already exported.
    pub fn export_include_directory(mut self, directory: impl AsRef<Path>, prefix: &str) -> Self {
        let mut exported_prefixes = self.exported_include_prefixes.iter().chain(
            self.exported_include_directories
                .iter()
                .map(|(_path, prefix)| prefix),
        );
        if let Some(duplicate) =
            exported_prefixes.find(|exported_prefix| exported_prefix.starts_with(prefix))
        {
            panic!("Duplicate export_prefix! Cannot export `{prefix}`, as `{duplicate}` is already exported!");
        }

        self.exported_include_directories
            .push((directory.as_ref().into(), prefix.to_owned()));
        self
    }

    /// Reexport the dependency with the given link name.
    /// This will make the dependency available to downstream dependencies.
    ///
    /// Specifically it will reexport all include_prefixes of the given dependency
    /// as well as any definitions made by that dependency.
    ///
    /// Note that the link name may differ from the crate name.
    /// Check your dependencies manifest file for the correct link name.
    pub fn reexport_dependency(mut self, link_name: &str) -> Self {
        self.reexport_links.insert(link_name.to_owned());
        self
    }

    pub(crate) fn export(self, mut manifest: Manifest, dependencies: &[Dependency]) {
        // We automatically reexport all qt_modules and downstream dependencies
        // as they will always need to be enabled in the final binary.
        // However, we only reexport the headers of libraries that
        // are marked as re-export.
        let dependencies = reexported_dependencies(&self, &dependencies);

        manifest.exported_include_prefixes = all_include_prefixes(&self, &dependencies);

        let manifest_path = crate::dir::crate_target().join("manifest.json");
        let manifest_json =
            serde_json::to_string_pretty(&manifest).expect("Failed to convert Manifest to JSON!");
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
            interface
                .exported_include_directories
                .iter()
                .map(|(_path, prefix)| prefix.clone()),
        )
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
