// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QMarginsF;

#[cxx::bridge]
mod qmarginsf_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmarginsf.h");
        type QMarginsF = cxx_qt_lib::QMarginsF;
    }

    extern "Rust" {
        fn construct_qmarginsf() -> QMarginsF;
        fn read_qmarginsf(m: &QMarginsF) -> bool;
        fn clone_qmarginsf(m: &QMarginsF) -> QMarginsF;
    }
}

fn construct_qmarginsf() -> QMarginsF {
    QMarginsF::new(1.0, 2.0, 3.0, 4.0)
}

fn read_qmarginsf(m: &QMarginsF) -> bool {
    m.left() == 1.0 && m.top() == 2.0 && m.right() == 3.0 && m.bottom() == 4.0
}

fn clone_qmarginsf(m: &QMarginsF) -> QMarginsF {
    m.clone()
}
