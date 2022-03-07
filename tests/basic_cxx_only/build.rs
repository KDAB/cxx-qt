// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use clang_format::ClangFormatStyle;
use cxx_qt_build::CxxQtBuilder;

fn main() {
    // TODO: Further options for building will go here similar to cpp_format
    // eg if you want a QQmlExtensionModule etc
    CxxQtBuilder::new()
        .disable_qt()
        .cpp_format(ClangFormatStyle::Mozilla)
        .file("src/lib.rs")
        .build();
}
