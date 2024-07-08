// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This modules contains information about the paths where artifacts are placed by cxx-qt-build.

use crate::{crate_name, module_name_from_uri};
use std::io::Result;
use std::{
    env,
    path::{Path, PathBuf},
};

// Clean a directory by removing it and recreating it.
pub(crate) fn clean(path: impl AsRef<Path>) -> Result<()> {
    let result = std::fs::remove_dir_all(&path);
    if let Err(err) = result {
        // If the path doesn't exist that's fine, otherwise we want to panic
        if err.kind() != std::io::ErrorKind::NotFound {
            return Err(err);
        }
    }

    std::fs::create_dir_all(path)?;
    Ok(())
}

/// The target directory, namespaced by crate
pub(crate) fn crate_target() -> PathBuf {
    target().join("crates").join(crate_name())
}

/// The target directory, namespaced by plugin
pub(crate) fn module_target(module_uri: &str) -> PathBuf {
    target()
        .join("qml_modules")
        .join(module_name_from_uri(module_uri))
}

/// The target directory or another directory where we can write files that will be shared
/// between crates.
pub(crate) fn target() -> PathBuf {
    if let Some(export) = export() {
        return export;
    }

    // The CARGO_TARGET_DIR is only set by users that want to configure cargo.
    // So it's unlikely that it is indeed set.
    // However, if it is, it's the easiest way to get the target dir.
    let cargo_target_dir = env::var("CARGO_TARGET_DIR").ok().map(PathBuf::from);
    if let Some(cargo_target_dir) = cargo_target_dir {
        if cargo_target_dir.exists() && cargo_target_dir.is_absolute() {
            return cargo_target_dir.join("cxxqtbridge");
        }
    }

    scratch::path("cxxqtbridge")
}

/// The export directory, if one was specified through the environment.
/// Note that this is not namspaced by crate.
pub(crate) fn export() -> Option<PathBuf> {
    env::var("CXXQT_EXPORT_DIR").ok().map(PathBuf::from)
}

/// The include directory is namespaced by crate name when exporting for a C++ build system,
/// but for using cargo build without a C++ build system, OUT_DIR is already namespaced by crate name.
pub(crate) fn header_root() -> PathBuf {
    crate_target().join("include")
}

/// The OUT_DIR, converted into a PathBuf
pub(crate) fn out() -> PathBuf {
    env::var("OUT_DIR").unwrap().into()
}

pub(crate) fn is_exporting() -> bool {
    export().is_some()
}
