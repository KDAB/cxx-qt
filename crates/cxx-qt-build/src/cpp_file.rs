// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::{Path, PathBuf};

use crate::MocArguments;

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
