// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QPolygonF;

#[cxx::bridge]
mod qpolygonf_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpolygonf.h");

        type QPolygonF = cxx_qt_lib::QPolygonF;
    }

    extern "Rust" {
        fn clone_qpolygonf(p: &QPolygonF) -> QPolygonF;
        fn construct_qpolygonf() -> QPolygonF;
    }
}

fn construct_qpolygonf() -> QPolygonF {
    QPolygonF::default()
}

fn clone_qpolygonf(p: &QPolygonF) -> QPolygonF {
    p.clone()
}
