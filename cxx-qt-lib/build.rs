// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::env;

fn main() {
    let qt_modules = vec!["Core", "Gui"]
        .iter()
        .map(|m| String::from(*m))
        .collect();
    let qtbuild = qt_build::QtBuild::new(qt_modules).expect("Could not find Qt installation");
    qtbuild.cargo_link_libraries();

    // Copy qt_types.h so C++ build systems can #include it.
    // By design, CARGO_TARGET_DIR is not set by cargo when running build scripts.
    // Copying the header is only needed for making the header available to a C++
    // build system, in which case CARGO_TARGET_DIR will be set by
    // the C++ build system.
    println!("cargo:rerun-if-changed=include/qt_types.h");
    println!("cargo:rerun-if-env-changed=CARGO_TARGET_DIR");
    if let Ok(target_dir) = env::var("CARGO_TARGET_DIR") {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        std::fs::create_dir_all(&format!("{}/cxxbridge/cxx-qt-lib/include", target_dir)).unwrap();
        std::fs::copy(
            &format!("{}/include/qt_types.h", manifest_dir),
            &format!("{}/cxxbridge/cxx-qt-lib/include/qt_types.h", target_dir),
        )
        .unwrap();
    }

    let bridge_files = [
        "src/types/qcolor.rs",
        "src/types/qdate.rs",
        "src/types/qdatetime.rs",
        "src/types/qpoint.rs",
        "src/types/qpointf.rs",
        "src/types/qrect.rs",
        "src/types/qrectf.rs",
        "src/types/qsize.rs",
        "src/types/qsizef.rs",
        "src/types/qstring.rs",
        "src/types/qtime.rs",
        "src/types/qurl.rs",
        "src/types/qvariant.rs",
        "src/types/update_requester.rs",
    ];
    for bridge_file in bridge_files {
        println!("cargo:rerun-if-changed={}", bridge_file);
    }

    for include_path in qtbuild.include_paths() {
        cxx_build::CFG
            .exported_header_dirs
            .push(include_path.as_path());
    }

    let mut builder = cxx_build::bridges(&bridge_files);
    for cpp_file in ["src/qt_types.cpp"] {
        builder.file(cpp_file);
        println!("cargo:rerun-if-changed={}", cpp_file);
    }
    // MSVC
    builder.flag_if_supported("/std:c++17");
    builder.flag_if_supported("/Zc:__cplusplus");
    builder.flag_if_supported("/permissive-");
    // GCC + Clang
    builder.flag_if_supported("-std=c++17");
    builder.compile("cxx-qt-lib");
}
