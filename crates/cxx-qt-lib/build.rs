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
