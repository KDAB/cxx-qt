// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
// SPDX-FileContributor: Quentin Weber <quentin.weber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    let qml_module = QmlModule::new("com.kdab.cxx_qt.span_inspector").qml_file("qml/main.qml");
    CxxQtBuilder::new_qml_module(qml_module)
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature of cxx-qt-lib.
        // - Qt Qml is linked by enabling the qt_qml Cargo feature of cxx-qt-lib.
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network")
        .qt_module("Quick")
        .file("src/inspector.rs")
        .qobject_header("include/helper.h")
        .build();
    println!("cargo:rerun-if-changed=include/helper.h");
}
