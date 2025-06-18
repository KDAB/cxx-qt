// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;
use std::{cmp::Ordering, fmt};

#[allow(unused)]
use crate::TimeSpec;
use crate::{DateFormat, QDate, QString, QTime, QTimeZone};

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type TimeSpec = crate::TimeSpec;
        type DateFormat = crate::DateFormat;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qtypes.h");
        type qint64 = crate::qint64;
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

        #[doc(hidden)]
        #[rust_name = "add_days_qint64"]
        fn addDays(self: &QDateTime, ndays: qint64) -> QDateTime;

        /// Returns a QDateTime object containing a datetime `nmonths` months later than the datetime of this object (or earlier if `nmonths` is negative).
        ///
        /// If [`time_spec`](Self::time_spec) is [`TimeSpec::LocalTime`] or [`TimeSpec::TimeZone`] and the resulting date and time fall in the Standard Time to Daylight-Saving Time transition hour then the result will be just beyond this gap, in the direction of change. If the transition is at 2am and the clock goes forward to 3am, the result of aiming between 2am and 3am will be adjusted to fall before 2am (if `nmonths` is negative) or after 3am (otherwise).
        #[rust_name = "add_months"]
        fn addMonths(self: &QDateTime, nmonths: i32) -> QDateTime;

        #[doc(hidden)]
        #[rust_name = "add_msecs_qint64"]
        fn addMSecs(self: &QDateTime, msecs: qint64) -> QDateTime;

        #[doc(hidden)]
        #[rust_name = "add_secs_qint64"]
        fn addSecs(self: &QDateTime, secs: qint64) -> QDateTime;

        /// Returns a QDateTime object containing a datetime `nyears` years later than the datetime of this object (or earlier if `nyears` is negative).
        ///
        /// If [`time_spec`](Self::time_spec) is [`TimeSpec::LocalTime`] or [`TimeSpec::TimeZone`] and the resulting date and time fall in the Standard Time to Daylight-Saving Time transition hour then the result will be just beyond this gap, in the direction of change. If the transition is at 2am and the clock goes forward to 3am, the result of aiming between 2am and 3am will be adjusted to fall before 2am (if `nyears` is negative) or after 3am (otherwise).
        #[rust_name = "add_years"]
        fn addYears(self: &QDateTime, nyears: i32) -> QDateTime;

        /// Returns the date part of the datetime.
        fn date(self: &QDateTime) -> QDate;

        #[doc(hidden)]
        #[rust_name = "days_to_qint64"]
        fn daysTo(self: &QDateTime, other: &QDateTime) -> qint64;

        /// Returns `true` if this datetime falls in Daylight-Saving Time, otherwise `false`.
        ///
        /// If [`time_spec`](Self::time_spec) is not [`TimeSpec::LocalTime`] or [`TimeSpec::TimeZone`] then this will always return `false`.
        #[rust_name = "is_daylight_time"]
        fn isDaylightTime(self: &QDateTime) -> bool;

        /// Returns `true` if both the date and the time are null; otherwise returns `false`. A null datetime is invalid.
        #[rust_name = "is_null"]
        fn isNull(self: &QDateTime) -> bool;

        /// Returns `true` if both the `date` and the `time` are valid and they are valid in the current [`TimeSpec`](TimeSpec), otherwise returns `false`.
        #[rust_name = "is_valid"]
        fn isValid(self: &QDateTime) -> bool;

        /// Returns this date-time's offset from UTC in seconds.
        #[rust_name = "offset_from_utc"]
        fn offsetFromUtc(self: &QDateTime) -> i32;

        #[doc(hidden)]
        #[rust_name = "msecs_to_qint64"]
        fn msecsTo(self: &QDateTime, other: &QDateTime) -> qint64;

        #[doc(hidden)]
        #[rust_name = "secs_to_qint64"]
        fn secsTo(self: &QDateTime, other: &QDateTime) -> qint64;

        #[doc(hidden)]
        #[rust_name = "set_msecs_since_epoch_qint64"]
        fn setMSecsSinceEpoch(self: &mut QDateTime, msecs: qint64);

        /// Sets the [`time_spec`](Self::time_spec) to [`TimeSpec::OffsetFromUTC`] and the offset to `offset_seconds`.
        ///
        /// **Note:** This method is only available with Qt < 6.8.
        #[cfg(not(cxxqt_qt_version_at_least_6_8))]
        #[rust_name = "set_offset_from_utc"]
        fn setOffsetFromUtc(self: &mut QDateTime, offset_seconds: i32);

        #[doc(hidden)]
        #[rust_name = "set_secs_since_epoch_qint64"]
        fn setSecsSinceEpoch(self: &mut QDateTime, secs: qint64);

        /// Sets the time specification used in this datetime to `spec`. The datetime will refer to a different point in time.
        ///
        /// **Note:** This method is only available with Qt < 6.8.
        #[cfg(not(cxxqt_qt_version_at_least_6_8))]
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

        /// Returns a copy of this datetime converted to local time.
        ///
        /// The result represents the same moment in time as, and is equal to, this datetime.
        #[rust_name = "to_local_time"]
        fn toLocalTime(self: &QDateTime) -> QDateTime;

        #[doc(hidden)]
        #[rust_name = "to_msecs_since_epoch_qint64"]
        fn toMSecsSinceEpoch(self: &QDateTime) -> qint64;

        /// Returns a copy of this datetime converted to a spec of [`TimeSpec::OffsetFromUTC`] with the given `offset_seconds`.
        ///
        /// If the `offset_seconds` equals 0 then a UTC datetime will be returned.
        ///
        /// The result represents the same moment in time as, and is equal to, this datetime.
        #[rust_name = "to_offset_from_utc"]
        fn toOffsetFromUtc(self: &QDateTime, offset_seconds: i32) -> QDateTime;

        #[doc(hidden)]
        #[rust_name = "to_secs_since_epoch_qint64"]
        fn toSecsSinceEpoch(self: &QDateTime) -> qint64;

        /// Returns the time as a string in the `format` given.
        #[rust_name = "format_enum"]
        fn toString(self: &QDateTime, format: DateFormat) -> QString;

        /// Returns a copy of this datetime converted to the given time `spec`.
        ///
        /// Note this method is only available with Qt < 6.8
        #[cfg(not(cxxqt_qt_version_at_least_6_8))]
        #[rust_name = "to_time_spec"]
        fn toTimeSpec(self: &QDateTime, spec: TimeSpec) -> QDateTime;

        /// Returns a copy of this datetime converted to the given `time_zone`.
        ///
        /// The result represents the same moment in time as, and is equal to, this datetime.
        ///
        /// If `time_zone` is invalid then the datetime will be invalid.
        #[rust_name = "to_time_zone"]
        fn toTimeZone(self: &QDateTime, time_zone: &QTimeZone) -> QDateTime;

        /// Returns a copy of this datetime converted to UTC.
        ///
        /// The result represents the same moment in time as, and is equal to, this datetime.
        #[rust_name = "to_utc"]
        fn toUTC(self: &QDateTime) -> QDateTime;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qdatetime_current_date_time"]
        fn qdatetimeCurrentDateTime() -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_current_date_time_utc"]
        fn qdatetimeCurrentDateTimeUtc() -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_current_msecs_since_epoch"]
        fn qdatetimeCurrentMSecsSinceEpoch() -> qint64;
        #[doc(hidden)]
        #[rust_name = "qdatetime_current_secs_since_epoch"]
        fn qdatetimeCurrentSecsSinceEpoch() -> qint64;
        #[doc(hidden)]
        #[rust_name = "qdatetime_from_msecs_since_epoch"]
        fn qdatetimeFromMSecsSinceEpoch(msecs: qint64, time_zone: &QTimeZone) -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qdatetime_from_secs_since_epoch"]
        fn qdatetimeFromSecsSinceEpoch(secs: qint64, time_zone: &QTimeZone) -> QDateTime;
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
        #[rust_name = "qdatetime_time_zone"]
        fn qdatetimeTimeZone(datetime: &QDateTime) -> UniquePtr<QTimeZone>;
        #[rust_name = "qdatetime_settimezone"]
        fn qdatetimeSetTimeZone(datetime: &mut QDateTime, time_zone: &QTimeZone);
        #[rust_name = "qdatetime_from_string"]
        fn qdatetimeFromQString(string: &QString, format: DateFormat) -> QDateTime;
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
        #[cfg(not(cxxqt_qt_version_at_least_6_8))]
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
        #[rust_name = "qdatetime_to_debug_qstring"]
        fn toDebugQString(value: &QDateTime) -> QString;
    }
}

