// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Retrieves the headers for cxx-qt-lib-extras
///
/// These can be passed into [cxx_qt_build::CxxQtBuilder].
pub fn build_opts() -> cxx_qt_build::CxxQtBuildersOpts {
    let mut opts = cxx_qt_build::CxxQtBuildersOpts::default();

    for (file_contents, file_name) in [
        (
            include_str!("../include/core/qelapsedtimer.h"),
            "qelapsedtimer.h",
        ),
        (
            include_str!("../include/core/qcommandlineparser.h"),
            "qcommandlineparser.h",
        ),
        (
            include_str!("../include/core/qcommandlineoption.h"),
            "qcommandlineoption.h",
        ),
        (
            include_str!("../include/gui/qapplication.h"),
            "qapplication.h",
        ),
    ] {
        opts = opts.header(file_contents, "cxx-qt-lib-extras", file_name);
    }

    opts.qt_module("Gui").qt_module("Widgets")
}
