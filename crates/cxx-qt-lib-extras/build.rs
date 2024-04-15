// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_build::CxxQtBuilder;

fn main() {
    let mut builder = CxxQtBuilder::new();

    let rust_bridges = vec![
        "core/qelapsedtimer",
        "core/qcommandlineoption",
        "core/qcommandlineparser",
    ];

    for rust_source in &rust_bridges {
        builder = builder.file(format!("src/{rust_source}.rs"));
    }

    let cpp_files = vec![
        "core/qelapsedtimer",
        "core/qcommandlineoption",
        "core/qcommandlineparser",
    ];

    builder = builder.cc_builder(move |cc| {
        for cpp_file in &cpp_files {
            cc.file(format!("src/{cpp_file}.cpp"));
            println!("cargo:rerun-if-changed=src/{cpp_file}.cpp");
        }
        cc.file("src/qt_types.cpp");
        println!("cargo:rerun-if-changed=src/qt_types.cpp");
    });
    println!("cargo:rerun-if-changed=src/assertion_utils.h");

    builder
        .with_opts(cxx_qt_lib_headers::build_opts())
        .with_opts(cxx_qt_lib_extras_headers::build_opts())
        .build();
}
