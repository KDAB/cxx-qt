// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_build_rs
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .cc_builder(|cc| {
            cc.include("../cpp");
        })
        .file("src/custom_base_class.rs")
        .file("src/invokables.rs")
        .file("src/multiple_qobjects.rs")
        .file("src/serialisation.rs")
        .file("src/signals.rs")
        .file("src/properties.rs")
        .file("src/threading.rs")
        .file("src/types.rs")
        .build();
}
// ANCHOR_END: book_build_rs
