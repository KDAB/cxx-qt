// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QPointF;

#[cxx::bridge]
mod qpointf_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QPointF = cxx_qt_lib::QPointF;
    }

    extern "Rust" {
        fn construct_qpointf() -> QPointF;
        fn read_qpointf(p: &QPointF) -> bool;
        fn clone_qpointf(p: &QPointF) -> QPointF;
    }
}

fn construct_qpointf() -> QPointF {
    QPointF::new(1.23, 4.56)
}

fn read_qpointf(p: &QPointF) -> bool {
    ((p.x() - 1.23).abs() < f64::EPSILON) && ((p.y() - 4.56).abs() < f64::EPSILON)
}

fn clone_qpointf(p: &QPointF) -> QPointF {
    p.clone()
}
