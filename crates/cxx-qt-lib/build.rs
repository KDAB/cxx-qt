// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_build::CxxQtBuilder;
use std::path::PathBuf;
fn qt_gui_enabled() -> bool {
    std::env::var("CARGO_FEATURE_QT_GUI").is_ok()
}

fn qt_qml_enabled() -> bool {
    std::env::var("CARGO_FEATURE_QT_QML").is_ok()
}

fn qt_quickcontrols_enabled() -> bool {
    std::env::var("CARGO_FEATURE_QT_QUICKCONTROLS").is_ok()
}

fn header_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("include")
        .join("cxx-qt-lib")
}

fn write_definitions_header() {
    // We cannot ensure that downstream dependencies set the same compile-time definitions.
    // So we generate a header file that adds those definitions, which will be passed along
    // to downstream dependencies with all other headers.
    //
    // Thanks to David Faure for reminding us of this useful trick in his blog post:
    // https://www.kdab.com/setting-defines-with-cmake/
    let mut definitions = "#pragma once\n".to_owned();

    if qt_gui_enabled() {
        definitions.push_str("#define CXX_QT_GUI_FEATURE\n");
    }

    if qt_qml_enabled() {
        definitions.push_str("#define CXX_QT_QML_FEATURE\n");
    }

    if qt_quickcontrols_enabled() {
        definitions.push_str("#define CXX_QT_QUICKCONTROLS_FEATURE\n");
    }

    std::fs::create_dir_all(header_dir()).expect("Failed to create cxx-qt-lib include directory");
    std::fs::write(header_dir().join("definitions.h"), definitions)
        .expect("Failed to write cxx-qt-lib/definitions.h");
}

