// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QPainterPath;

#[cxx::bridge]
mod qpainterpath_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpainterpath.h");

        type QPainterPath = cxx_qt_lib::QPainterPath;
    }

    extern "Rust" {
        fn construct_qpainterpath() -> QPainterPath;
        fn clone_qpainterpath(p: &QPainterPath) -> QPainterPath;
    }
}

fn construct_qpainterpath() -> QPainterPath {
    QPainterPath::default()
}

fn clone_qpainterpath(p: &QPainterPath) -> QPainterPath {
    p.clone()
}
