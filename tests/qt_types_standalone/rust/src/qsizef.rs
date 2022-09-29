// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QSizeF;

#[cxx::bridge]
mod qsizef_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QSizeF = cxx_qt_lib::QSizeF;
    }

    extern "Rust" {
        fn construct_qsizef() -> QSizeF;
        fn read_qsizef(p: &QSizeF) -> bool;
        fn clone_qsizef(p: &QSizeF) -> QSizeF;
    }
}

fn construct_qsizef() -> QSizeF {
    QSizeF::new(1.23, 4.56)
}

fn read_qsizef(s: &QSizeF) -> bool {
    ((s.width() - 1.23).abs() < f64::EPSILON) && ((s.height() - 4.56).abs() < f64::EPSILON)
}

fn clone_qsizef(s: &QSizeF) -> QSizeF {
    s.clone()
}
