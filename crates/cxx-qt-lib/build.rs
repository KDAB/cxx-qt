// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

fn main() {
    let qt_modules = vec!["Core", "Gui"]
        .iter()
        .map(|m| String::from(*m))
        .collect();
    let qtbuild = qt_build_utils::QtBuild::new(qt_modules).expect("Could not find Qt installation");
    qtbuild.cargo_link_libraries();

    // Find the Qt version and tell the Rust compiler
    // this allows us to have conditional Rust code
    println!(
        "cargo:rustc-cfg=qt_version_major=\"{}\"",
        qtbuild.version().major
    );

    let rust_bridges = [
        "qcolor",
        "qdate",
        "qdatetime",
        "qhash/qhash_qstring_qvariant",
        "qmodelindex",
        "qpersistentmodelindex",
        "qpoint",
        "qpointf",
        "qrect",
        "qrectf",
        "qset/qset_bool",
        "qset/qset_f32",
        "qset/qset_f64",
        "qset/qset_i8",
        "qset/qset_i16",
        "qset/qset_i32",
        "qset/qset_i64",
        "qset/qset_qdate",
        "qset/qset_qdatetime",
        "qset/qset_qstring",
        "qset/qset_qtime",
        "qset/qset_qurl",
        "qset/qset_u8",
        "qset/qset_u16",
        "qset/qset_u32",
        "qset/qset_u64",
        "qsize",
        "qsizef",
        "qstring",
        "qtime",
        "qurl",
        "qvariant",
        "qvector/qvector_bool",
        "qvector/qvector_f32",
        "qvector/qvector_f64",
        "qvector/qvector_i8",
        "qvector/qvector_i16",
        "qvector/qvector_i32",
        "qvector/qvector_i64",
        "qvector/qvector_qcolor",
        "qvector/qvector_qdate",
        "qvector/qvector_qdatetime",
        "qvector/qvector_qpoint",
        "qvector/qvector_qpointf",
        "qvector/qvector_qrect",
        "qvector/qvector_qrectf",
        "qvector/qvector_qsize",
        "qvector/qvector_qsizef",
        "qvector/qvector_qstring",
        "qvector/qvector_qtime",
        "qvector/qvector_qurl",
        "qvector/qvector_qvariant",
        "qvector/qvector_u8",
        "qvector/qvector_u16",
        "qvector/qvector_u32",
        "qvector/qvector_u64",
    ];
    for bridge in rust_bridges {
        println!("cargo:rerun-if-changed=src/types/{}.rs", bridge);
    }

    for include_path in qtbuild.include_paths() {
        cxx_build::CFG
            .exported_header_dirs
            .push(include_path.as_path());
    }

    let mut builder = cxx_build::bridges(
        rust_bridges
            .iter()
            .map(|bridge| format!("src/types/{}.rs", bridge)),
    );

    let cpp_files = [
        "qcolor",
        "qdate",
        "qdatetime",
        "qhash/qhash",
        "qmodelindex",
        "qpersistentmodelindex",
        "qpoint",
        "qpointf",
        "qrect",
        "qrectf",
        "qset/qset",
        "qsize",
        "qsizef",
        "qstring",
        "qtime",
        "qurl",
        "qvariant",
        "qvector/qvector",
    ];
    for cpp_file in cpp_files {
        builder.file(format!("src/types/{}.cpp", cpp_file));
        println!("cargo:rerun-if-changed=src/types/{}.cpp", cpp_file);
    }
    builder.file("src/qt_types.cpp");
    println!("cargo:rerun-if-changed=src/qt_types.cpp");
    println!("cargo:rerun-if-changed=src/types/assertion_utils.h");

    // Write this library's manually written C++ headers to files and add them to include paths
    let out_dir = std::env::var("OUT_DIR").unwrap();
    cxx_qt_lib_headers::write_headers(format!("{}/cxx-qt-lib", out_dir));
    builder.include(out_dir);

    // MSVC
    builder.flag_if_supported("/std:c++17");
    builder.flag_if_supported("/Zc:__cplusplus");
    builder.flag_if_supported("/permissive-");
    // GCC + Clang
    builder.flag_if_supported("-std=c++17");
    builder.compile("cxx-qt-lib");
}
