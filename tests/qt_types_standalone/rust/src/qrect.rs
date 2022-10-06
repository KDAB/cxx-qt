// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QRect;

#[cxx::bridge]
mod qrect_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qrect.h");

        type QRect = cxx_qt_lib::QRect;
    }

    extern "Rust" {
        fn construct_qrect() -> QRect;
        fn read_qrect(p: &QRect) -> bool;
        fn clone_qrect(p: &QRect) -> QRect;
    }
}

fn construct_qrect() -> QRect {
    QRect::new(1, 4, 2, 8)
}

fn read_qrect(r: &QRect) -> bool {
    r.x() == 1 && r.y() == 4 && r.width() == 2 && r.height() == 8
}

fn clone_qrect(r: &QRect) -> QRect {
    r.clone()
}
