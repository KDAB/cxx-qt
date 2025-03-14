// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "qmake")]
pub(crate) mod qmake;

use semver::Version;
use std::path::{Path, PathBuf};

use crate::{Initializer, MocArguments, MocProducts, QmlModuleRegistrationFiles};

/// A Qt Installation that can be used by cxx-qt-build to run Qt related tasks
///
/// Note that it is the responsbility of the QtInstallation implementation
/// to print any cargo::rerun-if-changed lines
pub trait QtInstallation {
    /// Return the include paths for Qt, including Qt module subdirectories.
    ///
    /// This is intended to be passed to whichever tool you are using to invoke the C++ compiler.
    fn include_paths(&self, qt_modules: Vec<String>) -> Vec<PathBuf>;
    /// Configure the given cc::Build and cargo to link to the given Qt modules
    fn link_modules(&self, builder: &mut cc::Build, qt_modules: Vec<String>);
    /// Run moc on a C++ header file and save the output into [cargo's OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html)
    /// The return value contains the path to the generated C++ file
    fn moc(&self, input_file: &Path, arguments: MocArguments) -> MocProducts;
    /// TODO
    fn qml_cache_gen(&self) -> PathBuf;
    /// TODO
    fn qml_type_registrar(
        &self,
        qml_types: &Path,
        version_major: u64,
        version_minor: u64,
        uri: &str,
    ) -> PathBuf;
    /// TODO
    /// TODO: instead just return the object file?
    fn qrc(&self, input_file: &Path) -> Initializer;
    /// Version of the detected Qt installation
    fn version(&self) -> Version;
}
