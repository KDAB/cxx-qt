// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "qmake")]
pub(crate) mod qmake;

#[cfg(feature = "qt_minimal")]
pub(crate) mod qt_minimal;

mod shared;

use semver::Version;
use std::path::PathBuf;

use crate::QtTool;

/// A Qt Installation that can be used by cxx-qt-build to run Qt related tasks
///
/// Note that it is the responsbility of the QtInstallation implementation
/// to print any cargo::rerun-if-changed lines
pub trait QtInstallation {
    /// Return the framework paths for Qt
    ///
    /// This is intended to be passed to whichever tool you are using to invoke the C++ compiler.
    fn framework_paths(&self, qt_modules: &[String]) -> Vec<PathBuf>;
    /// Return the include paths for Qt, including Qt module subdirectories.
    ///
    /// This is intended to be passed to whichever tool you are using to invoke the C++ compiler.
    fn include_paths(&self, qt_modules: &[String]) -> Vec<PathBuf>;
    /// Configure the given cc::Build and cargo to link to the given Qt modules
    ///
    // TODO: should we hand in a cc::Build or should we instead return a struct
    // with details of the rustc-link-lib / search paths ? and then have the
    // calling function apply those and any flags to the cc::Build?
    // eg return the following?
    //
    // pub struct LinkArgs {
    //     builder_flag_if_supported: Vec<String>,
    //     builder_object: Vec<String>,
    //     rustc_link_arg: Vec<String>,
    //     rustc_link_lib: Vec<String>,
    //     rustc_link_search: Vec<String>,
    // }
    fn link_modules(&self, builder: &mut cc::Build, qt_modules: &[String]);
    /// Find the path to a given Qt tool for the Qt installation
    fn try_find_tool(&self, tool: QtTool) -> anyhow::Result<PathBuf>;
    /// Version of the detected Qt installation
    fn version(&self) -> Version;
}
