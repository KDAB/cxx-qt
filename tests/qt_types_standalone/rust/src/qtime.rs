// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QTime;

#[cxx::bridge]
mod qtime_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qtime.h");

        type QTime = cxx_qt_lib::QTime;
    }

    extern "Rust" {
        fn construct_qtime() -> QTime;
        fn read_qtime(p: &QTime) -> bool;
        fn clone_qtime(p: &QTime) -> QTime;
    }
}

fn construct_qtime() -> QTime {
    QTime::new(1, 2, 3, 4)
}

fn read_qtime(s: &QTime) -> bool {
    s.hour() == 1 && s.minute() == 2 && s.second() == 3 && s.msec() == 4
}

fn clone_qtime(s: &QTime) -> QTime {
    s.clone()
}
