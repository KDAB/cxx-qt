// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature (default).
        // - Qt Qml is linked by enabling the qt_qml Cargo feature (default).
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network")
        .qt_module("Widgets")
        // Generate C++ from the `#[cxx_qt::bridge]` module
        .file("src/qapplication_cxx.rs")
        .file("src/qpushbutton_cxx.rs")
        .cc_builder(|cc| {
            cc.include("./cpp");
            cc.file("./cpp/qapplication.cpp");
            cc.file("./cpp/qpushbutton.cpp");
        })
        .build();
}
