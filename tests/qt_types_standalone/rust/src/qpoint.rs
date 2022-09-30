// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QPoint;

#[cxx::bridge]
mod qpoint_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qpoint.h");

        type QPoint = cxx_qt_lib::QPoint;
    }

    extern "Rust" {
        fn construct_qpoint() -> QPoint;
        fn read_qpoint(p: &QPoint) -> bool;
        fn clone_qpoint(p: &QPoint) -> QPoint;
    }
}

fn construct_qpoint() -> QPoint {
    QPoint::new(2, 4)
}

fn read_qpoint(p: &QPoint) -> bool {
    p.x() == 2 && p.y() == 4
}

fn clone_qpoint(p: &QPoint) -> QPoint {
    p.clone()
}
