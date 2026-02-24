// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::PathBuf;

use crate::{parse_cflags, utils};

/// Find a prl file for a Qt module
///
/// Note that sometimes prl files use their architecture naming scheme,
/// so try them first then fallback to none.
pub(crate) fn find_prl_for_qt_module(
    qt_module: &str,
    path_lib: PathBuf,
    qt_version: &semver::Version,
) -> Option<PathBuf> {
    let version_major = qt_version.major;
    // TODO: should this instead read the CARGO_CFG_TARGET_* for the arch?
    for arch in ["_arm64-v8a", "_armeabi-v7a", "_x86", "_x86_64", ""] {
        let lib_name = format!("Qt{version_major}{qt_module}{arch}.prl");
        let prl_path = path_lib.join(lib_with_prefix(&lib_name));
        // NOTE: this was try_exists before, do we need to use try_exists?
        if prl_path.exists() {
            return Some(prl_path);
        }
    }

    None
}

pub(crate) fn framework_paths_for_qt_modules(
    _qt_modules: &[String],
    path_lib: PathBuf,
) -> Vec<PathBuf> {
    let mut paths = vec![];

    if utils::is_apple_target() {
        // Note that this adds the framework path which allows for
        // includes such as <QtCore/QObject> to be resolved correctly
        paths.push(path_lib);
    }

    paths.into_iter().filter(|path| path.exists()).collect()
}

pub(crate) fn include_paths_for_qt_modules(
    qt_modules: &[String],
    path_include: PathBuf,
    path_lib: PathBuf,
) -> Vec<PathBuf> {
    let mut paths = vec![];

    for qt_module in qt_modules {
        // Add the usual location for the Qt module
        paths.push(path_include.join(format!("Qt{qt_module}")));

        // TODO: should we add the private header?

        // Ensure that we add any framework's headers path
        //
        // Note that the individual Qt modules should in theory work
        // by giving `-framework QtCore` to the cc builder. However these
        // appear to be lost in flag_if_supported.
        //
        // Also note we still need these include directs even with the -F / framework paths
        // as otherwise only <QtCore/QtGlobal> works but <QtGlobal> does not.
        if utils::is_apple_target() {
            paths.push(
                path_lib
                    .join(format!("Qt{qt_module}.framework"))
                    .join("Headers"),
            );
        }
    }

    // Add the QT_INSTALL_HEADERS itself
    paths.push(path_include);

    paths.into_iter().filter(|path| path.exists()).collect()
}

fn lib_with_prefix(lib_name: &str) -> String {
    let prefix = if utils::is_windows_target() {
        ""
    } else {
        "lib"
    };

    format!("{prefix}{lib_name}")
}

fn link_qt_module_prl(
    builder: &mut cc::Build,
    link_lib: String,
    path_lib: PathBuf,
    path_prefix: PathBuf,
    path_prl: PathBuf,
) {
    println!("cargo::rustc-link-lib={link_lib}");

    match std::fs::read_to_string(path_prl.clone()) {
        Ok(prl) => {
            for line in prl.lines() {
                if let Some(line) = line.strip_prefix("QMAKE_PRL_LIBS = ") {
                    parse_cflags::parse_libs_cflags(
                        line.replace(r"$$[QT_INSTALL_LIBS]", &path_lib.to_string_lossy())
                            .replace(r"$$[QT_INSTALL_PREFIX]", &path_prefix.to_string_lossy())
                            .as_bytes(),
                        builder,
                    );
                }
            }
        }
        Err(e) => {
            println!(
                "cargo::warning=Could not open {} file to read libraries to link: {}",
                path_prl.display(),
                e
            );
        }
    }
}

pub(crate) fn link_for_qt_modules(
    builder: &mut cc::Build,
    qt_modules: &[String],
    path_frameworks: Vec<PathBuf>,
    path_lib: PathBuf,
    path_prefix: PathBuf,
    path_plugins: PathBuf,
    qt_version: semver::Version,
) {
    println!("cargo::rustc-link-search={}", path_lib.display());

    // Add the QT_INSTALL_LIBS as a framework link search path as well
    //
    // Note that leaving the kind empty should default to all,
    // but this doesn't appear to find frameworks in all situations
    // https://github.com/KDAB/cxx-qt/issues/885
    //
    // Note this doesn't have an adverse affect running all the time
    // as it appears that all rustc-link-search are added
    //
    // Note that this adds the framework path which allows for
    // includes such as <QtCore/QObject> to be resolved correctly
    if utils::is_apple_target() {
        println!("cargo::rustc-link-search=framework={}", path_lib.display());

        // Ensure that any framework paths are set to -F
        for framework_path in path_frameworks {
            builder.flag_if_supported(format!("-F{}", framework_path.display()));
            // Also set the -rpath otherwise frameworks can not be found at runtime
            println!(
                "cargo::rustc-link-arg=-Wl,-rpath,{}",
                framework_path.display()
            );
        }
    }

    for qt_module in qt_modules {
        let framework = if utils::is_apple_target() {
            path_lib.join(format!("Qt{qt_module}.framework")).exists()
        } else {
            false
        };

        let (link_lib, path_prl) = if framework {
            (
                format!("framework=Qt{qt_module}"),
                path_lib.join(format!(
                    "Qt{qt_module}.framework/Resources/Qt{qt_module}.prl"
                )),
            )
        } else {
            (
                format!("Qt{}{qt_module}", qt_version.major),
                super::shared::find_prl_for_qt_module(qt_module, path_lib.clone(), &qt_version)
                    .unwrap_or_else(|| {
                        panic!("Could not find a prl path for Qt module: {qt_module}");
                    }),
            )
        };

        super::shared::link_qt_module_prl(
            builder,
            link_lib.clone(),
            path_lib.clone(),
            path_prefix.clone(),
            path_prl,
        );
    }

    if utils::is_emscripten_target() {
        let path_platforms = path_plugins.join("platforms");
        println!("cargo::rustc-link-search={}", path_platforms.display());
        let path_prl = path_platforms.join("libqwasm.prl");
        super::shared::link_qt_module_prl(
            builder,
            "qwasm".to_string(),
            path_lib,
            path_prefix,
            path_prl,
        );
    }
}
