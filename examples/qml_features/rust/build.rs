// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_build_rs
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .file("src/containers.rs")
        .file("src/custom_base_class.rs")
        .file("src/invokables.rs")
        .file("src/multiple_qobjects.rs")
        .file("src/nested_qobjects.rs")
        .file("src/serialisation.rs")
        .file("src/signals.rs")
        .file("src/singleton.rs")
        .file("src/properties.rs")
        .file("src/threading.rs")
        .file("src/types.rs")
        // custom_object.cpp/h need to be handled here rather than CMakeLists.txt,
        // otherwise linking cargo tests fails because the symbols from those files are not found.
        .cc_builder(|cc| {
            cc.include("../cpp");
            cc.file("../cpp/custom_object.cpp");
        })
        .qobject_header("../cpp/custom_object.h")
        .build();
}
// ANCHOR_END: book_build_rs
