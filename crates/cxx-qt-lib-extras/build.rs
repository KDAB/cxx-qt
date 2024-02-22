// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_build::CxxQtBuilder;
use qt_build_utils::QtBuild;

fn main() {
    let emscripten_targeted = match std::env::var("CARGO_CFG_TARGET_OS") {
        Ok(val) => val == "emscripten",
        Err(_) => false,
    };

    let mut builder = CxxQtBuilder::new();

    // Find the Qt version and tell the Rust compiler
    // this allows us to have conditional Rust code
    //
    // TODO: is this useful to have in cxx-qt-build?
    println!(
        "cargo:rustc-cfg=qt_version_major=\"{}\"",
        QtBuild::new(vec!())
            .expect("Could not find Qt installation")
            .version()
            .major
    );

    let mut rust_bridges = vec![
        "core/qcommandlineoption",
    ];

    for rust_source in &rust_bridges {
        builder = builder.file(format!("src/{rust_source}.rs"));
    }

    let mut cpp_files = vec![
        "core/qcommandlineoption",
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

    builder.with_opts(cxx_qt_lib_extras_headers::build_opts()).build();
}
