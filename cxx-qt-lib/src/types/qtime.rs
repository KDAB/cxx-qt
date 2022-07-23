// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qt_types.h");

        type QTime = super::QTime;

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

        #[doc(hidden)]
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qtime_init_default"]
        fn qtimeInitDefault() -> QTime;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qtime_init"]
        fn qtimeInit(h: i32, m: i32, s: i32, ms: i32) -> QTime;
    }
}

/// The QTime class provides clock time functions.
#[derive(Debug, Clone, Copy)]
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

// Safety:
//
// Static checks on the C++ side ensure that QTime is trivial.
unsafe impl ExternType for QTime {
    type Id = type_id!("QTime");
    type Kind = cxx::kind::Trivial;
}

#[doc(hidden)]
impl From<&QTime> for QTime {
    // TODO: in the future remove at least the deref to a clone and potentially remove this ?
    fn from(qtime: &QTime) -> Self {
        *qtime
    }
}