fn main() {
    write_definitions_header();

    let qtbuild = qt_build_utils::QtBuild::new(vec!["Core".to_owned()])
        .expect("Could not find Qt installation");

    let emscripten_targeted = match std::env::var("CARGO_CFG_TARGET_OS") {
        Ok(val) => val == "emscripten",
        Err(_) => false,
    };

    let mut rust_bridges = vec![
        "core/qbytearray",
        "core/qcoreapplication",
        "core/qdate",
        "core/qhash/qhash_i32_qbytearray",
        "core/qhash/qhash_qstring_qvariant",
        "core/qline",
        "core/qlinef",
        "core/qlist/qlist_bool",
        "core/qlist/qlist_f32",
        "core/qlist/qlist_f64",
        "core/qlist/qlist_i8",
        "core/qlist/qlist_i16",
        "core/qlist/qlist_i32",
        "core/qlist/qlist_i64",
        "core/qlist/qlist_qbytearray",
        "core/qlist/qlist_qdate",
        "core/qlist/qlist_qline",
        "core/qlist/qlist_qlinef",
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
        "core/qlist/qlist_quuid",
        "core/qlist/qlist_qvariant",
        "core/qlist/qlist_u8",
        "core/qlist/qlist_u16",
        "core/qlist/qlist_u32",
        "core/qlist/qlist_u64",
        "core/qmap/qmap_qstring_qvariant",
        "core/qmargins",
        "core/qmarginsf",
        "core/qmodelindex",
        "core/qobject",
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
        "core/qset/qset_qpersistentmodelindex",
        "core/qset/qset_qstring",
        "core/qset/qset_qtime",
        "core/qset/qset_qurl",
        "core/qset/qset_quuid",
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
        "core/qtlogging",
        "core/qtypes",
        "core/qurl",
        "core/quuid",
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
        "core/qvariant/qvariant_quuid",
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
        "core/qvector/qvector_qline",
        "core/qvector/qvector_qlinef",
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
        "core/qvector/qvector_quuid",
        "core/qvector/qvector_qvariant",
        "core/qvector/qvector_u8",
        "core/qvector/qvector_u16",
        "core/qvector/qvector_u32",
        "core/qvector/qvector_u64",
    ];

    if qtbuild.version().major > 5 {
        rust_bridges.extend(["core/qanystringview"]);
    }

    if qt_gui_enabled() {
        rust_bridges.extend([
            "core/qlist/qlist_qcolor",
            "core/qvariant/qvariant_qcolor",
            "core/qvariant/qvariant_qquaternion",
            "core/qvariant/qvariant_qvector2d",
            "core/qvariant/qvariant_qvector3d",
            "core/qvariant/qvariant_qvector4d",
            "core/qvector/qvector_qcolor",
            "gui/qcolor",
            "gui/qfont",
            "gui/qguiapplication",
            "gui/qimage",
            "gui/qquaternion",
            "gui/qpainterpath",
            "gui/qpainter",
            "gui/qpen",
            "gui/qpolygon",
            "gui/qpolygonf",
            "gui/qregion",
            "gui/qvector2d",
            "gui/qvector3d",
            "gui/qvector4d",
        ]);
    }

    if qt_qml_enabled() {
        rust_bridges.extend(["qml/qqmlapplicationengine", "qml/qqmlengine"]);
    }

    if qt_quickcontrols_enabled() {
        rust_bridges.extend(["quickcontrols/qquickstyle"]);
    }

    if !emscripten_targeted {
        rust_bridges.extend([
            "core/qdatetime",
            "core/qtimezone",
            "core/qlist/qlist_qdatetime",
            "core/qset/qset_qdatetime",
            "core/qvariant/qvariant_qdatetime",
            "core/qvector/qvector_qdatetime",
        ]);
    }

    let mut cpp_files = vec![
        "core/qbytearray",
        "core/qcoreapplication",
        "core/qdate",
        "core/qhash/qhash",
        "core/qline",
        "core/qlinef",
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
        "core/qtlogging",
        "core/qtypes",
        "core/qurl",
        "core/quuid",
        "core/qvariant/qvariant",
        "core/qvector/qvector",
    ];

    if qtbuild.version().major > 5 {
        cpp_files.extend(["core/qanystringview"]);
    }

    if qt_gui_enabled() {
        cpp_files.extend([
            "gui/qcolor",
            "gui/qfont",
            "gui/qgenericmatrix",
            "gui/qguiapplication",
            "gui/qimage",
            "gui/qpainterpath",
            "gui/qpainter",
            "gui/qpen",
            "gui/qpolygon",
            "gui/qpolygonf",
            "gui/qquaternion",
            "gui/qregion",
            "gui/qvector2d",
            "gui/qvector3d",
            "gui/qvector4d",
        ]);
    }

    if qt_qml_enabled() {
        cpp_files.extend(["qml/qqmlapplicationengine", "qml/qqmlengine"]);
    }

    if !emscripten_targeted {
        cpp_files.extend(["core/qdatetime", "core/qtimezone"]);
    }

    let mut builder = CxxQtBuilder::new()
        // Use a short name due to the Windows file path limit!
        // We don't re-export these headers anyway
        .include_prefix("private")
        .crate_include_root(Some("include/".to_owned()))
        .include_dir(header_dir())
        .initializer(qt_build_utils::Initializer {
            file: Some("src/core/init.cpp".into()),
            ..qt_build_utils::Initializer::default_signature("init_cxx_qt_lib_core")
        });

    if qt_gui_enabled() {
        builder = builder
            .qt_module("Gui")
            .initializer(qt_build_utils::Initializer {
                file: Some("src/gui/init.cpp".into()),
                ..qt_build_utils::Initializer::default_signature("init_cxx_qt_lib_gui")
            });
    }

    if qt_qml_enabled() {
        builder = builder.qt_module("Qml");
    }

    if qt_quickcontrols_enabled() {
        builder = builder.qt_module("QuickControls2");
    }

    for rust_source in &rust_bridges {
        builder = builder.file(format!("src/{rust_source}.rs"));
    }

    for cpp_file in &cpp_files {
        builder = builder.cpp_file(format!("src/{cpp_file}.cpp"));
    }
    builder = builder.cpp_file("src/qt_types.cpp");

    let interface = builder.build();
    interface.reexport_dependency("cxx-qt").export();
}
