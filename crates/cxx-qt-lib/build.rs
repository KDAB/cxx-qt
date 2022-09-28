// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

fn main() {
    let qt_modules = vec!["Core", "Gui"]
        .iter()
        .map(|m| String::from(*m))
        .collect();
    let qtbuild = qt_build::QtBuild::new(qt_modules).expect("Could not find Qt installation");
    qtbuild.cargo_link_libraries();

    // Find the Qt version and tell the Rust compiler
    // this allows us to have conditional Rust code
    println!(
        "cargo:rustc-cfg=qt_version_major=\"{}\"",
        qtbuild.version().major
    );

    let bridges = [
        "qcolor",
        "qdate",
        "qdatetime",
        "qpoint",
        "qpointf",
        "qrect",
        "qrectf",
        "qsize",
        "qsizef",
        "qstring",
        "qtime",
        "qurl",
        "qvariant",
    ];
    for bridge in bridges {
        println!("cargo:rerun-if-changed=src/types/{}.rs", bridge);
    }

    for include_path in qtbuild.include_paths() {
        cxx_build::CFG
            .exported_header_dirs
            .push(include_path.as_path());
    }

    let mut builder = cxx_build::bridges(
        bridges
            .iter()
            .map(|bridge| format!("src/types/{}.rs", bridge)),
    );
    for bridge in bridges {
        builder.file(format!("src/types/{}.cpp", bridge));
        println!("cargo:rerun-if-changed=src/types/{}.cpp", bridge);
    }
    builder.file("src/qt_types.cpp");
    println!("cargo:rerun-if-changed=src/qt_types.cpp");

    // Write this library's manually written C++ headers to files and add them to include paths
    let out_dir = std::env::var("OUT_DIR").unwrap();
    cxx_qt_lib_headers::write_headers(&format!("{}/cxx-qt-lib/include", out_dir));
    builder.include(out_dir);

    // MSVC
    builder.flag_if_supported("/std:c++17");
    builder.flag_if_supported("/Zc:__cplusplus");
    builder.flag_if_supported("/permissive-");
    // GCC + Clang
    builder.flag_if_supported("-std=c++17");
    builder.compile("cxx-qt-lib");
}
