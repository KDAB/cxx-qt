// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::PathBuf;

use cxx_qt_build::CxxQtBuilder;

fn header_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("include")
        .join("cxx-qt")
}

fn write_headers() {
    println!("cargo::rerun-if-changed=include/");
    std::fs::create_dir_all(header_dir()).expect("Failed to create include directory");

    for file_path in [
        "connection.h",
        "casting.h",
        "signalhandler.h",
        "thread.h",
        "threading.h",
        "type.h",
    ] {
        println!("cargo::rerun-if-changed=include/{file_path}");
        std::fs::copy(format!("include/{file_path}"), header_dir().join(file_path))
            .expect("Failed to copy header file!");
    }
}

fn main() {
    // TODO: should we even have this ? Instead pass in an include_directory to our
    // builder that handles this?
    // As they are first being copied to the OUT_DIR/include/<crate> and then
    // to the target/cxx-qt-build/../include
    //
    // This then causes problems because we try to symlink this dir now after
    // the CXX files are written already
    write_headers();

    let mut builder = CxxQtBuilder::library();

    let cpp_files = ["src/connection.cpp"];
    let rust_bridges = ["src/connection.rs", "src/qobject.rs"];

    for bridge in &rust_bridges {
        builder = builder.file(bridge);
    }

    builder = builder.cc_builder(move |cc| {
        for cpp_file in &cpp_files {
            cc.file(cpp_file);
            println!("cargo::rerun-if-changed={cpp_file}");
        }

        // TODO: before this came from the export_include_directory
        // but now that we export after the build it fails
        // should we always include the crate dir like CXX?
        cc.include(header_dir().parent().expect("header_dir has a parent"));
    });
    builder = builder.initializer(qt_build_utils::Initializer {
        file: Some("src/init.cpp".into()),
        ..qt_build_utils::Initializer::default_signature("init_cxx_qt_core")
    });

    let interface = builder.build();
    interface
        .export_include_prefixes([])
        .export_include_directory(header_dir(), "cxx-qt")
        .export();
}
