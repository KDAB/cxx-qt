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

/// On Unix platforms, included files are symlinked into destination folders.
/// On non-Unix platforms, due to poor support for symlinking, included files are deep copied.
#[cfg(unix)]
pub(crate) const INCLUDE_VERB: &str = "create symlink";
/// On Unix platforms, included files are symlinked into destination folders.
/// On non-Unix platforms, due to poor support for symlinking, included files are deep copied.
#[cfg(not(unix))]
pub(crate) const INCLUDE_VERB: &str = "deep copy files";

pub(crate) fn gen() -> PathBuf {
    // Use a short name due to the Windows file path limit!
    out().join("cxxqtgen")
}

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
    let path = target();
    if is_exporting_crate() {
        path.join("crates").join(crate_name())
    } else {
        // If we're not exporting, use a shortened path
        // The paths for files in the OUT_DIR can get pretty long, especially if combined with
        // Corrosion/CMake.
        // This is an issue, as Windows has a maximum path length of 260 characters.
        // The OUT_DIR is already namespaced by crate name, so we don't need to prefix again.
        // See also: https://github.com/KDAB/cxx-qt/issues/1237
        path
    }
}

/// The target directory, namespaced by QML module
pub(crate) fn module_target(module_uri: &str) -> PathBuf {
    module_export(module_uri).unwrap_or_else(|| {
        out()
            // Use a short name due to the Windows file path limit!
            .join("cxxqtqml")
            .join(module_name_from_uri(module_uri))
    })
}

/// The export directory, namespaced by QML module
///
/// In conctrast to the crate_export directory, this is `Some` for downstream dependencies as well.
/// This allows CMake to import QML modules from dependencies.
///
/// TODO: This may conflict if two dependencies are building QML modules with the same name!
/// We should probably include a lockfile here to avoid this.
pub(crate) fn module_export(module_uri: &str) -> Option<PathBuf> {
    // In contrast to crate_export, we don't need to check for the specific crate here.
    // QML modules should always be exported.
    env::var("CXX_QT_EXPORT_DIR")
        .ok()
        .map(PathBuf::from)
        .map(|dir| {
            dir.join("qml_modules")
                .join(module_name_from_uri(module_uri))
        })
}

/// The target directory or another directory where we can write files that will be shared
/// between crates.
pub(crate) fn target() -> PathBuf {
    if let Some(export) = crate_export() {
        return export;
    }

    // Use a short name due to the Windows file path limit!
    out().join("cxxqtbuild")
}

/// The export directory, if one was specified through the environment.
/// Note that this is not namspaced by crate.
pub(crate) fn crate_export() -> Option<PathBuf> {
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

pub(crate) fn is_exporting_crate() -> bool {
    crate_export().is_some()
}

pub(crate) fn initializers(key: &str) -> PathBuf {
    let path = out().join("cxx-qt-build").join("initializers").join(key);
    std::fs::create_dir_all(&path).expect("Failed to create initializers path!");
    path
}

pub(crate) fn manifest() -> Option<PathBuf> {
    std::env::var("CARGO_MANIFEST_DIR").ok().map(PathBuf::from)
}

#[cfg(unix)]
pub(crate) fn symlink_or_copy_directory(
    source: impl AsRef<Path>,
    dest: impl AsRef<Path>,
) -> Result<bool> {
    match std::os::unix::fs::symlink(&source, &dest) {
        Ok(()) => Ok(true),
        Err(e) if e.kind() != std::io::ErrorKind::AlreadyExists => Err(e),
        // Two dependencies may be reexporting the same shared dependency, which will
        // result in conflicting symlinks.
        // Try detecting this by resolving the symlinks and checking whether this leads us
        // to the same paths. If so, it's the same include path for the same prefix, which
        // is fine.
        Err(_) => Ok(fs::canonicalize(source)? == fs::canonicalize(dest)?),
    }
}

#[cfg(not(unix))]
pub(crate) fn symlink_or_copy_directory(
    source: impl AsRef<Path>,
    dest: impl AsRef<Path>,
) -> Result<bool> {
    deep_copy_directory(source.as_ref(), dest.as_ref())
}

#[cfg(not(unix))]
fn deep_copy_directory(source: &Path, dest: &Path) -> Result<bool> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            if deep_copy_directory(&source_path, &dest_path)? {
                continue;
            }
            return Ok(false);
        }
        if !dest_path.try_exists()? {
            fs::copy(&source_path, &dest_path)?;
        } else if files_conflict(&source_path, &dest_path)? {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(not(unix))]
fn files_conflict(source: &Path, dest: &Path) -> Result<bool> {
    use fs::File;
    use std::io::{BufRead, BufReader};
    let source = File::open(source)?;
    let dest = File::open(dest)?;
    if source.metadata()?.len() != dest.metadata()?.len() {
        return Ok(true);
    }
    let mut source = BufReader::new(source);
    let mut dest = BufReader::new(dest);
    loop {
        let source_bytes = source.fill_buf()?;
        let bytes_len = source_bytes.len();
        let dest_bytes = dest.fill_buf()?;
        let bytes_len = bytes_len.min(dest_bytes.len());
        if bytes_len == 0 {
            return Ok(false);
        }
        if source_bytes[..bytes_len] != dest_bytes[..bytes_len] {
            return Ok(true);
        }
        source.consume(bytes_len);
        dest.consume(bytes_len);
    }
}
