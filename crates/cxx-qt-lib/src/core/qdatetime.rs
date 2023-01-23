// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;
use std::{cmp::Ordering, fmt};

use crate::{QDate, QTime};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qdatetime.h");
        include!("cxx-qt-lib/qdate.h");
        include!("cxx-qt-lib/qtime.h");
        include!("cxx-qt-lib/qstring.h");

        type QDate = crate::QDate;
        type QDateTime = super::QDateTime;
        type QTime = crate::QTime;
        type QString = crate::QString;

        /// Returns the date part of the datetime.
        fn date(self: &QDateTime) -> QDate;
        /// Returns the time part of the datetime.
        fn time(self: &QDateTime) -> QTime;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qdatetime_drop"]
        fn drop(datetime: &mut QDateTime);
        #[doc(hidden)]
        #[rust_name = "qdatetime_init_default"]
        fn construct() -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_init_from_date_and_time"]
        fn construct(date: &QDate, time: &QTime) -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_init_from_qdatetime"]
        fn construct(datetime: &QDateTime) -> QDateTime;

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
        #[doc(hidden)]
        #[rust_name = "qdatetime_eq"]
        fn operatorEq(a: &QDateTime, b: &QDateTime) -> bool;
        #[doc(hidden)]
        #[rust_name = "qdatetime_cmp"]
        fn operatorCmp(a: &QDateTime, b: &QDateTime) -> i8;
        #[doc(hidden)]
        #[rust_name = "qdatetime_to_qstring"]
        fn toQString(value: &QDateTime) -> QString;
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

impl std::cmp::PartialEq for QDateTime {
    fn eq(&self, other: &Self) -> bool {
        ffi::qdatetime_eq(self, other)
    }
}

impl std::cmp::Eq for QDateTime {}

impl std::cmp::PartialOrd for QDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        crate::get_ordering(ffi::qdatetime_cmp(self, other))
    }
}

impl fmt::Display for QDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qdatetime_to_qstring(self))
    }
}

impl fmt::Debug for QDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
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