/// The `QDateTime` class provides date and time functions.
///
/// Qt Documentation: [QDateTime](https://doc.qt.io/qt/qdatetime.html#details)
#[repr(C)]
pub struct QDateTime {
    _space: MaybeUninit<usize>,
}

impl QDateTime {
    /// Sets the time zone used in this datetime to `time_zone`.
    ///
    /// The datetime may refer to a different point in time. It uses the time representation of `time_zone`, which may change the meaning of its unchanged [`date`](Self::date) and [`time`](Self::time).
    ///
    /// If `time_zone` is invalid then the datetime will be invalid.
    pub fn set_time_zone(&mut self, time_zone: &QTimeZone) {
        ffi::qdatetime_settimezone(self, time_zone)
    }

    /// Returns a `QDateTime` object containing a datetime `ndays` days later than the datetime of this object (or earlier if `ndays` is negative).
    ///
    /// If [`time_spec`](Self::time_spec) is [`TimeSpec::LocalTime`] or [`TimeSpec::TimeZone`] and the resulting date and time fall in the Standard Time to Daylight-Saving Time transition hour then the result will be just beyond this gap, in the direction of change. If the transition is at 2am and the clock goes forward to 3am, the result of aiming between 2am and 3am will be adjusted to fall before 2am (if `ndays` is negative) or after 3am (otherwise).
    pub fn add_days(self: &QDateTime, ndays: i64) -> QDateTime {
        self.add_days_qint64(ndays.into())
    }

