// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QMargins;

#[cxx::bridge]
mod qmargins_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmargins.h");
        type QMargins = cxx_qt_lib::QMargins;
    }

    extern "Rust" {
        fn construct_qmargins() -> QMargins;
        fn read_qmargins(m: &QMargins) -> bool;
        fn clone_qmargins(m: &QMargins) -> QMargins;
        fn test_is_null(m: &QMargins) -> bool;
        fn add_margins(m1: QMargins, m2: QMargins) -> QMargins;
    }
}

fn construct_qmargins() -> QMargins {
    QMargins::new(1, 2, 3, 4)
}

fn read_qmargins(m: &QMargins) -> bool {
    m.left() == 1 && m.top() == 2 && m.right() == 3 && m.bottom() == 4
}

fn clone_qmargins(m: &QMargins) -> QMargins {
    m.clone()
}

fn test_is_null(m: &QMargins) -> bool {
    m.is_null()
}

fn add_margins(m1: QMargins, m2: QMargins) -> QMargins {
   m1 + m2
}
