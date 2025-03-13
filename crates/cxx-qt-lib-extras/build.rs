// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_build::CxxQtBuilder;

fn main() {
    let mut builder = CxxQtBuilder::library()
        .qt_module("Gui")
        .qt_module("Widgets");

    let rust_bridges = vec![
        "core/qelapsedtimer",
        "core/qeventloop",
        "core/qcommandlineoption",
        "core/qcommandlineparser",
        "gui/qapplication",
    ];

    for rust_source in &rust_bridges {
        builder = builder.file(format!("src/{rust_source}.rs"));
    }

    let cpp_files = vec![
        "core/qelapsedtimer",
        "core/qcommandlineoption",
        "core/qcommandlineparser",
        "gui/qapplication",
    ];

    builder = builder.cc_builder(move |cc| {
        for cpp_file in &cpp_files {
            cc.file(format!("src/{cpp_file}.cpp"));
            println!("cargo::rerun-if-changed=src/{cpp_file}.cpp");
        }
        cc.file("src/qt_types.cpp");
        println!("cargo::rerun-if-changed=src/qt_types.cpp");
    });
    println!("cargo::rerun-if-changed=src/assertion_utils.h");

    let interface = builder
        // Use a short name due to the Windows file path limit!
        // We don't re-export these headers anyway.
        .include_prefix("private")
        .build();

    // Disable exporting the standard include directory, as we are exporting custom header
    interface
        .export_include_prefixes([])
        .export_include_directory("cxx-qt-lib-extras", "cxx-qt-lib-extras")
        .reexport_dependency("cxx-qt-lib")
        .export();
}
