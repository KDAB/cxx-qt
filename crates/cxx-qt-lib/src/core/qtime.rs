// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qtime.h");
        include!("cxx-qt-lib/qstring.h");

        type QTime = super::QTime;
        type QString = crate::QString;

        /// Returns the hour part (0 to 23) of the time.
        fn hour(self: &QTime) -> i32;
        /// Returns the minute part (0 to 59) of the time.
        fn minute(self: &QTime) -> i32;
        /// Returns the second part (0 to 59) of the time.
        fn second(self: &QTime) -> i32;
        /// Returns the millisecond part (0 to 999) of the time.
        fn msec(self: &QTime) -> i32;

        /// Sets the time to hour h, minute m, seconds s and milliseconds ms.
        #[rust_name = "set_hms"]
        fn setHMS(self: &mut QTime, h: i32, m: i32, s: i32, ms: i32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qtime_init_default"]
        fn construct() -> QTime;
        #[doc(hidden)]
        #[rust_name = "qtime_init"]
        fn construct(h: i32, m: i32, s: i32, ms: i32) -> QTime;
        #[doc(hidden)]
        #[rust_name = "qtime_to_qstring"]
        fn toQString(value: &QTime) -> QString;
    }
}

/// The QTime class provides clock time functions.
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct QTime {
    mds: i32,
}

impl QTime {
    /// Constructs a time with hour h, minute m, seconds s and milliseconds ms.
    pub fn new(h: i32, m: i32, s: i32, ms: i32) -> Self {
        ffi::qtime_init(h, m, s, ms)
    }
}

impl Default for QTime {
    /// Constructs a null time object.
    fn default() -> Self {
        ffi::qtime_init_default()
    }
}

impl fmt::Display for QTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qtime_to_qstring(self))
    }
}

impl fmt::Debug for QTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QTime is trivial.
unsafe impl ExternType for QTime {
    type Id = type_id!("QTime");
    type Kind = cxx::kind::Trivial;
}
