// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .file("src/empty.rs")
        .file("src/data.rs")
        .file("src/lib.rs")
        .file("src/locking.rs")
        .file("src/types.rs")
        .with_opts(cxx_qt_lib_headers::build_opts())
        .build();
}