    /// Returns a `QDateTime` object containing a datetime `msecs` milliseconds later than the datetime of this object (or earlier if `msecs` is negative).
    ///
    /// If this datetime is invalid, an invalid datetime will be returned.
    pub fn add_msecs(self: &QDateTime, msecs: i64) -> QDateTime {
        self.add_msecs_qint64(msecs.into())
    }

    /// Returns a `QDateTime` object containing a datetime `secs` seconds later than the datetime of this object (or earlier if `secs` is negative).
    ///
    /// If this datetime is invalid, an invalid datetime will be returned.
    pub fn add_secs(self: &QDateTime, secs: i64) -> QDateTime {
        self.add_secs_qint64(secs.into())
    }

    /// Returns the system clock's current datetime, using local time.
    pub fn current_date_time() -> Self {
        ffi::qdatetime_current_date_time()
    }

    /// Returns the system clock's current datetime, expressed in terms of UTC.
    pub fn current_date_time_utc() -> Self {
        ffi::qdatetime_current_date_time_utc()
    }

    /// Returns the current number of milliseconds since the start, in UTC, of the year 1970.
    ///
    /// This number is like the POSIX `time_t` variable, but expressed in milliseconds instead.
    pub fn current_msecs_since_epoch() -> i64 {
        ffi::qdatetime_current_msecs_since_epoch().into()
    }

    /// Returns the number of seconds since the start, in UTC, of the year 1970.
    ///
    /// This number is like the POSIX `time_t` variable.
    pub fn current_secs_since_epoch() -> i64 {
        ffi::qdatetime_current_secs_since_epoch().into()
    }

