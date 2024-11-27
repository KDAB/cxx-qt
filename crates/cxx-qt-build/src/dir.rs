// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This modules contains information about the paths where artifacts are placed by cxx-qt-build.

use crate::{crate_name, module_name_from_uri};
use std::io::Result;
use std::{
    env, fs,
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

    out().join("cxx-qt-build").join("target")
}

/// The export directory, if one was specified through the environment.
/// Note that this is not namspaced by crate.
pub(crate) fn export() -> Option<PathBuf> {
    // Make sure to synchronize the naming of these variables with CMake!
    let export_flag = format!("CXX_QT_EXPORT_CRATE_{}", crate_name());
    // We only want to export this crate if it is the specific crate that CMake is looking for and
    // not any of that crates dependencies.
    // This should avoid issues where multiple configurations of the same crate are being built in
    // parallel by Cargo.
    // CMake should usually only have a single configuration in the same build directory (and therefore
    // export directory) so there should never be more than one configuration writing to that same
    // export directory.
    if env::var(export_flag).is_ok() {
        env::var("CXX_QT_EXPORT_DIR").ok().map(PathBuf::from)
    } else {
        None
    }
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

/// Two dependencies may be reexporting the same shared dependency, which will
/// result in conflicting symlinks.
/// Try detecting this by resolving the symlinks and checking whether this leads us
/// to the same paths. If so, it's the same include path for the same prefix, which
/// is fine.
#[cfg(unix)]
pub(crate) fn symlinks_conflict(
    symlink: impl AsRef<Path>,
    target: impl AsRef<Path>,
) -> Result<bool> {
    if !symlink.as_ref().exists() {
        return Ok(false);
    }
    let symlink = fs::canonicalize(symlink)?;
    let target = fs::canonicalize(target)?;
    Ok(symlink == target)
}

pub(crate) fn symlink_or_copy_directory(
    source: impl AsRef<Path>,
    dest: impl AsRef<Path>,
) -> Result<()> {
    #[cfg(unix)]
    let result = std::os::unix::fs::symlink(source, dest);

    #[cfg(not(unix))]
    let result = copy_dir_all(source.as_ref(), dest.as_ref());

    result
}

#[cfg(not(unix))]
fn copy_dir_all(source: &Path, dest: &Path) -> Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target_path = entry.path();
        let link_path = dest.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(&target_path, &link_path)?;
        } else {
            fs::copy(&target_path, &link_path)?;
        }
    }
    Ok(())
}
