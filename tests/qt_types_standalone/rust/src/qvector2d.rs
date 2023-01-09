// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QVector2D;

#[cxx::bridge]
mod qvector2d_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvector2d.h");
        type QVector2D = cxx_qt_lib::QVector2D;
    }

    extern "Rust" {
        fn construct_qvector2d() -> QVector2D;
        fn read_qvector2d(p: &QVector2D) -> bool;
        fn clone_qvector2d(p: &QVector2D) -> QVector2D;
    }
}

fn construct_qvector2d() -> QVector2D {
    QVector2D::new(1.23, 4.56)
}

fn read_qvector2d(v: &QVector2D) -> bool {
    ((v.x() - 1.23).abs() < f32::EPSILON) && ((v.y() - 4.56).abs() < f32::EPSILON)
}

fn clone_qvector2d(v: &QVector2D) -> QVector2D {
    v.clone()
}