    /// Returns the number of days from this datetime to the `other` datetime.
    /// The number of days is counted as the number of times midnight is reached between this datetime to the other datetime.
    /// This means that a 10 minute difference from 23:55 to 0:05 the next day counts as one day.
    ///
    /// If the `other` datetime is earlier than this datetime, the value returned is negative.
    pub fn days_to(&self, other: &Self) -> i64 {
        self.days_to_qint64(other).into()
    }

    /// Constructs a datetime with the given `date` and `time`, using the time representation described by `time_zone`.
    ///
    /// If `date` is valid and `time` is not, the time will be set to midnight. If `time_zone` is invalid then the datetime will be invalid.
    pub fn from_date_and_time_time_zone(date: &QDate, time: &QTime, time_zone: &QTimeZone) -> Self {
        ffi::qdatetime_init_from_date_and_time_time_zone(date, time, time_zone)
    }

    /// Constructs a datetime with the given `date` and `time`, using the time specification defined by `spec` and `offset_seconds` seconds.
    ///
    /// If `date` is valid and `time` is not, the time will be set to midnight.
    ///
    /// If the spec is not [`TimeSpec::OffsetFromUTC`] then `offset_seconds` will be ignored.
    ///
    /// If the spec is [`TimeSpec::OffsetFromUTC`] and `offset_seconds` is 0 then the time spec will be set to [`TimeSpec::UTC`], i.e. an offset of 0 seconds.
    ///
    /// If spec is [`TimeSpec:TimeZone`] then the spec will be set to [`TimeSpec::LocalTime`], i.e. the current system time zone.
    ///
    /// **Note:** This method is only available with Qt < 6.8.
    #[cfg(not(cxxqt_qt_version_at_least_6_8))]
    pub fn from_date_and_time_time_spec(
        date: &QDate,
        time: &QTime,
        time_spec: TimeSpec,
        offset_seconds: i32,
    ) -> Self {
        ffi::qdatetime_init_from_date_and_time_time_spec(date, time, time_spec, offset_seconds)
    }

    /// Returns a datetime representing a moment the given number `msecs` of milliseconds after the start, in UTC, of the year 1970, described as specified by `time_zone`.
    ///
    /// Note that there are possible values for `msecs` that lie outside the valid range of `QDateTime`, both negative and positive. The behavior of this function is undefined for those values.
    pub fn from_msecs_since_epoch(msecs: i64, time_zone: &QTimeZone) -> Self {
        ffi::qdatetime_from_msecs_since_epoch(msecs.into(), time_zone)
    }

    /// Returns a datetime representing a moment the given number `secs` of seconds after the start, in UTC, of the year 1970, described as specified by `time_zone`.
    ///
    /// Note that there are possible values for `secs` that lie outside the valid range of `QDateTime`, both negative and positive. The behavior of this function is undefined for those values.
    pub fn from_secs_since_epoch(secs: i64, time_zone: &QTimeZone) -> Self {
        ffi::qdatetime_from_secs_since_epoch(secs.into(), time_zone)
    }

    /// Returns the datetime represented by the `string`, using the `format` given, or `None` if this is not possible.
    ///
    /// Note for [`DateFormat::TextDate`]: only English month names (e.g. "Jan" in short form or "January" in long form) are recognized.
    pub fn from_string(string: &QString, format: DateFormat) -> Option<Self> {
        let date = ffi::qdatetime_from_string(string, format);
        if date.is_valid() {
            Some(date)
        } else {
            None
        }
    }

    /// Returns the number of milliseconds from this datetime to the `other` datetime.
    /// If the `other` datetime is earlier than this datetime, the value returned is negative.
    ///
    /// Returns 0 if either datetime is invalid.
    pub fn msecs_to(self: &QDateTime, other: &QDateTime) -> i64 {
        self.msecs_to_qint64(other).into()
    }

