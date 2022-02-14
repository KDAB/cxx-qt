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
pub type QDateTimeCpp = ffi::QDateTime;

impl QDateTimeCpp {
    /// Create a new Rust QDateTime from this QDateTimeCpp.
    /// This is a copy operation so any changes will not propagate to the original QDateTimeCpp.
    pub fn to_rust(&self) -> QDateTime {
        QDateTime::from_qdatetime(self)
    }
}

/// The Rust representation of Qt's QDateTime
///
/// Internally this holds a UniquePtr to a QDateTimeCpp which has been constructed on the C++ side.
pub struct QDateTime {
    inner: cxx::UniquePtr<QDateTimeCpp>,
}

impl Default for QDateTime {
    fn default() -> Self {
        QDateTime::from_unique_ptr(ffi::qdatetime_init())
    }
}

impl QDateTime {
    /// Construct a Rust QDateTime from an existing UniquePtr<QDateTimeCpp> this is a move operation
    ///
    /// This is used in QVariant::value so that we don't need to perform another copy
    pub(crate) fn from_unique_ptr(ptr: cxx::UniquePtr<QDateTimeCpp>) -> Self {
        Self { inner: ptr }
    }

    /// Construct a Rust QDateTime from an existing QDateTimeCpp, this is a copy operation.
    pub fn from_qdatetime(qdatetime: &QDateTimeCpp) -> Self {
        Self {
            inner: ffi::qdatetime_init_from_qdatetime(qdatetime),
        }
    }

    /// Construct a Rust QDateTime from a given QDate and QTime
    pub fn from_date_and_time(date: &QDate, time: &QTime) -> Self {
        Self {
            inner: ffi::qdatetime_init_from_date_and_time(date, time),
        }
    }

    /// Returns the date part of the datetime.
    pub fn date(&self) -> QDate {
        if let Some(inner) = self.inner.as_ref() {
            inner.date()
        } else {
            QDate::default()
        }
    }

    /// Returns the time part of the datetime.
    pub fn time(&self) -> QTime {
        if let Some(inner) = self.inner.as_ref() {
            inner.time()
        } else {
            QTime::default()
        }
    }

    /// Sets the date part of this datetime to date. If no time is set yet, it is set to midnight.
    /// If date is invalid, this QDateTime becomes invalid.
    pub fn set_date(&mut self, date: QDate) {
        if let Some(inner) = self.inner.as_mut() {
            ffi::qdatetime_set_date(inner, date);
        }
    }

    /// Sets the time part of this datetime to time. If time is not valid, this function sets it to midnight.
    /// Therefore, it's possible to clear any set time in a QDateTimeCpp by setting it to a default QTime:
    pub fn set_time(&mut self, time: QTime) {
        if let Some(inner) = self.inner.as_mut() {
            ffi::qdatetime_set_time(inner, time);
        }
    }
}

impl crate::ToUniquePtr for QDateTime {
    type CppType = QDateTimeCpp;

    /// Retrieve the UniquePtr to the Qt QDateTimeCpp of this Rust QDateTime
    /// so that this object can be passed back to C++.
    fn to_unique_ptr(self) -> cxx::UniquePtr<QDateTimeCpp> {
        self.inner
    }
}

impl From<&QDateTimeCpp> for QDateTime {
    fn from(qdatetime: &QDateTimeCpp) -> Self {
        qdatetime.to_rust()
    }
}
