// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cargo_executable_build_rs
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        // Link Qt's Qml and Network libraries. Qt Core and Gui are always
        // linked, so there is no need to specify them here.
        .qt_modules(&["Qml", "Network"])
        // Generate C++ from the `#[cxx_qt::bridge]` module
        .file("src/cxxqt_object.rs")
        // Generate C++ code from the .qrc file with the rcc tool
        // https://doc.qt.io/qt-6/resources.html
        .qrc("qml/qml.qrc")
        // Tell CxxQtBuilder's internal cc::Build struct to compile the manually
        // written C++ file in addition to the generated C++.
        .cc_builder(|cc| {
            cc.file("src/cpp/run.cpp");
            println!("cargo:rerun-if-changed=src/cpp/run.cpp");
        })
        .build();
}
// ANCHOR_END: book_cargo_executable_build_rs
