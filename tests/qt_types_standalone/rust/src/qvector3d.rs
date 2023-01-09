// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QVector3D;

#[cxx::bridge]
mod qvector3d_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvector3d.h");
        type QVector3D = cxx_qt_lib::QVector3D;
    }

    extern "Rust" {
        fn construct_qvector3d() -> QVector3D;
        fn read_qvector3d(p: &QVector3D) -> bool;
        fn clone_qvector3d(p: &QVector3D) -> QVector3D;
    }
}

fn construct_qvector3d() -> QVector3D {
    QVector3D::new(1.23, 4.56, 7.89)
}

fn read_qvector3d(v: &QVector3D) -> bool {
    ((v.x() - 1.23).abs() < f32::EPSILON)
        && ((v.y() - 4.56).abs() < f32::EPSILON)
        && ((v.z() - 7.89).abs() < f32::EPSILON)
}

fn clone_qvector3d(v: &QVector3D) -> QVector3D {
    v.clone()
}
