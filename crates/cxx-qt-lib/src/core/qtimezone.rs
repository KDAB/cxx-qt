// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use std::fmt;

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum QTimeZoneNameType {
        /// The default form of the time zone name, one of LongName, ShortName or OffsetName
        DefaultName,
        /// The long form of the time zone name, e.g. "Central European Time"
        LongName,
        /// The short form of the time zone name, usually an abbreviation, e.g. "CET", in locales
        /// that have one for the zone, otherwise a compact GMT-ofset form, e.g. "GMT+1"
        ShortName,
        /// The standard ISO offset form of the time zone name, e.g. "UTC+01:00"
        OffsetName,
    }

    #[repr(i32)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum QTimeZoneTimeType {
        /// The standard time in a time zone, i.e. when Daylight-Saving is not in effect. For
        /// example when formatting a display name this will show something like "Pacific Standard
        /// Time".
        StandardTime,
        /// A time when Daylight-Saving is in effect. For example when formatting a display name
        /// this will show something like "Pacific daylight-saving time".
        DaylightTime,
        /// A time which is not specifically Standard or Daylight-Saving time, either an unknown
        /// time or a neutral form. For example when formatting a display name this will show
        /// something like "Pacific Time".
        GenericTime,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = crate::QDateTime;
        include!("cxx-qt-lib/qlist_QByteArray.h");
        type QList_QByteArray = crate::QList<crate::QByteArray>;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qtimezone.h");
        /// The QTimeZone class converts between UTC and local time in a specific time zone.
        //
        // QTimeZone only has a copy-constructor and not a move-constructor, which means that the following is true
        // "When the move constructor is not implicitly declared or explicitly supplied, expressions
        // that otherwise would have invoked the move constructor may instead invoke a copy constructor."
        //
        // Therefore the internal QSharedDataPointer is incremented causing a memory leak, so use an opaque type.
        type QTimeZone;
        type QTimeZoneNameType;
        type QTimeZoneTimeType;

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
        #[rust_name = "qtimezone_clone"]
        fn qtimezoneClone(timezone: &QTimeZone) -> UniquePtr<QTimeZone>;
        #[doc(hidden)]
        #[rust_name = "qtimezone_default"]
        fn qtimezoneDefault() -> UniquePtr<QTimeZone>;
        #[doc(hidden)]
        #[rust_name = "qtimezone_display_name"]
        fn qtimezoneDisplayName(
            timezone: &QTimeZone,
            time_type: QTimeZoneTimeType,
            name_type: QTimeZoneNameType,
        ) -> QString;
        #[doc(hidden)]
        #[rust_name = "qtimezone_from_offset_seconds"]
        fn qtimezoneFromOffsetSeconds(offset_seconds: i32) -> UniquePtr<QTimeZone>;
        #[doc(hidden)]
        #[rust_name = "qtimezone_from_iana"]
        fn qtimezoneFromIana(iana_id: &QByteArray) -> UniquePtr<QTimeZone>;
        #[doc(hidden)]
        #[rust_name = "qtimezone_system_time_zone"]
        fn qtimezoneSystemTimeZone() -> UniquePtr<QTimeZone>;
        #[doc(hidden)]
        #[rust_name = "qtimezone_system_time_zone_id"]
        fn qtimezoneSystemTimeZoneId() -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "qtimezone_utc"]
        fn qtimezoneUtc() -> UniquePtr<QTimeZone>;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qtimezone_eq"]
        fn operatorEq(a: &QTimeZone, b: &QTimeZone) -> bool;
        #[doc(hidden)]
        #[rust_name = "qtimezone_to_debug_qstring"]
        fn toDebugQString(value: &QTimeZone) -> QString;
    }

    // QTimeZone only has a copy-constructor and not a move-constructor, which means that the following is true
    // "When the move constructor is not implicitly declared or explicitly supplied, expressions
    // that otherwise would have invoked the move constructor may instead invoke a copy constructor."
    //
    // Therefore the internal QSharedDataPointer is incremented causing a memory leak, so use an opaque type.
    impl UniquePtr<QTimeZone> {}
}

pub use ffi::{QTimeZone, QTimeZoneNameType, QTimeZoneTimeType};

impl Default for QTimeZoneNameType {
    fn default() -> Self {
        Self::DefaultName
    }
}

impl QTimeZone {
    /// Returns a list of all available IANA time zone IDs on this system.
    pub fn available_time_zone_ids() -> ffi::QList_QByteArray {
        ffi::qtimezone_available_time_zone_ids()
    }

    /// Returns the localized time zone display name.
    ///
    /// Where the time zone display names have changed over time, the current names will be used.
    /// If no suitably localized name of the given type is available, another name type may be
    /// used, or an empty string may be returned.
    ///
    /// For custom timezones created by client code, the data supplied to the constructor are
    /// used, as no localization data will be available for it. If this timezone is invalid, an
    /// empty string is returned. This may also arise for the representation of local time if
    /// determining the system time zone fails.
    fn display_name(
        &self,
        time_type: QTimeZoneTimeType,
        name_type: QTimeZoneNameType,
    ) -> ffi::QString {
        ffi::qtimezone_display_name(self, time_type, name_type)
    }

    /// Creates an instance of a time zone with the requested Offset from UTC of offsetSeconds.
    pub fn owned_from_offset_seconds(offset_seconds: i32) -> cxx::UniquePtr<Self> {
        ffi::qtimezone_from_offset_seconds(offset_seconds)
    }

    /// Creates an instance of the requested time zone ianaId.
    pub fn owned_from_iana(iana_id: &ffi::QByteArray) -> cxx::UniquePtr<Self> {
        ffi::qtimezone_from_iana(iana_id)
    }

    /// Create a null/invalid time zone instance.
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qtimezone_default()
    }

    /// Returns a QTimeZone object that refers to the local system time, as specified by systemTimeZoneId().
    pub fn system_time_zone() -> cxx::UniquePtr<Self> {
        ffi::qtimezone_system_time_zone()
    }

    /// Returns the current system time zone IANA ID.
    pub fn system_time_zone_id() -> ffi::QByteArray {
        ffi::qtimezone_system_time_zone_id()
    }

    /// Copy constructor, create a copy of the QTimeZone.
    pub fn to_owned(&self) -> cxx::UniquePtr<Self> {
        ffi::qtimezone_clone(self)
    }

    /// Returns a QTimeZone object that refers to UTC (Universal Time Coordinated).
    pub fn utc() -> cxx::UniquePtr<Self> {
        ffi::qtimezone_utc()
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
        self.display_name(
            QTimeZoneTimeType::GenericTime,
            QTimeZoneNameType::DefaultName,
        )
        .fmt(f)
    }
}

impl fmt::Debug for QTimeZone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qtimezone_to_debug_qstring(self).fmt(f)
    }
}
