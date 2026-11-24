// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{fs::read_dir, path::PathBuf};

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
    if utils::is_apple_target() {
        let prl_path = PathBuf::from(format!(
            "{}/Qt{qt_module}.framework/Resources/Qt{qt_module}.prl",
            path_lib.display()
        ));
        if prl_path.exists() {
            return Some(prl_path);
        }
    }
    let version_major = qt_version.major;
    // TODO: should this instead read the CARGO_CFG_TARGET_* for the arch?
    for arch in ["_arm64-v8a", "_armeabi-v7a", "_x86", "_x86_64", ""] {
        let lib_name = format!("Qt{version_major}{qt_module}{arch}.prl");
        let prl_path = path_lib.clone().join(lib_with_prefix(&lib_name));
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

fn link_prl_file(
    builder: &mut cc::Build,
    path_lib: PathBuf,
    path_prefix: PathBuf,
    path_plugins: PathBuf,
    path_qml: PathBuf,
    path_prl: PathBuf,
) {
    match std::fs::read_to_string(path_prl.clone()) {
        Ok(prl) => {
            for line in prl.lines() {
                if let Some(line) = line.strip_prefix("QMAKE_PRL_LIBS = ") {
                    parse_cflags::parse_libs_cflags(
                        line.replace(r"$$[QT_INSTALL_LIBS]", &path_lib.to_string_lossy())
                            .replace(r"$$[QT_INSTALL_PREFIX]", &path_prefix.to_string_lossy())
                            .replace(r"$$[QT_INSTALL_PLUGINS]", &path_plugins.to_string_lossy())
                            .replace(r"$$[QT_INSTALL_QML]", &path_qml.to_string_lossy())
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

fn link_static_initializers(static_initializers_dir: PathBuf, builder: &mut cc::Build) {
    for entry in read_dir(static_initializers_dir).unwrap() {
        let plugin_init_dir = entry.unwrap().path();
        if plugin_init_dir.is_dir() {
            for entry in read_dir(plugin_init_dir).unwrap() {
                let object_file = entry.unwrap().path();
                if crate::utils::is_object_file(&object_file) {
                    builder.object(object_file);
                }
            }
        }
    }
}

/// Recursively link all Qt plugins in a directory and its subdirectories
fn link_plugin_dir(
    dir: PathBuf,
    path_prefix: PathBuf,
    path_lib: PathBuf,
    path_plugins: PathBuf,
    path_qml: PathBuf,
    builder: &mut cc::Build,
) {
    let target = std::env::var("TARGET").unwrap();
    for entry in read_dir(dir).unwrap() {
        let plugin_subdir = entry.unwrap().path();
        if plugin_subdir.is_dir() {
            for entry in read_dir(&plugin_subdir).unwrap() {
                let plugin_file = entry.unwrap().path();
                match plugin_file.extension() {
                    Some(extension) => {
                        if extension == "prl" {
                            link_prl_file(
                                builder,
                                path_lib.clone(),
                                path_prefix.clone(),
                                path_plugins.clone(),
                                path_qml.clone(),
                                plugin_file,
                            );
                        // tls and networkinformation plugins have .prl files that do not
                        // specify to link the plugin's library in QMAKE_PRL_LIBS, so link any
                        // library files found regardless of whether a .prl file says to link it.
                        } else if let Some(lib_name) =
                            crate::parse_cflags::extract_lib_from_filename(
                                &target,
                                &plugin_file.file_name().unwrap().to_string_lossy(),
                            )
                        {
                            println!("cargo::rustc-link-search={}", plugin_subdir.display());
                            println!("cargo::rustc-link-lib={lib_name}");
                        }
                    }
                    None => {
                        // QML plugins have more plugins in subdirectories
                        if plugin_file.is_dir() {
                            link_plugin_dir(
                                plugin_subdir.clone(),
                                path_prefix.clone(),
                                path_lib.clone(),
                                path_plugins.clone(),
                                path_qml.clone(),
                                builder,
                            );
                        }
                    }
                }
            }
            // Link static initializers of plugins. These are object files compiled from tiny C++ files
            // that just have a `Q_IMPORT_PLUGIN` macro. These require linking with the `+whole-archive`
            // link modifier or they will be discarded and the plugin won't end up in the final executable.
            //
            // Qt's configure script has a `-debug-and-release` option that uses CMake's Ninja Multi-Config
            // generator. In that case, there is both an `objects-Release` and `objects-Debug` directory.
            // The debug libraries and their .prl files have a `_debug` suffix in their file name
            // (before the file extension), which find_prl_for_qt_module does not search for. Thus, in this
            // case, qt-build-utils links the release libraries, and the static initalizers in the
            // `objects-Release` directory should be preferred to `objects-Debug`.
            let static_initializers_dir = plugin_subdir.join("objects-Release");
            if static_initializers_dir.is_dir() {
                link_static_initializers(static_initializers_dir, builder);
            } else {
                let static_initializers_dir = plugin_subdir.join("objects-Debug");
                if static_initializers_dir.is_dir() {
                    link_static_initializers(static_initializers_dir, builder);
                }
            }
        }
    }
}

pub(crate) fn link_plugins(
    path_prefix: PathBuf,
    path_lib: PathBuf,
    path_plugins: PathBuf,
    path_qml: PathBuf,
    builder: &mut cc::Build,
    qt_version: &semver::Version,
) {
    if is_qt_static(path_lib.clone(), qt_version) || utils::is_emscripten_target() {
        link_plugin_dir(
            path_plugins.clone(),
            path_prefix.clone(),
            path_lib.clone(),
            path_plugins.clone(),
            path_qml.clone(),
            builder,
        );
        link_plugin_dir(
            path_qml.clone(),
            path_prefix.clone(),
            path_lib.clone(),
            path_plugins.clone(),
            path_qml,
            builder,
        );
    }
}

fn is_qt_static(path_lib: PathBuf, qt_version: &semver::Version) -> bool {
    let qtcore_prl = find_prl_for_qt_module("Core", path_lib.clone(), qt_version)
        .expect("Could not find .prl file for QtCore module");
    match std::fs::read_to_string(&qtcore_prl) {
        Ok(prl) => {
            for line in prl.lines() {
                if line == "QMAKE_PRL_CONFIG = static" {
                    return true;
                }
            }
        }
        Err(e) => {
            println!(
                "cargo::warning=Could not open {} file to check if Qt is was built statically: {}",
                qtcore_prl.display(),
                e
            );
        }
    }
    false
}

pub(crate) fn link_for_qt_modules(
    builder: &mut cc::Build,
    qt_modules: &[String],
    path_frameworks: Vec<PathBuf>,
    path_lib: PathBuf,
    path_prefix: PathBuf,
    path_plugins: PathBuf,
    path_qml: PathBuf,
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
        let link_lib = if utils::is_apple_target()
            && path_lib.join(format!("Qt{qt_module}.framework")).exists()
        {
            format!("framework=Qt{qt_module}")
        } else {
            format!("Qt{}{qt_module}", qt_version.major)
        };
        println!("cargo::rustc-link-lib={link_lib}");

        let path_prl = find_prl_for_qt_module(qt_module, path_lib.clone(), &qt_version)
            .unwrap_or_else(|| {
                panic!("Could not find a prl path for Qt module: {qt_module}");
            });

        link_prl_file(
            builder,
            path_lib.clone(),
            path_prefix.clone(),
            path_plugins.clone(),
            path_qml.clone(),
            path_prl,
        );
    }
}
