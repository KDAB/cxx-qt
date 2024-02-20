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
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type TimeSpec = crate::TimeSpec;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qdate.h");
        type QDate = crate::QDate;
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = super::QDateTime;
        include!("cxx-qt-lib/qtime.h");
        type QTime = crate::QTime;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qtimezone.h");
        type QTimeZone = crate::QTimeZone;

        /// Returns a QDateTime object containing a datetime nmonths months later than the datetime of this object (or earlier if nmonths is negative).
        #[rust_name = "add_months"]
        fn addMonths(self: &QDateTime, nmonths: i32) -> QDateTime;

        /// Returns a QDateTime object containing a datetime nyears years later than the datetime of this object (or earlier if nyears is negative).
        #[rust_name = "add_years"]
        fn addYears(self: &QDateTime, nyears: i32) -> QDateTime;

        /// Returns the date part of the datetime.
        fn date(self: &QDateTime) -> QDate;

        /// Returns if this datetime falls in Daylight-Saving Time.
        #[rust_name = "is_daylight_time"]
        fn isDaylightTime(self: &QDateTime) -> bool;

        /// Returns true if both the date and the time are null; otherwise returns false. A null datetime is invalid.
        #[rust_name = "is_null"]
        fn isNull(self: &QDateTime) -> bool;

        /// Returns true if both the date and the time are valid and they are valid in the current Qt::TimeSpec, otherwise returns false.
        #[rust_name = "is_valid"]
        fn isValid(self: &QDateTime) -> bool;

        /// Returns this date-time's Offset From UTC in seconds.
        #[rust_name = "offset_from_utc"]
        fn offsetFromUtc(self: &QDateTime) -> i32;

        /// Sets the timeSpec() to Qt::OffsetFromUTC and the offset to offsetSeconds.
        #[rust_name = "set_offset_from_utc"]
        fn setOffsetFromUtc(self: &mut QDateTime, offset_seconds: i32);

        /// Sets the time specification used in this datetime to spec. The datetime will refer to a different point in time.
        #[rust_name = "set_time_spec"]
        fn setTimeSpec(self: &mut QDateTime, spec: TimeSpec);

        /// Returns the time part of the datetime.
        fn time(self: &QDateTime) -> QTime;

        /// Returns the time specification of the datetime.
        #[rust_name = "time_spec"]
        fn timeSpec(self: &QDateTime) -> TimeSpec;

        /// Returns the Time Zone Abbreviation for this datetime.
        #[rust_name = "time_zone_abbreviation"]
        fn timeZoneAbbreviation(self: &QDateTime) -> QString;

        /// Returns a datetime containing the date and time information in this datetime, but specified using the Qt::LocalTime definition.
        #[rust_name = "to_local_time"]
        fn toLocalTime(self: &QDateTime) -> QDateTime;

        /// Returns a copy of this datetime converted to a spec of Qt::OffsetFromUTC with the given offsetSeconds.
        #[rust_name = "to_offset_from_utc"]
        fn toOffsetFromUtc(self: &QDateTime, offset_seconds: i32) -> QDateTime;

        /// Returns a copy of this datetime converted to the given time spec.
        #[rust_name = "to_time_spec"]
        fn toTimeSpec(self: &QDateTime, spec: TimeSpec) -> QDateTime;

        /// Returns a copy of this datetime converted to the given timeZone
        #[rust_name = "to_time_zone"]
        fn toTimeZone(self: &QDateTime, timeZone: &QTimeZone) -> QDateTime;

        /// Returns a datetime containing the date and time information in this datetime, but specified using the Qt::UTC definition.
        #[rust_name = "to_utc"]
        fn toUTC(self: &QDateTime) -> QDateTime;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qdatetime_add_days"]
        fn qdatetimeAddDays(datetime: &QDateTime, ndays: i64) -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_add_msecs"]
        fn qdatetimeAddMSecs(datetime: &QDateTime, msecs: i64) -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_add_secs"]
        fn qdatetimeAddSecs(datetime: &QDateTime, secs: i64) -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_current_date_time"]
        fn qdatetimeCurrentDateTime() -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_current_date_time_utc"]
        fn qdatetimeCurrentDateTimeUtc() -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_current_msecs_since_epoch"]
        fn qdatetimeCurrentMSecsSinceEpoch() -> i64;
        #[doc(hidden)]
        #[rust_name = "qdatetime_current_secs_since_epoch"]
        fn qdatetimeCurrentSecsSinceEpoch() -> i64;
        #[doc(hidden)]
        #[rust_name = "qdatetime_days_to"]
        fn qdatetimeDaysTo(datetime: &QDateTime, other: &QDateTime) -> i64;
        #[doc(hidden)]
        #[rust_name = "qdatetime_from_msecs_since_epoch"]
        fn qdatetimeFromMSecsSinceEpoch(msecs: i64, time_zone: &QTimeZone) -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_from_secs_since_epoch"]
        fn qdatetimeFromSecsSinceEpoch(secs: i64, time_zone: &QTimeZone) -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_msecs_to"]
        fn qdatetimeMSecsTo(datetime: &QDateTime, other: &QDateTime) -> i64;
        #[doc(hidden)]
        #[rust_name = "qdatetime_secs_to"]
        fn qdatetimeSecsTo(datetime: &QDateTime, other: &QDateTime) -> i64;
        // Note that Qt 5 takes const-ref and Qt 6 takes by-value
        // for QDateTime::setDate and QDateTime::setTime
        //
        // We want by-value, as that is Rust-idiomatic, so for Qt 5 we create a proxy
        #[doc(hidden)]
        #[rust_name = "qdatetime_set_date"]
        fn qdatetimeSetDate(datetime: &mut QDateTime, date: QDate);
        #[doc(hidden)]
        #[rust_name = "qdatetime_set_msecs_since_epoch"]
        fn qdatetimeSetMSecsSinceEpoch(datetime: &mut QDateTime, msecs: i64);
        #[doc(hidden)]
        #[rust_name = "qdatetime_set_secs_since_epoch"]
        fn qdatetimeSetSecsSinceEpoch(datetime: &mut QDateTime, secs: i64);
        #[doc(hidden)]
        #[rust_name = "qdatetime_set_time"]
        fn qdatetimeSetTime(datetime: &mut QDateTime, time: QTime);
        #[doc(hidden)]
        #[rust_name = "qdatetime_time_zone"]
        fn qdatetimeTimeZone(datetime: &QDateTime) -> UniquePtr<QTimeZone>;
        #[doc(hidden)]
        #[rust_name = "qdatetime_to_msecs_since_epoch"]
        fn qdatetimeToMSecsSinceEpoch(datetime: &QDateTime) -> i64;
        #[doc(hidden)]
        #[rust_name = "qdatetime_to_secs_since_epoch"]
        fn qdatetimeToSecsSinceEpoch(datetime: &QDateTime) -> i64;
        #[rust_name = "qdatetime_settimezone"]
        fn qdatetimeSetTimeZone(datetime: &mut QDateTime, time_zone: &QTimeZone);
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
        #[rust_name = "qdatetime_init_from_date_and_time_time_zone"]
        fn construct(date: &QDate, time: &QTime, time_zone: &QTimeZone) -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_init_from_date_and_time_time_spec"]
        fn construct(
            date: &QDate,
            time: &QTime,
            time_spec: TimeSpec,
            offset_seconds: i32,
        ) -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_init_from_qdatetime"]
        fn construct(datetime: &QDateTime) -> QDateTime;

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
    /// Sets the time zone used in this datetime to toZone. The datetime will refer to a different point in time.
    pub fn set_time_zone(&mut self, time_zone: &ffi::QTimeZone) {
        ffi::qdatetime_settimezone(self, time_zone)
    }

    /// Returns a QDateTime object containing a datetime ndays days later than the datetime of this object (or earlier if ndays is negative).
    pub fn add_days(&self, ndays: i64) -> Self {
        ffi::qdatetime_add_days(self, ndays)
    }

    /// Returns a QDateTime object containing a datetime msecs milliseconds later than the datetime of this object (or earlier if msecs is negative).
    pub fn add_msecs(&self, msecs: i64) -> Self {
        ffi::qdatetime_add_msecs(self, msecs)
    }

    /// Returns a QDateTime object containing a datetime s seconds later than the datetime of this object (or earlier if s is negative).
    pub fn add_secs(&self, secs: i64) -> Self {
        ffi::qdatetime_add_secs(self, secs)
    }

    /// Returns the current datetime, as reported by the system clock, in the local time zone.
    pub fn current_date_time() -> Self {
        ffi::qdatetime_current_date_time()
    }

    /// Returns the current datetime, as reported by the system clock, in UTC.
    pub fn current_date_time_utc() -> Self {
        ffi::qdatetime_current_date_time_utc()
    }

    /// Returns the number of milliseconds since 1970-01-01T00:00:00 Universal Coordinated Time.
    /// This number is like the POSIX time_t variable, but expressed in milliseconds instead.
    pub fn current_msecs_since_epoch() -> i64 {
        ffi::qdatetime_current_msecs_since_epoch()
    }

    /// Returns the number of seconds since 1970-01-01T00:00:00 Universal Coordinated Time.
    pub fn current_secs_since_epoch() -> i64 {
        ffi::qdatetime_current_secs_since_epoch()
    }

    /// Returns the number of days from this datetime to the other datetime.
    /// The number of days is counted as the number of times midnight is reached between this datetime to the other datetime.
    /// This means that a 10 minute difference from 23:55 to 0:05 the next day counts as one day.
    pub fn days_to(&self, other: &Self) -> i64 {
        ffi::qdatetime_days_to(self, other)
    }

    /// Construct a Rust QDateTime from a given QDate, QTime, and QTimeZone
    pub fn from_date_and_time_time_zone(
        date: &QDate,
        time: &QTime,
        time_zone: &ffi::QTimeZone,
    ) -> Self {
        ffi::qdatetime_init_from_date_and_time_time_zone(date, time, time_zone)
    }

    /// Construct a Rust QDateTime from a given QDate, QTime, Qt::TimeSpec, and offset
    pub fn from_date_and_time_time_spec(
        date: &QDate,
        time: &QTime,
        time_spec: ffi::TimeSpec,
        offset_seconds: i32,
    ) -> Self {
        ffi::qdatetime_init_from_date_and_time_time_spec(date, time, time_spec, offset_seconds)
    }

    /// Returns a datetime whose date and time are the number of milliseconds msecs that have passed since 1970-01-01T00:00:00.000,
    /// Coordinated Universal Time (Qt::UTC) and with the given timeZone.
    pub fn from_msecs_since_epoch(msecs: i64, time_zone: &ffi::QTimeZone) -> Self {
        ffi::qdatetime_from_msecs_since_epoch(msecs, time_zone)
    }

    /// Returns a datetime whose date and time are the number of seconds secs that have passed since 1970-01-01T00:00:00.000,
    /// Coordinated Universal Time (Qt::UTC) and converted to the given spec.
    pub fn from_secs_since_epoch(secs: i64, time_zone: &ffi::QTimeZone) -> Self {
        ffi::qdatetime_from_secs_since_epoch(secs, time_zone)
    }

    /// Returns the number of milliseconds from this datetime to the other datetime.
    /// If the other datetime is earlier than this datetime, the value returned is negative.
    pub fn msecs_to(&self, other: &Self) -> i64 {
        ffi::qdatetime_msecs_to(self, other)
    }

    /// Returns the number of seconds from this datetime to the other datetime.
    /// If the other datetime is earlier than this datetime, the value returned is negative.
    pub fn secs_to(&self, other: &Self) -> i64 {
        ffi::qdatetime_secs_to(self, other)
    }

    /// Sets the date part of this datetime to date. If no time is set yet, it is set to midnight.
    /// If date is invalid, this QDateTime becomes invalid.
    pub fn set_date(&mut self, date: QDate) {
        ffi::qdatetime_set_date(self, date);
    }

    /// Sets the date and time given the number of milliseconds msecs that have passed since 1970-01-01T00:00:00.000,
    /// Coordinated Universal Time (Qt::UTC). On systems that do not support time zones this function will behave as if local time were Qt::UTC.
    pub fn set_msecs_since_epoch(&mut self, msecs: i64) {
        ffi::qdatetime_set_msecs_since_epoch(self, msecs);
    }

    /// Sets the date and time given the number of seconds secs that have passed since 1970-01-01T00:00:00.000,
    /// Coordinated Universal Time (Qt::UTC). On systems that do not support time zones this function will behave as if local time were Qt::UTC.
    pub fn set_secs_since_epoch(&mut self, secs: i64) {
        ffi::qdatetime_set_secs_since_epoch(self, secs);
    }

    /// Sets the time part of this datetime to time. If time is not valid, this function sets it to midnight.
    /// Therefore, it's possible to clear any set time in a QDateTime by setting it to a default QTime.
    pub fn set_time(&mut self, time: QTime) {
        ffi::qdatetime_set_time(self, time);
    }

    /// Returns the time zone of the datetime.
    pub fn time_zone(&self) -> cxx::UniquePtr<ffi::QTimeZone> {
        ffi::qdatetime_time_zone(self)
    }

    /// Returns the datetime as the number of milliseconds that have passed since 1970-01-01T00:00:00.000, Coordinated Universal Time (Qt::UTC).
    pub fn to_msecs_since_epoch(&self) -> i64 {
        ffi::qdatetime_to_msecs_since_epoch(self)
    }

    /// Returns the datetime as the number of seconds that have passed since 1970-01-01T00:00:00.000, Coordinated Universal Time (Qt::UTC).
    pub fn to_secs_since_epoch(&self) -> i64 {
        ffi::qdatetime_to_secs_since_epoch(self)
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

impl PartialEq for QDateTime {
    fn eq(&self, other: &Self) -> bool {
        ffi::qdatetime_eq(self, other)
    }
}

impl Eq for QDateTime {}

impl PartialOrd for QDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QDateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        ffi::qdatetime_cmp(self, other).cmp(&0)
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

#[cfg(feature = "chrono")]
use chrono::Offset;

#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for QDateTime {
    fn from(value: chrono::DateTime<Tz>) -> Self {
        QDateTime::from_date_and_time_time_spec(
            &QDate::from(value.date_naive()),
            &QTime::from(value.time()),
            ffi::TimeSpec::OffsetFromUTC,
            value.offset().fix().local_minus_utc(),
        )
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<QDateTime> for chrono::DateTime<chrono::FixedOffset> {
    type Error = &'static str;

    fn try_from(value: QDateTime) -> Result<Self, Self::Error> {
        let timezone_east = chrono::FixedOffset::east_opt(value.offset_from_utc())
            .ok_or("out-of-bound offset secs")?;
        let naivedatetime_east = chrono::NaiveDate::try_from(value.date())?
            .and_time(chrono::NaiveTime::try_from(value.time())?);
        Ok(chrono::DateTime::<chrono::FixedOffset>::from_local(
            naivedatetime_east,
            timezone_east,
        ))
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<QDateTime> for chrono::DateTime<chrono::Utc> {
    type Error = &'static str;

    fn try_from(value: QDateTime) -> Result<Self, Self::Error> {
        let value_utc = value.to_utc();
        let naivedatetime_utc = chrono::NaiveDate::try_from(value_utc.date())?
            .and_time(chrono::NaiveTime::try_from(value_utc.time())?);
        Ok(chrono::DateTime::<chrono::Utc>::from_utc(
            naivedatetime_utc,
            chrono::Utc,
        ))
    }
}

#[cfg(feature = "time")]
impl From<time::OffsetDateTime> for QDateTime {
    fn from(value: time::OffsetDateTime) -> Self {
        QDateTime::from_date_and_time_time_spec(
            &QDate::from(value.date()),
            &QTime::from(value.time()),
            ffi::TimeSpec::OffsetFromUTC,
            value.offset().whole_seconds(),
        )
    }
}

#[cfg(feature = "time")]
impl From<time::PrimitiveDateTime> for QDateTime {
    fn from(value: time::PrimitiveDateTime) -> Self {
        QDateTime::from_date_and_time_time_spec(
            &QDate::from(value.date()),
            &QTime::from(value.time()),
            ffi::TimeSpec::UTC,
            0,
        )
    }
}

#[cfg(feature = "time")]
impl TryFrom<QDateTime> for time::OffsetDateTime {
    type Error = time::error::ComponentRange;

    fn try_from(value: QDateTime) -> Result<Self, Self::Error> {
        Ok(time::Date::try_from(value.date())?
            .with_time(time::Time::try_from(value.time())?)
            .assume_offset(time::UtcOffset::from_whole_seconds(
                value.offset_from_utc(),
            )?))
    }
}

#[cfg(feature = "time")]
impl TryFrom<QDateTime> for time::PrimitiveDateTime {
    type Error = time::error::ComponentRange;

    fn try_from(value: QDateTime) -> Result<Self, Self::Error> {
        let value_utc = value.to_utc();
        Ok(time::Date::try_from(value_utc.date())?
            .with_time(time::Time::try_from(value_utc.time())?))
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QDateTime {
    type Id = type_id!("QDateTime");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ordering() {
        let qdatetime_a = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 1, 1),
            &QTime::new(1, 1, 1, 1),
            &ffi::QTimeZone::utc(),
        );
        let qdatetime_b = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 2, 2),
            &QTime::new(2, 2, 2, 2),
            &ffi::QTimeZone::utc(),
        );

        assert!(qdatetime_a < qdatetime_b);
        assert_eq!(qdatetime_a.cmp(&qdatetime_b), Ordering::Less);
        assert_eq!(qdatetime_b.cmp(&qdatetime_a), Ordering::Greater);
        assert_eq!(qdatetime_a.cmp(&qdatetime_a), Ordering::Equal);
    }
}

#[cfg(test)]
#[cfg(feature = "chrono")]
mod test_chrono {
    use super::*;

    #[test]
    fn qdatetime_from_chrono() {
        let datetime_east = {
            let timezone_east = chrono::FixedOffset::east_opt(60 * 60).unwrap();
            let naivedatetime_east = chrono::NaiveDate::from_ymd_opt(2023, 1, 1)
                .unwrap()
                .and_hms_milli_opt(1, 2, 3, 4)
                .unwrap();
            chrono::DateTime::<chrono::FixedOffset>::from_local(naivedatetime_east, timezone_east)
        };

        let qdatetime = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 1, 1),
            &QTime::new(1, 2, 3, 4),
            &ffi::QTimeZone::from_offset_seconds(60 * 60),
        );
        assert_eq!(QDateTime::from(datetime_east), qdatetime);
    }

    #[test]
    fn qdatetime_to_chrono_fixed_offset() {
        let datetime_east = {
            let timezone_east = chrono::FixedOffset::east_opt(60 * 60).unwrap();
            let naivedatetime_east = chrono::NaiveDate::from_ymd_opt(2023, 1, 1)
                .unwrap()
                .and_hms_milli_opt(1, 2, 3, 4)
                .unwrap();
            chrono::DateTime::<chrono::FixedOffset>::from_local(naivedatetime_east, timezone_east)
        };

        let qdatetime = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 1, 1),
            &QTime::new(1, 2, 3, 4),
            &ffi::QTimeZone::from_offset_seconds(60 * 60),
        );
        assert_eq!(
            chrono::DateTime::<chrono::FixedOffset>::try_from(qdatetime).unwrap(),
            datetime_east
        );
    }

    #[test]
    fn qdatetime_to_chrono_utc() {
        let datetime_utc = {
            let naivedatetime_utc = chrono::NaiveDate::from_ymd_opt(2023, 1, 1)
                .unwrap()
                .and_hms_milli_opt(1, 2, 3, 4)
                .unwrap();
            chrono::DateTime::<chrono::Utc>::from_utc(naivedatetime_utc, chrono::Utc)
        };

        let qdatetime = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 1, 1),
            &QTime::new(1, 2, 3, 4),
            &ffi::QTimeZone::utc(),
        );
        assert_eq!(
            chrono::DateTime::<chrono::Utc>::try_from(qdatetime).unwrap(),
            datetime_utc
        );
    }

    #[test]
    fn qdatetime_to_chrono_utc_with_offset() {
        let datetime_utc = {
            let naivedatetime_utc = chrono::NaiveDate::from_ymd_opt(2023, 1, 1)
                .unwrap()
                .and_hms_milli_opt(0, 2, 3, 4)
                .unwrap();
            chrono::DateTime::<chrono::Utc>::from_utc(naivedatetime_utc, chrono::Utc)
        };

        let qdatetime = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 1, 1),
            &QTime::new(1, 2, 3, 4),
            // Should cause one hour offset when in chrono::DateTime
            &ffi::QTimeZone::from_offset_seconds(60 * 60),
        );
        assert_eq!(
            chrono::DateTime::<chrono::Utc>::try_from(qdatetime).unwrap(),
            datetime_utc
        );
    }
}

