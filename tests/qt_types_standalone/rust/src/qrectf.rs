// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QRectF;

#[cxx::bridge]
mod qrectf_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QRectF = cxx_qt_lib::QRectF;
    }

    extern "Rust" {
        fn construct_qrectf() -> QRectF;
        fn read_qrectf(p: &QRectF) -> bool;
        fn clone_qrectf(p: &QRectF) -> QRectF;
    }
}

fn construct_qrectf() -> QRectF {
    QRectF::new(1.23, 4.56, 2.46, 9.12)
}

fn read_qrectf(p: &QRectF) -> bool {
    ((p.x() - 1.23).abs() < f64::EPSILON)
        && ((p.y() - 4.56).abs() < f64::EPSILON)
        && ((p.width() - 2.46).abs() < f64::EPSILON)
        && ((p.height() - 9.12).abs() < f64::EPSILON)
}

fn clone_qrectf(p: &QRectF) -> QRectF {
    p.clone()
}
