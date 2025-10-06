// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .file("src/lib.rs")
        .qrc("../images/images.qrc")
        .qml_module(QmlModule {
            uri: "com.kdab.energy",
            qml_files: &[
                "../qml/Button.qml",
                "../qml/MainWindow.qml",
                "../qml/Panel.qml",
                "../qml/SensorUI.qml",
                "../qml/SideText.qml",
            ],
            ..Default::default()
        })
        .files(["src/lib.rs"])
        .build();
}
