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

#[cfg(feature = "serde")]
use serde::ser::SerializeMap;

#[cfg(feature = "serde")]
struct QDateVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for QDateVisitor {
    type Value = QDate;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("QDate")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut year = None;
        let mut month = None;
        let mut day = None;

        while let Some((key, value)) = map.next_entry()? {
            match key {
                "year" => year = Some(value),
                "month" => month = Some(value),
                "day" => day = Some(value),
                others => {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(others),
                        &"expected either year, month, or day as a key",
                    ));
                }
            }
        }

        if let (Some(year), Some(month), Some(day)) = (year, month, day) {
            Ok(QDate::new(year, month, day))
        } else {
            Err(serde::de::Error::missing_field(
                "missing year, month, or day as key",
            ))
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(QDateVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for QDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("year", &self.year())?;
        map.serialize_entry("month", &self.month())?;
        map.serialize_entry("day", &self.day())?;
        map.end()
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    fn test_serde_deserialize() {
        let test_data: QDate = serde_json::from_str(r#"{"year":2023,"month":1,"day":1}"#).unwrap();
        assert_eq!(test_data, QDate::new(2023, 1, 1));
    }

    #[test]
    fn test_serde_serialize() {
        let test_data = QDate::new(2023, 1, 1);
        let data_string = serde_json::to_string(&test_data).unwrap();
        assert_eq!(data_string, r#"{"year":2023,"month":1,"day":1}"#);
    }
}
