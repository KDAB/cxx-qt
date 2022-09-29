// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QSize;

#[cxx::bridge]
mod qsize_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QSize = cxx_qt_lib::QSize;
    }

    extern "Rust" {
        fn construct_qsize() -> QSize;
        fn read_qsize(p: &QSize) -> bool;
        fn clone_qsize(p: &QSize) -> QSize;
    }
}

fn construct_qsize() -> QSize {
    QSize::new(1, 4)
}

fn read_qsize(s: &QSize) -> bool {
    s.width() == 1 && s.height() == 4
}

fn clone_qsize(s: &QSize) -> QSize {
    s.clone()
}
