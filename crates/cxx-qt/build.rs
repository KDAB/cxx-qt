// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{fs::File, io::Write};

fn main() {
    let qtbuild = qt_build_utils::QtBuild::new(vec!["Core".to_owned()])
        .expect("Could not find Qt installation");

    // Required for tests
    qt_build_utils::setup_linker();

    let cpp_files = ["src/connection.cpp"];
    let rust_bridges = ["src/connection.rs"];

    for bridge in &rust_bridges {
        println!("cargo:rerun-if-changed={bridge}");
    }

    let mut builder = cxx_build::bridges(rust_bridges);

    qtbuild.cargo_link_libraries(&mut builder);

    for cpp_file in &cpp_files {
        builder.file(cpp_file);
        println!("cargo:rerun-if-changed={cpp_file}");
    }

    // Write this library's manually written C++ headers to files and add them to include paths
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let directory = format!("{out_dir}/cxx-qt");
    std::fs::create_dir_all(&directory).expect("Could not create cxx-qt header directory");
    // Note we only need connection.h for now, but lets move all headers to be consistent
    // ensure src/lib write_headers is consistent
    for (file_contents, file_name) in [
        (include_str!("include/connection.h"), "connection.h"),
        (include_str!("include/locking.h"), "locking.h"),
        (include_str!("include/maybelockguard.h"), "maybelockguard.h"),
        (include_str!("include/signalhandler.h"), "signalhandler.h"),
        (include_str!("include/thread.h"), "thread.h"),
        (include_str!("include/threading.h"), "threading.h"),
        (include_str!("include/type.h"), "type.h"),
    ] {
        let h_path = format!("{directory}/{file_name}");
        let mut header = File::create(h_path).expect("Could not create cxx-qt header");
        write!(header, "{file_contents}").expect("Could not write cxx-qt header");
    }
    builder.include(out_dir);
    builder.includes(qtbuild.include_paths());

    // Note, ensure our settings stay in sync across cxx-qt, cxx-qt-build, and cxx-qt-lib
    builder.cpp(true);
    builder.std("c++17");
    // MSVC
    builder.flag_if_supported("/Zc:__cplusplus");
    builder.flag_if_supported("/permissive-");
    builder.flag_if_supported("/bigobj");
    // MinGW requires big-obj otherwise debug builds fail
    builder.flag_if_supported("-Wa,-mbig-obj");

    builder.compile("cxx-qt");
}
