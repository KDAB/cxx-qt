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

fn write_headers_in(subfolder: &str) {
    println!("cargo::rerun-if-changed=include/{subfolder}");

    for entry in
        std::fs::read_dir(format!("include/{subfolder}")).expect("Failed to read include directory")
    {
        let entry = entry.expect("Failed to read header file!");
        let header_name = entry.file_name();
        println!(
            "cargo::rerun-if-changed=include/{subfolder}/{header_name}",
            header_name = header_name.to_string_lossy()
        );

        // TODO: Do we want to add the headers into a subdirectory?
        std::fs::copy(entry.path(), header_dir().join(header_name))
            .expect("Failed to copy header file!");
    }
}

fn write_headers() {
    println!("cargo::rerun-if-changed=include/");
    std::fs::create_dir_all(header_dir()).expect("Failed to create include directory");
    println!("cargo::rerun-if-changed=include/common.h");
    std::fs::copy("include/common.h", header_dir().join("common.h"))
        .expect("Failed to copy header file!");

    write_headers_in("core");
    if qt_gui_enabled() {
        write_headers_in("gui");
    }
    if qt_qml_enabled() {
        write_headers_in("qml");
    }
    if qt_quickcontrols_enabled() {
        write_headers_in("quickcontrols");
    }
}

fn main() {
    write_headers();

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

    if qt_gui_enabled() {
        rust_bridges.extend([
            "core/qlist/qlist_qcolor",
            "core/qvariant/qvariant_qcolor",
            "core/qvector/qvector_qcolor",
            "gui/qcolor",
            "gui/qfont",
            "gui/qguiapplication",
            "gui/qimage",
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
        "core/qurl",
        "core/qvariant/qvariant",
        "core/qvector/qvector",
    ];

    if qt_gui_enabled() {
        cpp_files.extend([
            "gui/qcolor",
            "gui/qfont",
            "gui/qguiapplication",
            "gui/qimage",
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
        cpp_files.extend(["qml/qqmlapplicationengine", "qml/qqmlengine"]);
    }

    if qt_quickcontrols_enabled() {
        cpp_files.extend(["quickcontrols/qquickstyle"]);
    }

    if !emscripten_targeted {
        cpp_files.extend(["core/qdatetime", "core/qtimezone"]);
    }

    let mut interface = cxx_qt_build::Interface::default()
        .initializer("src/core/init.cpp")
        .export_include_prefixes([])
        .export_include_directory(header_dir(), "cxx-qt-lib");

    if qt_gui_enabled() {
        interface = interface
            .define("CXX_QT_GUI_FEATURE", None)
            .initializer("src/gui/init.cpp");
    }

    if qt_qml_enabled() {
        interface = interface.define("CXX_QT_QML_FEATURE", None);
    }

    if qt_quickcontrols_enabled() {
        interface = interface.define("CXX_QT_QUICKCONTROLS_FEATURE", None);
    }

    let mut builder = CxxQtBuilder::library(interface).include_prefix("cxx-qt-lib-internals");

    if qt_gui_enabled() {
        builder = builder.qt_module("Gui");
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

    builder = builder.cc_builder(move |cc| {
        for cpp_file in &cpp_files {
            cc.file(format!("src/{cpp_file}.cpp"));
            println!("cargo:rerun-if-changed=src/{cpp_file}.cpp");
        }
        cc.file("src/qt_types.cpp");
        println!("cargo:rerun-if-changed=src/qt_types.cpp");
    });
    println!("cargo:rerun-if-changed=src/assertion_utils.h");

    builder.build();
}
