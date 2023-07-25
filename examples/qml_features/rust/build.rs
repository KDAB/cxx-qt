// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_build_rs
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .qml_module(
            "com.kdab.cxx_qt.demo",
            1,
            0,
            &[
                "src/containers.rs",
                "src/custom_base_class.rs",
                "src/custom_parent_class.rs",
                "src/invokables.rs",
                "src/multiple_qobjects.rs",
                "src/nested_qobjects.rs",
                "src/serialisation.rs",
                "src/signals.rs",
                "src/singleton.rs",
                "src/properties.rs",
                "src/threading.rs",
                "src/types.rs",
                "src/uncreatable.rs",
            ],
        )
        // custom_object.cpp/h need to be handled here rather than CMakeLists.txt,
        // otherwise linking cargo tests fails because the symbols from those files are not found.
        .cc_builder(|cc| {
            cc.include("../cpp");
            cc.file("../cpp/custom_object.cpp");
        })
        .qobject_header("../cpp/custom_object.h")
        // Ensure that Quick module is linked, so that cargo test can work.
        // In a CMake project this isn't required as the linking happens in CMake.
        .qt_module("Quick")
        .build();
}
// ANCHOR_END: book_build_rs
