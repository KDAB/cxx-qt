// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .qt_modules(&["Qml", "Network"])
        .file("src/cxxqt_object.rs")
        .qrc("src/qml.qrc")
        .cc_builder(|cc| {
            cc.file("src/run.cpp");
            println!("cargo:rerun-if-changed=src/run.cpp");
        })
        .build();
}
