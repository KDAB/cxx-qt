// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("com.kdab.energy").qml_files([
        "../qml/Button.qml",
        "../qml/MainWindow.qml",
        "../qml/Panel.qml",
        "../qml/SensorUI.qml",
        "../qml/SideText.qml",
    ]))
    .file("src/lib.rs")
    .qrc("../images/images.qrc")
    .files(["src/lib.rs"])
    .build();
}
