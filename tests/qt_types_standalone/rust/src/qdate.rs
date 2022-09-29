// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QDate;

#[cxx::bridge]
mod qdate_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QDate = cxx_qt_lib::QDate;
    }

    extern "Rust" {
        fn construct_qdate() -> QDate;
        fn read_qdate(d: &QDate) -> bool;
        fn clone_qdate(d: &QDate) -> QDate;
    }
}

fn construct_qdate() -> QDate {
    QDate::new(2022, 1, 1)
}

fn read_qdate(d: &QDate) -> bool {
    d.year() == 2022 && d.month() == 1 && d.day() == 1
}

fn clone_qdate(d: &QDate) -> QDate {
    d.clone()
}
