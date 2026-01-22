// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .crate_include_root(Some("include".to_owned()))
        .files(["src/connection.rs", "src/qobject.rs"])
        .cpp_files(["src/connection.cpp"])
        .initializer(qt_build_utils::Initializer {
            file: Some("src/init.cpp".into()),
            ..qt_build_utils::Initializer::default_signature("init_cxx_qt_core")
        })
        .build()
        .export();
}
