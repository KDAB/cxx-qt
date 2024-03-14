// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::MocArguments;

/// Options for external crates to use
#[derive(Default)]
pub struct CxxQtBuildersOpts {
    /// Any extra definitions
    pub(crate) defines: HashSet<String>,
    /// Contents, directory, file name
    pub(crate) headers: Vec<(String, String, String)>,
    /// Qt modules that are required
    pub(crate) qt_modules: HashSet<String>,
}

impl CxxQtBuildersOpts {
    /// Any additional defines that are required from this opt
    pub fn define(mut self, define: &str) -> Self {
        self.defines.insert(define.to_owned());
        self
    }

    /// Any additional headers that are required from this opt
    ///
    /// These are placed in the given sub directory with the given file name
    pub fn header(mut self, contents: &str, directory: &str, file_name: &str) -> Self {
        self.headers.push((
            contents.to_owned(),
            directory.to_owned(),
            file_name.to_owned(),
        ));
        self
    }

    /// Link additional [Qt modules](https://doc.qt.io/qt-6/qtmodules.html) for this opt.
    /// Specify their names without the `Qt` prefix, for example `"Widgets"`.
    pub fn qt_module(mut self, module: &str) -> Self {
        self.qt_modules.insert(module.to_owned());
        self
    }
}

/// Options for qobject_headers
///
/// QObjectHeaderOpts can be created using the `From<impl AsRef<Path>>` trait.
/// ```
/// # use cxx_qt_build::{QObjectHeaderOpts, MocArguments};
/// QObjectHeaderOpts::from("path/to/header.h")
///     .moc_arguments(MocArguments::default());
/// ```
pub struct QObjectHeaderOpts {
    pub(crate) path: PathBuf,
    pub(crate) moc_arguments: MocArguments,
}

impl<T> From<T> for QObjectHeaderOpts
where
    T: AsRef<Path>,
{
    fn from(path: T) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            moc_arguments: MocArguments::default(),
        }
    }
}

impl QObjectHeaderOpts {
    /// Set the moc arguments for this header
    ///
    /// By default this is `MocArguments::default()`
    pub fn moc_arguments(self, moc_arguments: MocArguments) -> Self {
        Self {
            moc_arguments,
            ..self
        }
    }
}
