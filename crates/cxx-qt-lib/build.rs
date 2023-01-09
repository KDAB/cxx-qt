// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

fn main() {
    let feature_qt_gui_enabled = std::env::var("CARGO_FEATURE_QT_GUI").is_ok();

    let mut qt_modules = vec!["Core".to_owned()];
    if feature_qt_gui_enabled {
        qt_modules.push("Gui".to_owned());
    }

    let qtbuild = qt_build_utils::QtBuild::new(qt_modules).expect("Could not find Qt installation");
    qtbuild.cargo_link_libraries();
    // Required for tests
    qt_build_utils::setup_linker();

    // Find the Qt version and tell the Rust compiler
    // this allows us to have conditional Rust code
    println!(
        "cargo:rustc-cfg=qt_version_major=\"{}\"",
        qtbuild.version().major
    );

    let mut rust_bridges = vec![
        "qbytearray",
        "qdate",
        "qdatetime",
        "qhash/qhash_i32_qbytearray",
        "qhash/qhash_qstring_qvariant",
        "qlist/qlist_bool",
        "qlist/qlist_f32",
        "qlist/qlist_f64",
        "qlist/qlist_i8",
        "qlist/qlist_i16",
        "qlist/qlist_i32",
        "qlist/qlist_i64",
        "qlist/qlist_qdate",
        "qlist/qlist_qdatetime",
        "qlist/qlist_qpoint",
        "qlist/qlist_qpointf",
        "qlist/qlist_qrect",
        "qlist/qlist_qrectf",
        "qlist/qlist_qsize",
        "qlist/qlist_qsizef",
        "qlist/qlist_qstring",
        "qlist/qlist_qtime",
        "qlist/qlist_qurl",
        "qlist/qlist_qvariant",
        "qlist/qlist_u8",
        "qlist/qlist_u16",
        "qlist/qlist_u32",
        "qlist/qlist_u64",
        "qmap/qmap_qstring_qvariant",
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
        "qset/qset_qbytearray",
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
        "qstringlist",
        "qtime",
        "qurl",
        "qvariant/mod",
        "qvariant/qvariant_bool",
        "qvariant/qvariant_f32",
        "qvariant/qvariant_f64",
        "qvariant/qvariant_i8",
        "qvariant/qvariant_i16",
        "qvariant/qvariant_i32",
        "qvariant/qvariant_i64",
        "qvariant/qvariant_qdate",
        "qvariant/qvariant_qdatetime",
        "qvariant/qvariant_qpoint",
        "qvariant/qvariant_qpointf",
        "qvariant/qvariant_qrect",
        "qvariant/qvariant_qrectf",
        "qvariant/qvariant_qsize",
        "qvariant/qvariant_qsizef",
        "qvariant/qvariant_qstring",
        "qvariant/qvariant_qtime",
        "qvariant/qvariant_qurl",
        "qvariant/qvariant_u8",
        "qvariant/qvariant_u16",
        "qvariant/qvariant_u32",
        "qvariant/qvariant_u64",
        "qvector/qvector_bool",
        "qvector/qvector_f32",
        "qvector/qvector_f64",
        "qvector/qvector_i8",
        "qvector/qvector_i16",
        "qvector/qvector_i32",
        "qvector/qvector_i64",
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

    if feature_qt_gui_enabled {
        rust_bridges.extend([
            "qcolor",
            "qlist/qlist_qcolor",
            "qvariant/qvariant_qcolor",
            "qvector/qvector_qcolor",
            "qvector2d",
            "qvector3d",
            "qvector4d",
        ]);
    }

    for bridge in &rust_bridges {
        println!("cargo:rerun-if-changed=src/types/{bridge}.rs");
    }

    for include_path in qtbuild.include_paths() {
        cxx_build::CFG
            .exported_header_dirs
            .push(include_path.as_path());
    }

    let mut builder = cxx_build::bridges(
        rust_bridges
            .iter()
            .map(|bridge| format!("src/types/{bridge}.rs")),
    );

    let mut cpp_files = vec![
        "qbytearray",
        "qdate",
        "qdatetime",
        "qhash/qhash",
        "qlist/qlist",
        "qmap/qmap",
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
        "qstringlist",
        "qtime",
        "qurl",
        "qvariant/qvariant",
        "qvector/qvector",
    ];

    if feature_qt_gui_enabled {
        cpp_files.extend(["qcolor", "qvector2d", "qvector3d", "qvector4d"]);
    }

    for cpp_file in &cpp_files {
        builder.file(format!("src/types/{cpp_file}.cpp"));
        println!("cargo:rerun-if-changed=src/types/{cpp_file}.cpp");
    }
    builder.file("src/qt_types.cpp");
    println!("cargo:rerun-if-changed=src/qt_types.cpp");
    println!("cargo:rerun-if-changed=src/types/assertion_utils.h");

    // Write this library's manually written C++ headers to files and add them to include paths
    let out_dir = std::env::var("OUT_DIR").unwrap();
    cxx_qt_lib_headers::write_headers(format!("{out_dir}/cxx-qt-lib"));
    builder.include(out_dir);

    // Enable Qt Gui in C++ if the feature is enabled
    if feature_qt_gui_enabled {
        builder.define("CXX_QT_GUI_FEATURE", None);
    }

    // MSVC
    builder.flag_if_supported("/std:c++17");
    builder.flag_if_supported("/Zc:__cplusplus");
    builder.flag_if_supported("/permissive-");
    // GCC + Clang
    builder.flag_if_supported("-std=c++17");
    builder.compile("cxx-qt-lib");
}
