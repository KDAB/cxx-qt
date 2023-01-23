// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        include!("cxx-qt-lib/qdate.h");

        type QString = crate::QString;
        type QDate = super::QDate;

        /// Returns the year of this date.
        fn year(self: &QDate) -> i32;
        /// Returns the month-number for the date.
        ///
        /// Numbers the months of the year starting with 1 for the first
        fn month(self: &QDate) -> i32;
        /// Returns the day of the month for this date.
        fn day(self: &QDate) -> i32;

        /// Sets this to represent the date, in the Gregorian calendar, with the given year, month and day numbers.
        /// Returns true if the resulting date is valid, otherwise it sets this to represent an invalid date and returns false.
        #[rust_name = "set_date"]
        fn setDate(self: &mut QDate, y: i32, m: i32, d: i32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qdate_init_default"]
        fn construct() -> QDate;
        #[doc(hidden)]
        #[rust_name = "qdate_init"]
        fn construct(y: i32, m: i32, d: i32) -> QDate;
        #[doc(hidden)]
        #[rust_name = "qdate_to_qstring"]
        fn toQString(value: &QDate) -> QString;
    }
}

/// The QDate class provides date functions.
#[derive(Clone, PartialEq, Eq, PartialOrd)]
#[repr(C)]
pub struct QDate {
    jd: i64,
}

impl Default for QDate {
    /// Constructs a null date. Null dates are invalid.
    fn default() -> Self {
        ffi::qdate_init_default()
    }
}

impl fmt::Display for QDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qdate_to_qstring(self))
    }
}

impl fmt::Debug for QDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl QDate {
    /// Constructs a date with year y, month m and day d.
    pub fn new(y: i32, m: i32, d: i32) -> Self {
        ffi::qdate_init(y, m, d)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QDate is trivial.
unsafe impl ExternType for QDate {
    type Id = type_id!("QDate");
    type Kind = cxx::kind::Trivial;
}
