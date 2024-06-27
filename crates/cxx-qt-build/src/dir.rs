// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{crate_name, module_name_from_uri};
use std::{env, path::PathBuf};

/// The export directory, if one was specified through the environment, namespaced by crate
pub(crate) fn crate_target() -> Option<PathBuf> {
    target().map(|export_dir| export_dir.join("crates").join(crate_name()))
}

/// The export directory, if one was specified through the environment, namespaced by plugin
pub(crate) fn module_target(module_uri: &str) -> Option<PathBuf> {
    target().map(|export_dir| {
        export_dir
            .join("qml_modules")
            .join(module_name_from_uri(module_uri))
    })
}

/// The export directory, if one was specified through the environment.
/// Note that this is not namspaced by crate.
pub(crate) fn target() -> Option<PathBuf> {
    env::var("CXXQT_EXPORT_DIR").ok().map(PathBuf::from)
}

/// The include directory needs to be namespaced by crate name when exporting for a C++ build system,
/// but for using cargo build without a C++ build system, OUT_DIR is already namespaced by crate name.
pub(crate) fn header_root() -> PathBuf {
    crate_target()
        .unwrap_or_else(|| PathBuf::from(env::var("OUT_DIR").unwrap()))
        .join("include")
}

/// The OUT_DIR, converted into a PathBuf
pub(crate) fn out() -> PathBuf {
    env::var("OUT_DIR").unwrap().into()
}
