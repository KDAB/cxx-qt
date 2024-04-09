// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QRegion;

#[cxx::bridge]
mod qregion_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qregion.h");

        type QRegion = cxx_qt_lib::QRegion;
    }

    extern "Rust" {
        fn construct_qregion() -> QRegion;
        fn clone_qregion(p: &QRegion) -> QRegion;
    }
}

fn construct_qregion() -> QRegion {
    QRegion::default()
}

fn clone_qregion(p: &QRegion) -> QRegion {
    p.clone()
}
