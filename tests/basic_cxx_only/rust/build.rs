// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .file("src/lib.rs")
        .cc_builder(|cc| {
            cc.include("../cpp");
            // cxx_test.cpp need to be compiled by cargo rather than CMakeLists.txt,
            // otherwise linking cargo tests fails because the symbols from those files are not found.
            // This to make cargo only tests work.
            cc.file("../cpp/cxx_test.cpp");
        })
        .build();
}
