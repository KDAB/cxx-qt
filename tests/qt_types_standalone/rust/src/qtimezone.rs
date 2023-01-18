// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QByteArray, QTimeZone};

#[cxx::bridge]
mod qtimezone_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qtimezone.h");
        type QTimeZone = cxx_qt_lib::QTimeZone;
    }

    extern "Rust" {
        fn construct_qtimezone() -> QTimeZone;
        fn read_qtimezone(t: &QTimeZone) -> bool;
        fn clone_qtimezone(t: &QTimeZone) -> QTimeZone;
    }
}

fn construct_qtimezone() -> QTimeZone {
    QTimeZone::from_iana(&QByteArray::from("Europe/London"))
}

fn read_qtimezone(t: &QTimeZone) -> bool {
    t.id().to_string() == "Europe/London"
}

fn clone_qtimezone(t: &QTimeZone) -> QTimeZone {
    t.clone()
}