    /// Returns the number of seconds from this datetime to the `other` datetime.
    /// If the `other` datetime is earlier than this datetime, the value returned is negative.
    ///
    /// Returns 0 if either datetime is invalid.
    pub fn secs_to(self: &QDateTime, other: &QDateTime) -> i64 {
        self.secs_to_qint64(other).into()
    }

    /// Sets the date part of this datetime to `date`. If no time is set yet, it is set to midnight.
    /// If `date` is invalid, this `QDateTime` becomes invalid.
    pub fn set_date(&mut self, date: QDate) {
        ffi::qdatetime_set_date(self, date);
    }

    /// Sets the datetime to represent a moment a given number, `msecs`, of milliseconds after the start, in UTC, of the year 1970.
    ///
    /// On systems that do not support time zones, this function will behave as if local time were UTC.
    ///
    /// Note that passing `i64::MIN` to `msecs` will result in undefined behavior.
    pub fn set_msecs_since_epoch(self: &mut QDateTime, msecs: i64) {
        self.set_msecs_since_epoch_qint64(msecs.into());
    }

    /// Sets the datetime to represent a moment a given number, `secs`, of seconds after the start, in UTC, of the year 1970.
    ///
    /// On systems that do not support time zones, this function will behave as if local time were UTC.
    pub fn set_secs_since_epoch(self: &mut QDateTime, secs: i64) {
        self.set_secs_since_epoch_qint64(secs.into());
    }

    /// Sets the time part of this datetime to `time`. If `time` is not valid, this function sets it to midnight.
    /// Therefore, it's possible to clear any set time in a `QDateTime` by setting it to [`QTime::default()`].
    pub fn set_time(&mut self, time: QTime) {
        ffi::qdatetime_set_time(self, time);
    }

    /// Returns the time zone of the datetime.
    pub fn time_zone(&self) -> cxx::UniquePtr<QTimeZone> {
        ffi::qdatetime_time_zone(self)
    }

    /// Returns the datetime as a number of milliseconds after the start, in UTC, of the year 1970.
    ///
    /// On systems that do not support time zones, this function will behave as if local time were UTC.
    ///
    /// The behavior for this function is undefined if the datetime stored in this object is not valid. However, for all valid dates, this function returns a unique value.
    pub fn to_msecs_since_epoch(self: &QDateTime) -> i64 {
        self.to_msecs_since_epoch_qint64().into()
    }

    /// Returns the datetime as a number of seconds after the start, in UTC, of the year 1970.
    ///
    /// On systems that do not support time zones, this function will behave as if local time were UTC.
    ///
    /// The behavior for this function is undefined if the datetime stored in this object is not valid. However, for all valid dates, this function returns a unique value.
    pub fn to_secs_since_epoch(self: &QDateTime) -> i64 {
        self.to_secs_since_epoch_qint64().into()
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
        self.format_enum(DateFormat::TextDate).fmt(f)
    }
}

impl fmt::Debug for QDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qdatetime_to_debug_qstring(self).fmt(f)
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
impl<Tz: chrono::TimeZone> TryFrom<chrono::DateTime<Tz>> for QDateTime {
    type Error = &'static str;

