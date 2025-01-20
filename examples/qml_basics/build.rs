// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
use cxx_qt_build::{CxxQtBuilder, QmlModule};
fn main() {
    CxxQtBuilder::new()
        .qml_module(QmlModule {
            uri: "com.kdab.tutorial",
            qml_files: &["qml/main.qml"],
            rust_files: &["src/main.rs"],
            ..Default::default()
        })
        .qt_module("Network")
        .build();
}
