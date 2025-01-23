// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This modules contains utilities for specifying dependencies with cxx-qt-build.

use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// When generating a library with cxx-qt-build, the library may need to export certain flags or headers.
/// These are all specified by this Interface struct, which should be passed to the [crate::CxxQtBuilder::library] function.
pub struct Interface {
    pub(crate) initializers: Vec<PathBuf>,
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
            initializers: Vec::new(),
            reexport_links: HashSet::new(),
            exported_include_prefixes: vec![super::crate_name()],
            exported_include_directories: Vec::new(),
        }
    }
}

impl Interface {
    /// Add a C++ file path that will be exported as an initializer to downstream dependencies.
    ///
    /// Initializer files will be built into object files, instead of linked into the static
    /// library.
    /// This way, the static variables and their constructors in this code will not be optimized
    /// out by the linker.
    pub fn initializer(mut self, path: impl AsRef<Path>) -> Self {
        let path = PathBuf::from(path.as_ref());
        let path = path
            .canonicalize()
            .expect("Failed to canonicalize path to initializer! Does the path exist?");
        self.initializers.push(path);
        self
    }

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
}

#[derive(Clone, Serialize, Deserialize)]
/// This struct is used by cxx-qt-build internally to propagate data through to downstream
/// dependencies
pub(crate) struct Manifest {
    pub(crate) name: String,
    pub(crate) link_name: String,
    pub(crate) qt_modules: Vec<String>,
    pub(crate) initializers: Vec<PathBuf>,
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

pub(crate) fn initializer_paths(
    interface: Option<&Interface>,
    dependencies: &[Dependency],
) -> HashSet<PathBuf> {
    dependencies
        .iter()
        .flat_map(|dep| dep.manifest.initializers.iter().cloned())
        .chain(
            interface
                .iter()
                .flat_map(|interface| interface.initializers.iter().cloned()),
        )
        .collect()
}

pub(crate) fn all_include_prefixes(
    interface: &Interface,
    dependencies: &[Dependency],
) -> Vec<String> {
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

pub(crate) fn reexported_dependencies(
    interface: &Interface,
    dependencies: &[Dependency],
) -> Vec<Dependency> {
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
