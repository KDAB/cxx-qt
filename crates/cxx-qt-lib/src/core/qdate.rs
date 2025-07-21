// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

use crate::DateFormat;
use crate::QString;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type DateFormat = crate::DateFormat;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qtypes.h");
        type qint64 = crate::qint64;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qdate.h");
        type QDate = super::QDate;

        #[doc(hidden)]
        #[rust_name = "add_days_qint64"]
        fn addDays(self: &QDate, ndays: qint64) -> QDate;

        /// Returns a `QDate` object containing a date `nmonths` later than the date of this object (or earlier if `nmonths` is negative).
        #[rust_name = "add_months"]
        fn addMonths(self: &QDate, nmonths: i32) -> QDate;

        /// Returns a `QDate` object containing a date `nyears` later than the date of this object (or earlier if `nyears` is negative).
        ///
        /// **Note:** If the ending day/month combination does not exist in the resulting year (e.g., for the Gregorian calendar, if the date was Feb 29 and the final year is not a leap year), this function will return a date that is the latest valid date in the given month (in the example, Feb 28).
        #[rust_name = "add_years"]
        fn addYears(self: &QDate, nyears: i32) -> QDate;

        /// Returns the day of the month for this date.
        ///
        /// Uses the Gregorian calendar (for which the return ranges from 1 to 31). Returns 0 if the date is invalid.
        fn day(self: &QDate) -> i32;

        /// Returns the weekday (1 = Monday to 7 = Sunday) for this date.
        ///
        /// Uses the Gregorian calendar. Returns 0 if the date is invalid.
        #[rust_name = "day_of_week"]
        fn dayOfWeek(self: &QDate) -> i32;

        /// Returns the day of the year (1 for the first day) for this date.
        ///
        /// Uses the Gregorian calendar. Returns 0 if either the date or the first day of its year is invalid.
        #[rust_name = "day_of_year"]
        fn dayOfYear(self: &QDate) -> i32;

        /// Returns the number of days in the month for this date.
        ///
        /// Uses the Gregorian calendar (for which the result ranges from 28 to 31). Returns 0 if the date is invalid.
        #[rust_name = "days_in_monyth"]
        fn daysInMonth(self: &QDate) -> i32;

        /// Returns the number of days in the year for this date.
        ///
        /// Uses the Gregorian calendar (for which the result is 365 or 366). Returns 0 if the date is invalid.
        #[rust_name = "days_in_year"]
        fn daysInYear(self: &QDate) -> i32;

        /// Returns `true` if the date is null; otherwise returns `false`. A null date is invalid.
        #[rust_name = "is_null"]
        fn isNull(self: &QDate) -> bool;

        /// Returns `true` if this date is valid; otherwise returns `false`.
        #[rust_name = "is_valid"]
        fn isValid(self: &QDate) -> bool;

        /// Returns the month-number for the date.
        ///
        /// Uses the Gregorian calendar (for which the result ranges from 1 to 12). Returns 0 if the date is invalid.
        fn month(self: &QDate) -> i32;

        /// Sets this date to represent the date, in the Gregorian calendar, with the given `year`, `month` and `day` numbers.
        /// Returns `true` if the resulting date is valid, otherwise it sets this date to represent an invalid date and returns `false`.
        #[rust_name = "set_date"]
        fn setDate(self: &mut QDate, y: i32, m: i32, d: i32) -> bool;

        /// Returns the date as a string. The `format` parameter determines the format of the string.
        #[rust_name = "format_enum"]
        fn toString(self: &QDate, format: DateFormat) -> QString;

        /// Returns the year of this date.
        ///
        /// Uses the Gregorian calendar. Returns 0 if the date is invalid.
        fn year(self: &QDate) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qdate_current_date"]
        fn qdateCurrentDate() -> QDate;

        #[doc(hidden)]
        #[rust_name = "qdate_days_to"]
        fn qdateDaysTo(date: &QDate, d: QDate) -> qint64;

        #[doc(hidden)]
        #[rust_name = "qdate_from_string"]
        fn qdateFromString(string: &QString, format: &QString) -> QDate;
        #[doc(hidden)]
        #[rust_name = "qdate_from_string_enum"]
        fn qdateFromString(string: &QString, format: DateFormat) -> QDate;

        #[doc(hidden)]
        #[rust_name = "qdate_is_leap_year"]
        fn qdateIsLeapYear(year: i32) -> bool;

        #[doc(hidden)]
        #[rust_name = "qdate_to_format"]
        fn qdateToFormat(date: &QDate, format: &QString) -> QString;
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
        #[rust_name = "qdate_to_debug_qstring"]
        fn toDebugQString(value: &QDate) -> QString;
    }
}

/// The `QDate` class provides date functions.
///
/// Qt Documentation: [QDate](https://doc.qt.io/qt/qdate.html#details)
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
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
        self.format_enum(DateFormat::TextDate).fmt(f)
    }
}

impl fmt::Debug for QDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qdate_to_debug_qstring(self).fmt(f)
    }
}

impl QDate {
    /// Returns a `QDate` object containing a date `ndays` later than the date of this object (or earlier if `ndays` is negative).
    ///
    /// Returns a null date if the current date is invalid or the new date is out of range.
    pub fn add_days(self: &QDate, ndays: i64) -> QDate {
        self.add_days_qint64(ndays.into())
    }

