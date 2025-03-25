// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QDate, QDateTime, QTime, QTimeZone};

#[cxx::bridge]
mod qdatetime_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qdate.h");
        include!("cxx-qt-lib/qdatetime.h");
        include!("cxx-qt-lib/qtime.h");
        include!("cxx-qt-lib/qtimezone.h");

        type QDate = cxx_qt_lib::QDate;
        type QDateTime = cxx_qt_lib::QDateTime;
        type QTime = cxx_qt_lib::QTime;
        type QTimeZone = cxx_qt_lib::QTimeZone;
    }

    extern "Rust" {
        fn construct_qdatetime(date: &QDate, time: &QTime, time_zone: &QTimeZone) -> QDateTime;
        fn read_qdatetime(c: &QDateTime, date: &QDate, time: &QTime) -> bool;
        fn clone_qdatetime(c: &QDateTime) -> QDateTime;
    }
}

fn construct_qdatetime(date: &QDate, time: &QTime, time_zone: &QTimeZone) -> QDateTime {
    QDateTime::from_qdate_qtime_qtimezone(date, time, time_zone)
}

fn read_qdatetime(dt: &QDateTime, date: &QDate, time: &QTime) -> bool {
    dt.date().year() == date.year()
        && dt.date().month() == date.month()
        && dt.date().day() == date.day()
        && dt.time().hour() == time.hour()
        && dt.time().minute() == time.minute()
        && dt.time().second() == time.second()
        && dt.time().msec() == time.msec()
        && dt.offset_from_utc() == 0
}

fn clone_qdatetime(dt: &QDateTime) -> QDateTime {
    dt.clone()
}