#[cfg(test)]
#[cfg(feature = "time")]
mod test_time {
    use super::*;

    #[test]
    fn qdatetime_to_time_offsetdatetime() {
        let time_offsetdatetime = time::Date::from_calendar_date(2023, time::Month::January, 1)
            .unwrap()
            .with_hms_milli(1, 2, 3, 4)
            .unwrap()
            .assume_offset(time::UtcOffset::from_whole_seconds(60 * 60).unwrap());

        let qdatetime = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 1, 1),
            &QTime::new(1, 2, 3, 4),
            &ffi::QTimeZone::from_offset_seconds(60 * 60),
        );
        assert_eq!(
            time::OffsetDateTime::try_from(qdatetime).unwrap(),
            time_offsetdatetime
        );
    }

    #[test]
    fn qdatetime_to_time_primitivedatetime() {
        let time_offsetdatetime = time::Date::from_calendar_date(2023, time::Month::January, 1)
            .unwrap()
            .with_hms_milli(1, 2, 3, 4)
            .unwrap();

        let qdatetime = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 1, 1),
            &QTime::new(1, 2, 3, 4),
            &ffi::QTimeZone::utc(),
        );
        assert_eq!(
            time::PrimitiveDateTime::try_from(qdatetime).unwrap(),
            time_offsetdatetime
        );
    }

    #[test]
    fn qdatetime_from_time_offsetdatetime() {
        let time_offsetdatetime = time::Date::from_calendar_date(2023, time::Month::January, 1)
            .unwrap()
            .with_hms_milli(1, 2, 3, 4)
            .unwrap()
            .assume_offset(time::UtcOffset::from_whole_seconds(60 * 60).unwrap());

        let qdatetime = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 1, 1),
            &QTime::new(1, 2, 3, 4),
            &ffi::QTimeZone::from_offset_seconds(60 * 60),
        );
        assert_eq!(QDateTime::from(time_offsetdatetime), qdatetime);
    }

    #[test]
    fn qdatetime_from_time_primitivedatetime() {
        let time_offsetdatetime = time::Date::from_calendar_date(2023, time::Month::January, 1)
            .unwrap()
            .with_hms_milli(1, 2, 3, 4)
            .unwrap();

        let qdatetime = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 1, 1),
            &QTime::new(1, 2, 3, 4),
            &ffi::QTimeZone::utc(),
        );
        assert_eq!(QDateTime::from(time_offsetdatetime), qdatetime);
    }
}