    /// Returns the system clock's current date.
    pub fn current_date() -> Self {
        ffi::qdate_current_date()
    }

    /// Returns the number of days from this date to `date` (which is negative if `date` is earlier than this date).
    ///
    /// Returns 0 if either date is invalid.
    pub fn days_to(&self, date: Self) -> i64 {
        ffi::qdate_days_to(self, date).into()
    }

    /// Returns the date as a string. The `format` parameter determines the format of the string.
    pub fn format(&self, format: &QString) -> QString {
        ffi::qdate_to_format(self, format)
    }

    /// Converts the Julian day `jd` to a `QDate`.
    pub fn from_julian_day(jd: i64) -> Self {
        Self { jd }
    }

    /// Returns the `QDate` represented by the string `string`, using the `format` given, or `None` if the string cannot be parsed.
    pub fn from_string(string: &QString, format: &QString) -> Option<Self> {
        let date = ffi::qdate_from_string(string, format);
        if date.is_valid() {
            Some(date)
        } else {
            None
        }
    }

    /// Returns the `QDate` represented in the string `string`, using the `format` given, or `None` if the string cannot be parsed.
    ///
    /// Note for [`DateFormat::TextDate`]: only English month names (e.g. "Jan" in short form or "January" in long form) are recognized.
    pub fn from_string_enum(string: &QString, format: DateFormat) -> Option<Self> {
        let date = ffi::qdate_from_string_enum(string, format);
        if date.is_valid() {
            Some(date)
        } else {
            None
        }
    }

    /// Returns `true` if the specified `year` is a leap year in the Gregorian calendar; otherwise returns `false`.
    pub fn is_leap_year(year: i32) -> bool {
        ffi::qdate_is_leap_year(year)
    }

    /// Constructs a date with year `y`, month `m` and day `d`.
    ///
    /// The date is understood in terms of the Gregorian calendar. If the specified date is invalid, the date is not set and [`is_valid`](Self::is_valid) returns `false`.
    ///
    /// **Warning:** Years 1 to 99 are interpreted as is. Year 0 is invalid.
    pub fn new(y: i32, m: i32, d: i32) -> Self {
        ffi::qdate_init(y, m, d)
    }

    /// Converts the date to a Julian day.
    pub fn to_julian_day(&self) -> i64 {
        self.jd
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QDate is trivial.
unsafe impl ExternType for QDate {
    type Id = type_id!("QDate");
    type Kind = cxx::kind::Trivial;
}

#[cfg(feature = "chrono")]
use chrono::Datelike;

#[cfg(feature = "chrono")]
impl From<chrono::NaiveDate> for QDate {
    fn from(value: chrono::NaiveDate) -> Self {
        QDate::new(value.year(), value.month() as i32, value.day() as i32)
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<QDate> for chrono::NaiveDate {
    type Error = &'static str;

    fn try_from(value: QDate) -> Result<Self, Self::Error> {
        chrono::NaiveDate::from_ymd_opt(value.year(), value.month() as u32, value.day() as u32)
            .ok_or("out-of-range date, invalid month and/or day")
    }
}

#[cfg(feature = "time")]
impl From<time::Date> for QDate {
    fn from(value: time::Date) -> Self {
        QDate::new(
            value.year(),
            Into::<u8>::into(value.month()) as i32,
            value.day() as i32,
        )
    }
}

#[cfg(feature = "time")]
impl TryFrom<QDate> for time::Date {
    type Error = time::error::ComponentRange;

    fn try_from(value: QDate) -> Result<Self, Self::Error> {
        time::Date::from_calendar_date(
            value.year(),
            time::Month::try_from(value.month() as u8)?,
            value.day() as u8,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn qdate_current_date() {
        let date_a = QDate::current_date();
        let date_b = date_a.add_days(100.into());
        assert_eq!(date_a.days_to(date_b), 100);
    }

    #[test]
    fn qdate_julian_day() {
        let date_a = QDate::from_julian_day(1000);
        let date_b = QDate::from_julian_day(1010);
        assert_eq!(date_a.days_to(date_b), 10);
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn qdate_from_chrono_naive() {
        let naive = chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let qdate = QDate::new(2023, 1, 1);
        assert_eq!(QDate::from(naive), qdate);
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn qdate_to_chrono_naive() {
        let naive = chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let qdate = QDate::new(2023, 1, 1);
        assert_eq!(chrono::NaiveDate::try_from(qdate).unwrap(), naive);
    }

    #[cfg(feature = "time")]
    #[test]
    fn qdate_from_time_date() {
        let time_date = time::Date::from_calendar_date(2023, time::Month::January, 1).unwrap();
        let qdate = QDate::new(2023, 1, 1);
        assert_eq!(QDate::from(time_date), qdate);
    }

    #[cfg(feature = "time")]
    #[test]
    fn qdate_to_time_date() {
        let time_date = time::Date::from_calendar_date(2023, time::Month::January, 1).unwrap();
        let qdate = QDate::new(2023, 1, 1);
        assert_eq!(time::Date::try_from(qdate).unwrap(), time_date);
    }
}
