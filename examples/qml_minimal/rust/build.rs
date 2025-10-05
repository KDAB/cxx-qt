// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_build_rs
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        // ANCHOR: book_qml_module
        .qml_module(QmlModule::<&str, &str> {
            uri: "com.kdab.cxx_qt.demo",
            qml_files: &["../qml/main.qml"],
            ..Default::default()
        })
        .files(["src/cxxqt_object.rs"])
        // ANCHOR_END: book_qml_module
        .build();
}
// ANCHOR_END: book_build_rs
