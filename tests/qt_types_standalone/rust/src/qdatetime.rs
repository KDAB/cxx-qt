// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QDate, QDateTime, QTime};

#[cxx::bridge]
mod qdatetime_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qdate.h");
        include!("cxx-qt-lib/include/qdatetime.h");
        include!("cxx-qt-lib/include/qtime.h");

        type QDate = cxx_qt_lib::QDate;
        type QDateTime = cxx_qt_lib::QDateTime;
        type QTime = cxx_qt_lib::QTime;
    }

    extern "Rust" {
        fn construct_qdatetime(date: &QDate, time: &QTime) -> QDateTime;
        fn read_qdatetime(c: &QDateTime, date: &QDate, time: &QTime) -> bool;
        fn clone_qdatetime(c: &QDateTime) -> QDateTime;
    }
}

fn construct_qdatetime(date: &QDate, time: &QTime) -> QDateTime {
    QDateTime::from_date_and_time(date, time)
}

fn read_qdatetime(dt: &cxx_qt_lib::QDateTime, date: &QDate, time: &QTime) -> bool {
    dt.date().year() == date.year()
        && dt.date().month() == date.month()
        && dt.date().day() == date.day()
        && dt.time().hour() == time.hour()
        && dt.time().minute() == time.minute()
        && dt.time().second() == time.second()
        && dt.time().msec() == time.msec()
}

fn clone_qdatetime(dt: &QDateTime) -> QDateTime {
    dt.clone()
}
