// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::PathBuf;

use crate::utils;

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
