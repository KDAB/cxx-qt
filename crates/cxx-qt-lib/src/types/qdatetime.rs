// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{QDate, QTime};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QDate = crate::QDate;
        type QDateTime;
        type QTime = crate::QTime;

        fn date(self: &QDateTime) -> QDate;
        fn time(self: &QDateTime) -> QTime;

        // Note that Qt 5 takes const-ref and Qt 6 takes by-value
        // for QDateTime::setDate and QDateTime::setTime
        //
        // So we need our own methods otherwise CXX can't match the method types
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qdatetime_set_date"]
        fn qdatetimeSetDate(datetime: Pin<&mut QDateTime>, date: QDate);
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qdatetime_set_time"]
        fn qdatetimeSetTime(datetime: Pin<&mut QDateTime>, time: QTime);

        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qdatetime_init"]
        fn qdatetimeInit() -> UniquePtr<QDateTime>;
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qdatetime_init_from_date_and_time"]
        fn qdatetimeInitFromDateAndTime(date: &QDate, time: &QTime) -> UniquePtr<QDateTime>;
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qdatetime_init_from_qdatetime"]
        fn qdatetimeInitFromQDateTime(datetime: &QDateTime) -> UniquePtr<QDateTime>;
    }

    impl UniquePtr<QDateTime> {}
}

/// The QDateTimeCpp class provides date and time functions.
///
/// Note that this is the C++ representation and QDateTime should be used in Rust.
pub type QDateTime = ffi::QDateTime;

impl QDateTime {
    /// Constrct a default null QDateTime
    pub fn null() -> cxx::UniquePtr<Self> {
        ffi::qdatetime_init()
    }

    /// Construct a Rust QDateTime from an existing QDateTimeCpp, this is a copy operation.
    pub fn from_ref(qdatetime: &QDateTime) -> cxx::UniquePtr<Self> {
        ffi::qdatetime_init_from_qdatetime(qdatetime)
    }

    /// Construct a Rust QDateTime from a given QDate and QTime
    pub fn from_date_and_time(date: &QDate, time: &QTime) -> cxx::UniquePtr<Self> {
        ffi::qdatetime_init_from_date_and_time(date, time)
    }

    /// Sets the date part of this datetime to date. If no time is set yet, it is set to midnight.
    /// If date is invalid, this QDateTime becomes invalid.
    pub fn set_date(self: std::pin::Pin<&mut Self>, date: QDate) {
        ffi::qdatetime_set_date(self, date);
    }

    /// Sets the time part of this datetime to time. If time is not valid, this function sets it to midnight.
    /// Therefore, it's possible to clear any set time in a QDateTimeCpp by setting it to a default QTime:
    pub fn set_time(self: std::pin::Pin<&mut Self>, time: QTime) {
        ffi::qdatetime_set_time(self, time);
    }
}

impl From<&QDateTime> for cxx::UniquePtr<QDateTime> {
    fn from(value: &QDateTime) -> cxx::UniquePtr<QDateTime> {
        QDateTime::from_ref(value)
    }
}
