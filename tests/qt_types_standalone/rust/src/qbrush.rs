// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QBrush;
use cxx_qt_lib::QColor;

#[cxx::bridge]
mod qpoint_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbrush.h");
        type QBrush = cxx_qt_lib::QBrush;
    }

    extern "Rust" {
        fn construct_qbrush() -> QBrush;
        fn clone_qbrush(p: &QBrush) -> QBrush;
    }
}

fn construct_qbrush() -> QBrush {
    let mut brush = QBrush::default();
    brush.set_color(&QColor::from_rgb(255, 0, 0));
    brush
}

fn clone_qbrush(p: &QBrush) -> QBrush {
    p.clone()
}
