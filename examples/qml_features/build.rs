// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_build_rs
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .file("src/custom_base.rs")
        .file("src/empty.rs")
        .file("src/lib.rs")
        .file("src/mock_qt_types.rs")
        .file("src/rust_obj_invokables.rs")
        .file("src/serialisation.rs")
        .file("src/signals.rs")
        .file("src/struct_properties.rs")
        .file("src/types.rs")
        .build();
}
// ANCHOR_END: book_build_rs
