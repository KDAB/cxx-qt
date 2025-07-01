// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qml_module(QmlModule::<_, &str> {
            uri: "com.kdab.cxx_qt.demo.sub2",
            rust_files: &["src/sub2_object.rs"],
            ..Default::default()
        })
        .build()
        .export();
}
