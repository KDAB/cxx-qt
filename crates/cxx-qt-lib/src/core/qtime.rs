// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{AnyDateFormat, QString};
use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type DateFormat = crate::DateFormat;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qtime.h");
        include!("cxx-qt-lib/qstring.h");

        type QTime = super::QTime;
        type QString = crate::QString;

        /// Returns a QTime object containing a time ms milliseconds later
        /// than the time of this object (or earlier if ms is negative).
        #[rust_name = "add_msecs"]
        fn addMSecs(self: &QTime, ms: i32) -> QTime;

        /// Returns a QTime object containing a time s seconds later than the
        /// time of this object (or earlier if s is negative).
        #[rust_name = "add_secs"]
        fn addSecs(self: &QTime, s: i32) -> QTime;

        /// Returns the hour part (0 to 23) of the time.
        fn hour(self: &QTime) -> i32;

        /// Returns true if the time is null (i.e., the QTime object was
        /// constructed using the default constructor); otherwise returns false.
        /// A null time is also an invalid time.
        #[rust_name = "is_null"]
        fn isNull(self: &QTime) -> bool;

        /// Returns true if the time is valid; otherwise returns false.
        #[rust_name = "is_valid"]
        fn isValid(self: &QTime) -> bool;

        /// Returns the minute part (0 to 59) of the time.
        fn minute(self: &QTime) -> i32;

        /// Returns the millisecond part (0 to 999) of the time.
        fn msec(self: &QTime) -> i32;

        /// Returns the number of msecs since the start of the day, i.e. since 00:00:00.
        #[rust_name = "msecs_since_start_of_day"]
        fn msecsSinceStartOfDay(self: &QTime) -> i32;

        /// Returns the second part (0 to 59) of the time.
        fn second(self: &QTime) -> i32;

        /// Sets the time to hour h, minute m, seconds s and milliseconds ms.
        #[rust_name = "set_hms"]
        fn setHMS(self: &mut QTime, h: i32, m: i32, s: i32, ms: i32) -> bool;

        #[doc(hidden)]
        #[rust_name = "format_qstring"]
        fn toString(self: &QTime, format: &QString) -> QString;

        #[doc(hidden)]
        #[rust_name = "format_dateformat"]
        fn toString(self: &QTime, format: DateFormat) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qtime_current_time"]
        fn qtimeCurrentTime() -> QTime;

        #[doc(hidden)]
        #[rust_name = "qtime_from_msecs_since_start_of_day"]
        fn qtimeFromMSecsSinceStartOfDay(msecs: i32) -> QTime;

        #[doc(hidden)]
        #[rust_name = "qtime_from_qstring_qstring"]
        fn qtimeFromString(string: &QString, format: &QString) -> QTime;
        #[doc(hidden)]
        #[rust_name = "qtime_from_qstring_dateformat"]
        fn qtimeFromString(string: &QString, format: DateFormat) -> QTime;

        #[doc(hidden)]
        #[rust_name = "qtime_msecs_to"]
        fn qtimeMSecsTo(time: &QTime, t: QTime) -> i32;

        #[doc(hidden)]
        #[rust_name = "qtime_secs_to"]
        fn qtimeSecsTo(time: &QTime, t: QTime) -> i32;

        #[doc(hidden)]
        #[rust_name = "qtime_is_valid"]
        fn qtimeIsValid(h: i32, m: i32, s: i32, ms: i32) -> bool;
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
        #[rust_name = "qtime_to_debug_qstring"]
        fn toDebugQString(value: &QTime) -> QString;
    }
}

/// The QTime class provides clock time functions.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct QTime {
    mds: i32,
}

impl QTime {
    /// Returns the current time as reported by the system clock.
    pub fn current_time() -> Self {
        ffi::qtime_current_time()
    }

    /// Returns a new QTime instance with the time set to the number of msecs
    /// since the start of the day, i.e. since 00:00:00.
    pub fn from_msecs_since_start_of_day(msecs: i32) -> Self {
        ffi::qtime_from_msecs_since_start_of_day(msecs)
    }

    /// Returns the QTime represented by the string, using the format given.
    /// If the string cannot be parsed, returns `None`.
    pub fn from_qstring_opt<'a, T>(string: &QString, format: T) -> Option<Self>
    where
        T: Into<AnyDateFormat<'a>>,
    {
        let parsed = Self::from_qstring(string, format);
        if parsed.is_valid() {
            Some(parsed)
        } else {
            None
        }
    }

    /// Returns the QTime represented by the string, using the format given.
    /// If the string cannot be parsed, returns an invalid time.
    fn from_qstring<'a, T>(string: &QString, format: T) -> Self
    where
        T: Into<AnyDateFormat<'a>>,
    {
        match format.into() {
            AnyDateFormat::DateFormat(f) => ffi::qtime_from_qstring_dateformat(string, f),
            AnyDateFormat::QString(f) => ffi::qtime_from_qstring_qstring(string, f),
        }
    }

