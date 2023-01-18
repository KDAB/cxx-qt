// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::fmt;
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = crate::QDateTime;
        include!("cxx-qt-lib/qlist.h");
        type QList_QByteArray = crate::QList<crate::QByteArray>;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qtimezone.h");
        type QTimeZone = super::QTimeZone;

        /// Returns the time zone abbreviation at the given atDateTime. The abbreviation may change depending on DST or even historical events.
        fn abbreviation(self: &QTimeZone, atDateTime: &QDateTime) -> QString;

        /// Returns any comment for the time zone.
        fn comment(self: &QTimeZone) -> QString;

        /// Returns the daylight-saving time offset at the given atDateTime,
        /// i.e. the number of seconds to add to the standard time offset to obtain the local daylight-saving time.
        #[rust_name = "daylight_time_offset"]
        fn daylightTimeOffset(self: &QTimeZone, atDateTime: &QDateTime) -> i32;

        /// Returns true if the time zone has practiced daylight-saving at any time.
        #[rust_name = "has_daylight_time"]
        fn hasDaylightTime(self: &QTimeZone) -> bool;

        /// Returns true if the system backend supports obtaining transitions.
        #[rust_name = "has_transitions"]
        fn hasTransitions(self: &QTimeZone) -> bool;

        /// Returns the IANA ID for the time zone.
        fn id(self: &QTimeZone) -> QByteArray;

        /// Returns true if daylight-saving was in effect at the given atDateTime.
        #[rust_name = "is_daylight_time"]
        fn isDaylightTime(self: &QTimeZone, atDateTime: &QDateTime) -> bool;

        /// Returns true if this time zone is valid.
        #[rust_name = "is_valid"]
        fn isValid(self: &QTimeZone) -> bool;

        /// Returns the total effective offset at the given atDateTime, i.e. the number of seconds to add to UTC to obtain the local time.
        /// This includes any DST offset that may be in effect, i.e. it is the sum of standardTimeOffset() and daylightTimeOffset() for the given datetime.
        #[rust_name = "offset_from_utc"]
        fn offsetFromUtc(self: &QTimeZone, atDateTime: &QDateTime) -> i32;

        /// Returns the standard time offset at the given atDateTime, i.e. the number of seconds to add to UTC to obtain the local Standard Time.
        /// This excludes any DST offset that may be in effect.
        #[rust_name = "standard_time_offset"]
        fn standardTimeOffset(self: &QTimeZone, atDateTime: &QDateTime) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qtimezone_available_time_zone_ids"]
        fn qtimezoneAvailableTimeZoneIds() -> QList_QByteArray;
        #[doc(hidden)]
        #[rust_name = "qtimezone_from_offset_seconds"]
        fn qtimezoneFromOffsetSeconds(offset_seconds: i32) -> QTimeZone;
        #[doc(hidden)]
        #[rust_name = "qtimezone_from_iana"]
        fn qtimezoneFromIana(iana_id: &QByteArray) -> QTimeZone;
        #[doc(hidden)]
        #[rust_name = "qtimezone_system_time_zone"]
        fn qtimezoneSystemTimeZone() -> QTimeZone;
        #[doc(hidden)]
        #[rust_name = "qtimezone_system_time_zone_id"]
        fn qtimezoneSystemTimeZoneId() -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "qtimezone_utc"]
        fn qtimezoneUtc() -> QTimeZone;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qtimezone_drop"]
        fn drop(timezone: &mut QTimeZone);

        #[doc(hidden)]
        #[rust_name = "qtimezone_default"]
        fn construct() -> QTimeZone;
        #[doc(hidden)]
        #[rust_name = "qtimezone_clone"]
        fn construct(other: &QTimeZone) -> QTimeZone;
        #[doc(hidden)]
        #[rust_name = "qtimezone_eq"]
        fn operatorEq(a: &QTimeZone, b: &QTimeZone) -> bool;
        #[doc(hidden)]
        #[rust_name = "qtimezone_to_qstring"]
        fn toQString(value: &QTimeZone) -> QString;
    }
}

/// The QTimeZone class converts between UTC and local time in a specific time zone.
#[repr(C)]
pub struct QTimeZone {
    _space: MaybeUninit<usize>,
}

impl QTimeZone {
    /// Returns a list of all available IANA time zone IDs on this system.
    pub fn available_time_zone_ids() -> ffi::QList_QByteArray {
        ffi::qtimezone_available_time_zone_ids()
    }

    /// Creates an instance of a time zone with the requested Offset from UTC of offsetSeconds.
    pub fn from_offset_seconds(offset_seconds: i32) -> Self {
        ffi::qtimezone_from_offset_seconds(offset_seconds)
    }

    /// Creates an instance of the requested time zone ianaId.
    pub fn from_iana(iana_id: &ffi::QByteArray) -> Self {
        ffi::qtimezone_from_iana(iana_id)
    }

    /// Returns a QTimeZone object that refers to the local system time, as specified by systemTimeZoneId().
    pub fn system_time_zone() -> Self {
        ffi::qtimezone_system_time_zone()
    }

    /// Returns the current system time zone IANA ID.
    pub fn system_time_zone_id() -> ffi::QByteArray {
        ffi::qtimezone_system_time_zone_id()
    }

    /// Returns a QTimeZone object that refers to UTC (Universal Time Coordinated).
    pub fn utc() -> Self {
        ffi::qtimezone_utc()
    }
}

impl Clone for QTimeZone {
    /// Copy constructor, copy other to this.
    fn clone(&self) -> Self {
        ffi::qtimezone_clone(self)
    }
}

impl Default for QTimeZone {
    /// Create a null/invalid time zone instance.
    fn default() -> Self {
        ffi::qtimezone_default()
    }
}

impl Drop for QTimeZone {
    /// Destroys the time zone.
    fn drop(&mut self) {
        ffi::qtimezone_drop(self)
    }
}

impl std::cmp::PartialEq for QTimeZone {
    fn eq(&self, other: &Self) -> bool {
        ffi::qtimezone_eq(self, other)
    }
}

impl std::cmp::Eq for QTimeZone {}

impl fmt::Display for QTimeZone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qtimezone_to_qstring(self))
    }
}

impl fmt::Debug for QTimeZone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QTimeZone {
    type Id = type_id!("QTimeZone");
    type Kind = cxx::kind::Trivial;
}
