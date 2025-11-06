// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_build::{CxxQtBuilder, QmlFile, QmlModule};

fn main() {
    // The sub3 crate exists specifically to test building crates that only define QML files, and
    // no Rust QML components.
    // This allows distributing pure QML modules as Rust crates! 🚢🦀
    CxxQtBuilder::new_qml_module(
        QmlModule::new("com.kdab.cxx_qt.demo.sub3")
            .qml_file("qml/RedRect.qml")
            .qml_file(QmlFile::from("qml/Singleton.qml").singleton(true)),
    )
    .build()
    .export();
}