    fn try_from(value: chrono::DateTime<Tz>) -> Result<Self, Self::Error> {
        let tz = crate::QTimeZone::from_offset_seconds(value.offset().fix().local_minus_utc());
        Ok(QDateTime::from_date_and_time_time_zone(
            &QDate::from(value.date_naive()),
            &QTime::try_from(value.time())?,
            tz.as_ref().ok_or("Could not construct timezone")?,
        ))
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<QDateTime> for chrono::DateTime<chrono::FixedOffset> {
    type Error = &'static str;

    fn try_from(value: QDateTime) -> Result<Self, Self::Error> {
        let timezone_east = chrono::FixedOffset::east_opt(value.offset_from_utc())
            .expect("out-of-bound offset secs");
        let value_utc = value.to_utc();
        let naivedatetime_east = chrono::NaiveDate::try_from(value_utc.date())?
            .and_time(chrono::NaiveTime::try_from(value_utc.time())?);
        Ok(
            chrono::DateTime::<chrono::FixedOffset>::from_naive_utc_and_offset(
                naivedatetime_east,
                timezone_east,
            ),
        )
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<QDateTime> for chrono::DateTime<chrono::Utc> {
    type Error = &'static str;

    fn try_from(value: QDateTime) -> Result<Self, Self::Error> {
        let value_utc = value.to_utc();
        let naivedatetime_utc = chrono::NaiveDate::try_from(value_utc.date())?
            .and_time(chrono::NaiveTime::try_from(value_utc.time())?);
        Ok(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
            naivedatetime_utc,
            chrono::Utc,
        ))
    }
}

#[cfg(feature = "time")]
impl From<time::OffsetDateTime> for QDateTime {
    fn from(value: time::OffsetDateTime) -> Self {
        let tz = crate::QTimeZone::from_offset_seconds(value.offset().whole_seconds());
        QDateTime::from_date_and_time_time_zone(
            &QDate::from(value.date()),
            &QTime::from(value.time()),
            tz.as_ref().expect("Could not construct timezone"),
        )
    }
}

#[cfg(feature = "time")]
impl From<time::PrimitiveDateTime> for QDateTime {
    fn from(value: time::PrimitiveDateTime) -> Self {
        let tz = crate::QTimeZone::utc();
        QDateTime::from_date_and_time_time_zone(
            &QDate::from(value.date()),
            &QTime::from(value.time()),
            tz.as_ref().expect("Could not construct timezone"),
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
            &QTimeZone::utc(),
        );
        let qdatetime_b = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 2, 2),
            &QTime::new(2, 2, 2, 2),
            &QTimeZone::utc(),
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
            chrono::DateTime::<chrono::FixedOffset>::from_naive_utc_and_offset(
                naivedatetime_east,
                timezone_east,
            )
        };

        let qdatetime = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 1, 1),
            // Chrono adds the offset to the given time, so add the offset here to match Chrono
            &QTime::new(1 + 1 /* plus the offset */, 2, 3, 4),
            &QTimeZone::from_offset_seconds(60 * 60),
        );
        assert_eq!(QDateTime::try_from(datetime_east).unwrap(), qdatetime);
    }

    #[test]
    fn qdatetime_to_chrono_fixed_offset() {
        let datetime_east = {
            let timezone_east = chrono::FixedOffset::east_opt(60 * 60).unwrap();
            let naivedatetime_east = chrono::NaiveDate::from_ymd_opt(2023, 1, 1)
                .unwrap()
                // Chrono adds the offset to the given time, so minus the offset here to match Qt
                .and_hms_milli_opt(1 - 1 /* minus the offset */, 2, 3, 4)
                .unwrap();
            chrono::DateTime::<chrono::FixedOffset>::from_naive_utc_and_offset(
                naivedatetime_east,
                timezone_east,
            )
        };

        let qdatetime = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 1, 1),
            &QTime::new(1, 2, 3, 4),
            &QTimeZone::from_offset_seconds(60 * 60),
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
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                naivedatetime_utc,
                chrono::Utc,
            )
        };

        let qdatetime = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 1, 1),
            &QTime::new(1, 2, 3, 4),
            &QTimeZone::utc(),
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
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                naivedatetime_utc,
                chrono::Utc,
            )
        };

        let qdatetime = QDateTime::from_date_and_time_time_zone(
            &QDate::new(2023, 1, 1),
            &QTime::new(1, 2, 3, 4),
            // Should cause one hour offset when in chrono::DateTime
            &QTimeZone::from_offset_seconds(60 * 60),
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
            &QTimeZone::from_offset_seconds(60 * 60),
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
            &QTimeZone::utc(),
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
            &QTimeZone::from_offset_seconds(60 * 60),
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
            &QTimeZone::utc(),
        );
        assert_eq!(QDateTime::from(time_offsetdatetime), qdatetime);
    }
}
