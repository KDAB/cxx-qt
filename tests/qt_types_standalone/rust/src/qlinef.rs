// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QLineF, QPointF};

#[cxx::bridge]
mod qline_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlinef.h");
        type QLineF = cxx_qt_lib::QLineF;
    }

    extern "Rust" {
        fn construct_qlinef() -> QLineF;
        fn read_qlinef(m: &QLineF) -> bool;
        fn clone_qlinef(m: &QLineF) -> QLineF;
    }
}

fn construct_qlinef() -> QLineF {
    QLineF::new(QPointF::new(1.0, 2.0), QPointF::new(3.0, 4.0))
}

fn read_qlinef(m: &QLineF) -> bool {
    m.x1() == 1.0 && m.y1() == 2.0 && m.x2() == 3.0 && m.y2() == 4.0
}

fn clone_qlinef(m: &QLineF) -> QLineF {
    m.clone()
}
