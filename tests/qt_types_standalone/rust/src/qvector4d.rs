// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QVector4D;

#[cxx::bridge]
mod qvector4d_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvector4d.h");
        type QVector4D = cxx_qt_lib::QVector4D;
    }

    extern "Rust" {
        fn construct_qvector4d() -> QVector4D;
        fn read_qvector4d(p: &QVector4D) -> bool;
        fn clone_qvector4d(p: &QVector4D) -> QVector4D;
    }
}

fn construct_qvector4d() -> QVector4D {
    QVector4D::new(1.23, 4.56, 7.89, 1.47)
}

fn read_qvector4d(v: &QVector4D) -> bool {
    ((v.x() - 1.23).abs() < f32::EPSILON)
        && ((v.y() - 4.56).abs() < f32::EPSILON)
        && ((v.z() - 7.89).abs() < f32::EPSILON)
        && ((v.w() - 1.47).abs() < f32::EPSILON)
}

fn clone_qvector4d(v: &QVector4D) -> QVector4D {
    v.clone()
}
