// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use clang_format::ClangFormatStyle;
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .qqmlextensionplugin(
            "com.kdab.cxx_qt.demo",
            "example_basic_cxx_qt_qml_plugin_qmlplugin",
        )
        .cpp_format(ClangFormatStyle::Mozilla)
        .build();
}
