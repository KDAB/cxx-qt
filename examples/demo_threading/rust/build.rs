// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .qml_module("com.kdab.energy", 1, 0, &["src/lib.rs"])
        .build();
}
