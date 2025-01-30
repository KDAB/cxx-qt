// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::PathBuf;

use cxx_qt_build::{CxxQtBuilder, Interface};

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
    write_headers();

    let interface = Interface::default()
        .export_include_prefixes([])
        .export_include_directory(header_dir(), "cxx-qt");

    let mut builder = CxxQtBuilder::library(interface);

    let cpp_files = ["src/connection.cpp"];
    let rust_bridges = ["src/connection.rs"];

    for bridge in &rust_bridges {
        builder = builder.file(bridge);
    }

    builder = builder.cc_builder(move |cc| {
        for cpp_file in &cpp_files {
            cc.file(cpp_file);
            println!("cargo::rerun-if-changed={cpp_file}");
        }
    });

    // TODO: This is not ideal as it requires CXX-Qt to depend on qt_build_utils directly and
    // causes a duplicate search for the Qt installation.
    let qtbuild = qt_build_utils::QtBuild::new(vec![]).expect("Could not find Qt installation");
    if qtbuild.version().major == 5 {
        // If we are using Qt 5 then write the std_types source
        // This registers std numbers as a type for use in QML
        //
        // Note that we need this to be compiled into the initializers library
        // as they are stored in statics in the source.
        //
        // See also:
        // https://github.com/rust-lang/rust/issues/108081
        // https://github.com/KDAB/cxx-qt/pull/598
        builder = builder.initializer(qt_build_utils::Initializer {
            file: Some("src/std_types_qt5.cpp".into()),
            ..qt_build_utils::Initializer::default_signature("cxx_qt_init_std_types_qt5")
        });
    }

    builder.build();
}
