// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::{Path, PathBuf};

use crate::MocArguments;

/// This struct represents a C++ file to compile, together with appropriate options.
///
/// CppFile can be created using the `From<impl AsRef<Path>>` trait.
/// ```
/// # use cxx_qt_build::{CppFile, MocArguments};
/// CppFile::from("path/to/header.h")
///     .moc_arguments(MocArguments::default());
/// ```
pub struct CppFile {
    pub(crate) path: PathBuf,
    pub(crate) moc_arguments: MocArguments,
    pub(crate) enable_moc: bool,
    pub(crate) compile: bool,
}

impl<T> From<T> for CppFile
where
    T: AsRef<Path>,
{
    fn from(path: T) -> Self {
        let path = path.as_ref().to_owned();
        let is_header = path
            .extension()
            .map(|ext| Self::HEADER_EXTENSIONS.contains(&&*ext.to_string_lossy().to_lowercase()))
            .unwrap_or_default();
        let enable_moc = is_header;
        let compile = !is_header;
        Self {
            path,
            moc_arguments: MocArguments::default(),
            enable_moc,
            compile,
        }
    }
}

impl CppFile {
    /// Which extensions are treated as header files.
    pub const HEADER_EXTENSIONS: &'static [&'static str] = &["h", "hpp", "hh", "hxx", "h++"];

    /// Set the moc arguments for this header.
    /// This will also enable running moc over this file.
    ///
    /// By default this is `MocArguments::default()`.
    pub fn moc_arguments(self, moc_arguments: MocArguments) -> Self {
        Self {
            moc_arguments,
            enable_moc: true,
            ..self
        }
    }

    /// Whether to compile this file.
    ///
    /// By default, header files are not compiled, all other files are compiled (i.e. ending
    /// in one of the extensions listed in [Self::HEADER_EXTENSIONS])
    pub fn compile(self, compile: bool) -> Self {
        Self { compile, ..self }
    }

    /// Whether to run moc over this file.
    ///
    /// By default, moc is enabled for header files (i.e. ending in one of the extensions
    /// listed in [Self::HEADER_EXTENSIONS]).
    pub fn moc(self, enable_moc: bool) -> Self {
        Self { enable_moc, ..self }
    }
}