    /// Returns the time as a string. The format parameter determines the format of the result string.
    pub fn to_qstring<'a, T>(&self, format: T) -> QString
    where
        T: Into<AnyDateFormat<'a>>,
    {
        match format.into() {
            AnyDateFormat::DateFormat(f) => self.format_dateformat(f),
            AnyDateFormat::QString(f) => self.format_qstring(f),
        }
    }

    /// Returns the number of milliseconds from this time to t.
    /// If t is earlier than this time, the number of milliseconds returned is negative.
    pub fn msecs_to(&self, t: Self) -> i32 {
        ffi::qtime_msecs_to(self, t)
    }

    /// Constructs a time with hour h, minute m, seconds s and milliseconds ms.
    pub fn new(h: i32, m: i32, s: i32, ms: i32) -> Self {
        ffi::qtime_init(h, m, s, ms)
    }

    /// Returns the number of seconds from this time to t.
    /// If t is earlier than this time, the number of seconds returned is negative.
    pub fn secs_to(&self, t: Self) -> i32 {
        ffi::qtime_secs_to(self, t)
    }

    /// Returns true if the specified time is valid; otherwise returns false.
    /// The time is valid if h is in the range 0 to 23, m and s are in the range 0 to 59, and ms is in the range 0 to 999.
    pub fn is_valid_time(h: i32, m: i32, s: i32, ms: i32) -> bool {
        ffi::qtime_is_valid(h, m, s, ms)
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
        write!(f, "{}", self.to_qstring(ffi::DateFormat::TextDate))
    }
}

impl fmt::Debug for QTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qtime_to_debug_qstring(self))
    }
}

#[cfg(feature = "chrono")]
use chrono::Timelike;

#[cfg(feature = "chrono")]
impl TryFrom<chrono::NaiveTime> for QTime {
    type Error = &'static str;

    /// Errors if [chrono::NaiveTime] has milliseconds larger than 999,
    /// as Qt does not support representing a leap second in this way
    fn try_from(value: chrono::NaiveTime) -> Result<Self, Self::Error> {
        let ms = (value.nanosecond() / 1_000_000) as i32;
        // NaiveTime can have a nanosecond larger than 1 second
        // to represent a leap second.
        //
        // Qt has no way to represent this, we could add 1 second but
        // when then happens if the time is 23:59:59 + 1 ?
        if ms > 999 {
            return Err("leap second is not supported in Qt");
        }

        Ok(QTime::new(
            value.hour() as i32,
            value.minute() as i32,
            value.second() as i32,
            ms,
        ))
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<QTime> for chrono::NaiveTime {
    type Error = &'static str;

    fn try_from(value: QTime) -> Result<Self, Self::Error> {
        chrono::NaiveTime::from_hms_milli_opt(
            value.hour() as u32,
            value.minute() as u32,
            value.second() as u32,
            value.msec() as u32,
        )
        .ok_or("invalid hour, minute, second and/or millisecond")
    }
}

#[cfg(feature = "time")]
impl From<time::Time> for QTime {
    fn from(value: time::Time) -> Self {
        QTime::new(
            value.hour() as i32,
            value.minute() as i32,
            value.second() as i32,
            value.millisecond() as i32,
        )
    }
}

#[cfg(feature = "time")]
impl TryFrom<QTime> for time::Time {
    type Error = time::error::ComponentRange;

    fn try_from(value: QTime) -> Result<Self, Self::Error> {
        time::Time::from_hms_milli(
            value.hour() as u8,
            value.minute() as u8,
            value.second() as u8,
            value.msec() as u16,
        )
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QTime is trivial.
unsafe impl ExternType for QTime {
    type Id = type_id!("QTime");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
#[cfg(feature = "chrono")]
mod test_chrono {
    use super::*;

    #[test]
    fn qtime_from_chrono_naive() {
        let naive = chrono::NaiveTime::from_hms_milli_opt(1, 2, 3, 4).unwrap();
        let qtime = QTime::new(1, 2, 3, 4);
        assert_eq!(QTime::try_from(naive).unwrap(), qtime);
    }

    #[test]
    fn qtime_from_chrono_naive_leap_second() {
        let naive = chrono::NaiveTime::from_hms_milli_opt(1, 2, 59, 1_999).unwrap();
        assert!(QTime::try_from(naive).is_err());
    }

    #[test]
    fn qtime_to_chrono_naive() {
        let naive = chrono::NaiveTime::from_hms_milli_opt(1, 2, 3, 4).unwrap();
        let qtime = QTime::new(1, 2, 3, 4);
        assert_eq!(chrono::NaiveTime::try_from(qtime).unwrap(), naive);
    }
}

#[cfg(test)]
#[cfg(feature = "time")]
mod test_time {
    use super::*;

    #[test]
    fn qtime_from_time_time() {
        let time_time = time::Time::from_hms_milli(1, 2, 3, 4).unwrap();
        let qtime = QTime::new(1, 2, 3, 4);
        assert_eq!(QTime::from(time_time), qtime);
    }

    #[test]
    fn qtime_to_time_time() {
        let time_time = time::Time::from_hms_milli(1, 2, 3, 4).unwrap();
        let qtime = QTime::new(1, 2, 3, 4);
        assert_eq!(time::Time::try_from(qtime).unwrap(), time_time);
    }
}
