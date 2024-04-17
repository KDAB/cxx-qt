// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QPen;

#[cxx::bridge]
mod qpen_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpen.h");

        type QPen = cxx_qt_lib::QPen;
    }

    extern "Rust" {
        fn construct_qpen() -> QPen;
        fn clone_qpen(p: &QPen) -> QPen;
    }
}

fn construct_qpen() -> QPen {
    QPen::default()
}

fn clone_qpen(p: &QPen) -> QPen {
    p.clone()
}
