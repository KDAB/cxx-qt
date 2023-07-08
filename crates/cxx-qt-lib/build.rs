// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

fn main() {
    let feature_qt_gui_enabled = std::env::var("CARGO_FEATURE_QT_GUI").is_ok();
    let feature_qt_qml_enabled = std::env::var("CARGO_FEATURE_QT_QML").is_ok();

    let mut qt_modules = vec!["Core".to_owned()];
    if feature_qt_gui_enabled {
        qt_modules.push("Gui".to_owned());
    }
    if feature_qt_qml_enabled {
        qt_modules.push("Qml".to_owned());
    }

    let mut rust_bridges = vec![
        "core/qbytearray",
        "core/qcoreapplication",
        "core/qdate",
        "core/qdatetime",
        "core/qhash/qhash_i32_qbytearray",
        "core/qhash/qhash_qstring_qvariant",
        "core/qlist/qlist_bool",
        "core/qlist/qlist_f32",
        "core/qlist/qlist_f64",
        "core/qlist/qlist_i8",
        "core/qlist/qlist_i16",
        "core/qlist/qlist_i32",
        "core/qlist/qlist_i64",
        "core/qlist/qlist_qbytearray",
        "core/qlist/qlist_qdate",
        "core/qlist/qlist_qdatetime",
        "core/qlist/qlist_qmargins",
        "core/qlist/qlist_qmarginsf",
        "core/qlist/qlist_qpersistentmodelindex",
        "core/qlist/qlist_qpoint",
        "core/qlist/qlist_qpointf",
        "core/qlist/qlist_qrect",
        "core/qlist/qlist_qrectf",
        "core/qlist/qlist_qsize",
        "core/qlist/qlist_qsizef",
        "core/qlist/qlist_qstring",
        "core/qlist/qlist_qtime",
        "core/qlist/qlist_qurl",
        "core/qlist/qlist_qvariant",
        "core/qlist/qlist_u8",
        "core/qlist/qlist_u16",
        "core/qlist/qlist_u32",
        "core/qlist/qlist_u64",
        "core/qmap/qmap_qstring_qvariant",
        "core/qmargins",
        "core/qmarginsf",
        "core/qmodelindex",
        "core/qpersistentmodelindex",
        "core/qpoint",
        "core/qpointf",
        "core/qrect",
        "core/qrectf",
        "core/qset/qset_bool",
        "core/qset/qset_f32",
        "core/qset/qset_f64",
        "core/qset/qset_i8",
        "core/qset/qset_i16",
        "core/qset/qset_i32",
        "core/qset/qset_i64",
        "core/qset/qset_qbytearray",
        "core/qset/qset_qdate",
        "core/qset/qset_qdatetime",
        "core/qset/qset_qpersistentmodelindex",
        "core/qset/qset_qstring",
        "core/qset/qset_qtime",
        "core/qset/qset_qurl",
        "core/qset/qset_u8",
        "core/qset/qset_u16",
        "core/qset/qset_u32",
        "core/qset/qset_u64",
        "core/qsize",
        "core/qsizef",
        "core/qstring",
        "core/qstringlist",
        "core/qt",
        "core/qtime",
        "core/qtimezone",
        "core/qurl",
        "core/qvariant/mod",
        "core/qvariant/qvariant_bool",
        "core/qvariant/qvariant_f32",
        "core/qvariant/qvariant_f64",
        "core/qvariant/qvariant_i8",
        "core/qvariant/qvariant_i16",
        "core/qvariant/qvariant_i32",
        "core/qvariant/qvariant_i64",
        "core/qvariant/qvariant_qbytearray",
        "core/qvariant/qvariant_qdate",
        "core/qvariant/qvariant_qdatetime",
        "core/qvariant/qvariant_qmodelindex",
        "core/qvariant/qvariant_qpersistentmodelindex",
        "core/qvariant/qvariant_qpoint",
        "core/qvariant/qvariant_qpointf",
        "core/qvariant/qvariant_qrect",
        "core/qvariant/qvariant_qrectf",
        "core/qvariant/qvariant_qsize",
        "core/qvariant/qvariant_qsizef",
        "core/qvariant/qvariant_qstring",
        "core/qvariant/qvariant_qstringlist",
        "core/qvariant/qvariant_qtime",
        "core/qvariant/qvariant_qurl",
        "core/qvariant/qvariant_u8",
        "core/qvariant/qvariant_u16",
        "core/qvariant/qvariant_u32",
        "core/qvariant/qvariant_u64",
        "core/qvector/qvector_bool",
        "core/qvector/qvector_f32",
        "core/qvector/qvector_f64",
        "core/qvector/qvector_i8",
        "core/qvector/qvector_i16",
        "core/qvector/qvector_i32",
        "core/qvector/qvector_i64",
        "core/qvector/qvector_qbytearray",
        "core/qvector/qvector_qdate",
        "core/qvector/qvector_qdatetime",
        "core/qvector/qvector_qmargins",
        "core/qvector/qvector_qmarginsf",
        "core/qvector/qvector_qpersistentmodelindex",
        "core/qvector/qvector_qpoint",
        "core/qvector/qvector_qpointf",
        "core/qvector/qvector_qrect",
        "core/qvector/qvector_qrectf",
        "core/qvector/qvector_qsize",
        "core/qvector/qvector_qsizef",
        "core/qvector/qvector_qstring",
        "core/qvector/qvector_qtime",
        "core/qvector/qvector_qurl",
        "core/qvector/qvector_qvariant",
        "core/qvector/qvector_u8",
        "core/qvector/qvector_u16",
        "core/qvector/qvector_u32",
        "core/qvector/qvector_u64",
    ];

    if feature_qt_gui_enabled {
        rust_bridges.extend([
            "core/qlist/qlist_qcolor",
            "core/qvariant/qvariant_qcolor",
            "core/qvector/qvector_qcolor",
            "gui/qcolor",
            "gui/qguiapplication",
            "gui/qvector2d",
            "gui/qvector3d",
            "gui/qvector4d",
        ]);
    }

    if feature_qt_qml_enabled {
        rust_bridges.extend(["qml/qqmlapplicationengine", "qml/qqmlengine"]);
    }

    for bridge in &rust_bridges {
        println!("cargo:rerun-if-changed=src/{bridge}.rs");
    }

    let qtbuild = qt_build_utils::QtBuild::new(qt_modules).expect("Could not find Qt installation");

    for include_path in qtbuild.include_paths() {
        cxx_build::CFG
            .exported_header_dirs
            .push(include_path.as_path());
    }

    let mut builder =
        cxx_build::bridges(rust_bridges.iter().map(|bridge| format!("src/{bridge}.rs")));

    qtbuild.cargo_link_libraries(None);
    // Required for tests
    qt_build_utils::setup_linker();

    // Find the Qt version and tell the Rust compiler
    // this allows us to have conditional Rust code
    println!(
        "cargo:rustc-cfg=qt_version_major=\"{}\"",
        qtbuild.version().major
    );

    let mut cpp_files = vec![
        "core/qbytearray",
        "core/qcoreapplication",
        "core/qdate",
        "core/qdatetime",
        "core/qhash/qhash",
        "core/qlist/qlist",
        "core/qmap/qmap",
        "core/qmargins",
        "core/qmarginsf",
        "core/qmodelindex",
        "core/qpersistentmodelindex",
        "core/qpoint",
        "core/qpointf",
        "core/qrect",
        "core/qrectf",
        "core/qset/qset",
        "core/qsize",
        "core/qsizef",
        "core/qstring",
        "core/qstringlist",
        "core/qtime",
        "core/qtimezone",
        "core/qurl",
        "core/qvariant/qvariant",
        "core/qvector/qvector",
    ];

    if feature_qt_gui_enabled {
        cpp_files.extend([
            "gui/qcolor",
            "gui/qguiapplication",
            "gui/qvector2d",
            "gui/qvector3d",
            "gui/qvector4d",
        ]);
    }

    if feature_qt_qml_enabled {
        cpp_files.extend(["qml/qqmlapplicationengine", "qml/qqmlengine"]);
    }

    for cpp_file in &cpp_files {
        builder.file(format!("src/{cpp_file}.cpp"));
        println!("cargo:rerun-if-changed=src/{cpp_file}.cpp");
    }
    builder.file("src/qt_types.cpp");
    println!("cargo:rerun-if-changed=src/qt_types.cpp");
    builder.file("src/std_types.cpp");
    println!("cargo:rerun-if-changed=src/std_types.cpp");
    println!("cargo:rerun-if-changed=src/assertion_utils.h");

    // Write this library's manually written C++ headers to files and add them to include paths
    let out_dir = std::env::var("OUT_DIR").unwrap();
    cxx_qt_lib_headers::write_headers(format!("{out_dir}/cxx-qt-lib"));
    builder.include(out_dir);

    // Enable Qt Gui in C++ if the feature is enabled
    if feature_qt_gui_enabled {
        builder.define("CXX_QT_GUI_FEATURE", None);
    }

    // Enable Qt Qml in C++ if the feature is enabled
    if feature_qt_gui_enabled {
        builder.define("CXX_QT_QML_FEATURE", None);
    }

    // MSVC
    builder.flag_if_supported("/std:c++17");
    builder.flag_if_supported("/Zc:__cplusplus");
    builder.flag_if_supported("/permissive-");
    // GCC + Clang
    builder.flag_if_supported("-std=c++17");
    builder.compile("cxx-qt-lib");
}
