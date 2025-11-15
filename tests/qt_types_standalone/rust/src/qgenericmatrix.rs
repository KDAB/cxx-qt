// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QMatrix2x3;

#[cxx::bridge]
mod qset_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qgenericmatrix.h");
        type QMatrix2x3 = cxx_qt_lib::QMatrix2x3;
    }

    extern "Rust" {
        fn construct_qmatrix_ascending() -> QMatrix2x3;

        fn set_qmatrix_value(matrix: &mut QMatrix2x3, row: i32, col: i32, value: f32);
    }
}

fn construct_qmatrix_ascending() -> QMatrix2x3 {
    QMatrix2x3::new(&[[0.0, 1.0], [2.0, 3.0], [4.0, 5.0]])
}

fn set_qmatrix_value(matrix: &mut QMatrix2x3, row: i32, col: i32, value: f32) {
    matrix[(row as usize, col as usize)] = value;
}
