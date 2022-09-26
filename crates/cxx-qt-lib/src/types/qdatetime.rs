// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

use crate::{QDate, QTime};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QDate = crate::QDate;
        type QDateTime = super::QDateTime;
        type QTime = crate::QTime;

        /// Returns the date part of the datetime.
        fn date(self: &QDateTime) -> QDate;
        /// Returns the time part of the datetime.
        fn time(self: &QDateTime) -> QTime;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qdatetime_drop"]
        fn qdatetimeDrop(datetime: &mut QDateTime);
        #[doc(hidden)]
        #[rust_name = "qdatetime_init_default"]
        fn qdatetimeInitDefault() -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_init_from_date_and_time"]
        fn qdatetimeInitFromDateAndTime(date: &QDate, time: &QTime) -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_init_from_qdatetime"]
        fn qdatetimeInitFromQDateTime(datetime: &QDateTime) -> QDateTime;

        // Note that Qt 5 takes const-ref and Qt 6 takes by-value
        // for QDateTime::setDate and QDateTime::setTime
        //
        // We want by-value, as that is Rust-idiomatic, so for Qt 5 we create a proxy
        #[doc(hidden)]
        #[rust_name = "qdatetime_set_date"]
        fn qdatetimeSetDate(datetime: &mut QDateTime, date: QDate);
        #[doc(hidden)]
        #[rust_name = "qdatetime_set_time"]
        fn qdatetimeSetTime(datetime: &mut QDateTime, time: QTime);
    }
}

/// The QDateTime class provides date and time functions.
#[repr(C)]
pub struct QDateTime {
    _space: MaybeUninit<usize>,
}

impl QDateTime {
    /// Construct a Rust QDateTime from a given QDate and QTime
    pub fn from_date_and_time(date: &QDate, time: &QTime) -> Self {
        ffi::qdatetime_init_from_date_and_time(date, time)
    }

    /// Sets the date part of this datetime to date. If no time is set yet, it is set to midnight.
    /// If date is invalid, this QDateTime becomes invalid.
    pub fn set_date(&mut self, date: QDate) {
        ffi::qdatetime_set_date(self, date);
    }

    /// Sets the time part of this datetime to time. If time is not valid, this function sets it to midnight.
    /// Therefore, it's possible to clear any set time in a QDateTime by setting it to a default QTime.
    pub fn set_time(&mut self, time: QTime) {
        ffi::qdatetime_set_time(self, time);
    }
}

impl Clone for QDateTime {
    /// Constructs a copy of the other datetime.
    fn clone(&self) -> Self {
        ffi::qdatetime_init_from_qdatetime(self)
    }
}

impl Default for QDateTime {
    /// Construct a default null QDateTime
    fn default() -> Self {
        ffi::qdatetime_init_default()
    }
}

impl Drop for QDateTime {
    /// Destroys the datetime.
    fn drop(&mut self) {
        ffi::qdatetime_drop(self);
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QDateTime {
    type Id = type_id!("QDateTime");
    type Kind = cxx::kind::Trivial;
}
