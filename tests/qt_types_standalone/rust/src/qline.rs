// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QLine, QPoint};

#[cxx::bridge]
mod qline_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qline.h");
        type QLine = cxx_qt_lib::QLine;
    }

    extern "Rust" {
        fn construct_qline() -> QLine;
        fn read_qline(m: &QLine) -> bool;
        fn clone_qline(m: &QLine) -> QLine;
    }
}

fn construct_qline() -> QLine {
    QLine::new(QPoint::new(1, 2), QPoint::new(3, 4))
}

fn read_qline(m: &QLine) -> bool {
    m.x1() == 1 && m.y1() == 2 && m.x2() == 3 && m.y2() == 4
}

fn clone_qline(m: &QLine) -> QLine {
    m.clone()
}
