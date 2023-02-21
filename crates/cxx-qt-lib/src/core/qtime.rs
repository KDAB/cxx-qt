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
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[cfg(feature = "serde")]
use serde::ser::SerializeMap;

#[cfg(feature = "serde")]
struct QTimeVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for QTimeVisitor {
    type Value = QTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("QTime")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut hour = None;
        let mut minute = None;
        let mut second = None;
        let mut msec = None;

        while let Some((key, value)) = map.next_entry()? {
            match key {
                "hour" => hour = Some(value),
                "minute" => minute = Some(value),
                "second" => second = Some(value),
                "msec" => msec = Some(value),
                others => {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(others),
                        &"expected either hour, minute, second, or msec as a key",
                    ));
                }
            }
        }

        if let (Some(hour), Some(minute), Some(second), Some(msec)) = (hour, minute, second, msec) {
            Ok(QTime::new(hour, minute, second, msec))
        } else {
            Err(serde::de::Error::missing_field(
                "missing hour, minute, second, or msec as key",
            ))
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(QTimeVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for QTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("hour", &self.hour())?;
        map.serialize_entry("minute", &self.minute())?;
        map.serialize_entry("second", &self.second())?;
        map.serialize_entry("msec", &self.msec())?;
        map.end()
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    fn test_serde_deserialize() {
        let test_data: QTime =
            serde_json::from_str(r#"{"hour":1,"minute":2,"second":3,"msec":4}"#).unwrap();
        assert_eq!(test_data, QTime::new(1, 2, 3, 4));
    }

    #[test]
    fn test_serde_serialize() {
        let test_data = QTime::new(1, 2, 3, 4);
        let data_string = serde_json::to_string(&test_data).unwrap();
        assert_eq!(data_string, r#"{"hour":1,"minute":2,"second":3,"msec":4}"#);
    }
}
