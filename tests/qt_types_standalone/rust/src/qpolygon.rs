// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QPolygon;

#[cxx::bridge]
mod qpolygon_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpolygon.h");

        type QPolygon = cxx_qt_lib::QPolygon;
    }

    extern "Rust" {
        fn clone_qpolygon(p: &QPolygon) -> QPolygon;
        fn construct_qpolygon() -> QPolygon;
    }
}

fn construct_qpolygon() -> QPolygon {
    QPolygon::default()
}

fn clone_qpolygon(p: &QPolygon) -> QPolygon {
    p.clone()
}
