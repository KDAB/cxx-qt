// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This modules contains utilities for specifying dependencies with cxx-qt-build.

use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// When generating a library with cxx-qt-build, the library may need to export certain flags or headers.
/// These are all specified by this Interface struct, which should be passed to the [CxxQtBuilder::library] function.
pub struct Interface {
    pub(crate) qt_modules: HashSet<String>,
    pub(crate) compile_definitions: Vec<(String, Option<String>)>,
    pub(crate) initializers: Vec<PathBuf>,
    // TODO: The name of the links keys, whose CXX-Qt dependencies to reexport
    // reexport_links: Vec<String>,
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
            qt_modules: HashSet::new(),
            compile_definitions: Vec::new(),
            initializers: Vec::new(),
            exported_include_prefixes: vec![super::crate_name()],
            exported_include_directories: Vec::new(),
        }
    }
}

impl Interface {
    /// Add a qt module to the CxxQtBuilder for this crate and all downstream dependencies.
    pub fn qt_module(mut self, module: &str) -> Self {
        self.qt_modules.insert(module.to_owned());
        self
    }

    /// Add a compile-time-definition for the C++ code built by this crate and all downstream
    /// dependencies
    pub fn define(mut self, variable: &str, value: Option<&str>) -> Self {
        self.compile_definitions
            .push((variable.to_owned(), value.map(str::to_owned)));
        self
    }

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
    /// [export_include_directory] function.
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
    /// The prefix will automatically be exported (see also: [export_header_prefix])
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
}

// cxx-qt-lib example
// Interface {
//     export_crate_headers: false,
//     exported_header_prefixes: Some(vec!["cxx-qt-lib/", "python3"]),
//     generated_path: "cxx-qt-lib-internals",
// }

// cxx-qt-lib-headers example
// Interface {
//     export_crate_headers: true,
//     exported_header_prefixes: Some(vec!["cxx-qt-lib-extras/..."]),
//     reexport_links: vec!["cxx-qt-lib"],
// }

#[derive(Serialize, Deserialize)]
// This struct is used by cxx-qt-build internally to propagate data through to downstream
// dependencies
pub(crate) struct Manifest {
    pub(crate) name: String,
    pub(crate) qt_modules: Vec<String>,
    pub(crate) defines: Vec<(String, Option<String>)>,
    pub(crate) initializers: Vec<PathBuf>,
    pub(crate) exported_include_prefixes: Vec<String>,
}

pub(crate) struct Dependency {
    pub(crate) path: PathBuf,
    pub(crate) manifest: Manifest,
}
