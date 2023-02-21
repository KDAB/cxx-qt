// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qsize.h");
        include!("cxx-qt-lib/qstring.h");

        type QSize = super::QSize;
        type QString = crate::QString;

        /// Returns the height.
        fn height(self: &QSize) -> i32;
        /// Returns the width.
        fn width(self: &QSize) -> i32;

        /// Sets the height to the given height.
        #[rust_name = "set_height"]
        fn setHeight(self: &mut QSize, h: i32);
        /// Sets the width to the given width.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QSize, w: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qsize_init_default"]
        fn construct() -> QSize;
        #[doc(hidden)]
        #[rust_name = "qsize_init"]
        fn construct(w: i32, h: i32) -> QSize;
        #[doc(hidden)]
        #[rust_name = "qsize_to_qstring"]
        fn toQString(value: &QSize) -> QString;
        #[doc(hidden)]
        #[rust_name = "qsize_plus"]
        fn operatorPlus(a: &QSize, b: &QSize) -> QSize;
        #[doc(hidden)]
        #[rust_name = "qsize_minus"]
        fn operatorMinus(a: &QSize, b: &QSize) -> QSize;
        #[doc(hidden)]
        #[rust_name = "qsize_mul"]
        fn operatorMul(a: f64, b: &QSize) -> QSize;
        #[doc(hidden)]
        #[rust_name = "qsize_div"]
        fn operatorDiv(a: f64, b: &QSize) -> QSize;
    }
}

/// The QSize struct defines the size of a two-dimensional object using integer point precision.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct QSize {
    w: i32,
    h: i32,
}

impl QSize {
    /// Constructs a size with the given width and height.
    pub fn new(width: i32, height: i32) -> Self {
        ffi::qsize_init(width, height)
    }
}

impl Default for QSize {
    /// Constructs a size with an invalid width and height
    fn default() -> Self {
        ffi::qsize_init_default()
    }
}

impl fmt::Display for QSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qsize_to_qstring(self))
    }
}

impl std::ops::Add for QSize {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ffi::qsize_plus(&self, &other)
    }
}

impl std::ops::Sub for QSize {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ffi::qsize_minus(&self, &other)
    }
}

impl std::ops::Mul<f64> for QSize {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        ffi::qsize_mul(rhs, &self)
    }
}

impl std::ops::Div<f64> for QSize {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        ffi::qsize_div(rhs, &self)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QSize is trivial.
unsafe impl ExternType for QSize {
    type Id = type_id!("QSize");
    type Kind = cxx::kind::Trivial;
}

#[cfg(feature = "serde")]
use serde::ser::SerializeMap;

#[cfg(feature = "serde")]
struct QSizeVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for QSizeVisitor {
    type Value = QSize;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("QSize")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut width = None;
        let mut height = None;

        while let Some((key, value)) = map.next_entry()? {
            match key {
                "width" => width = Some(value),
                "height" => height = Some(value),
                others => {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(others),
                        &"expected either width or height as a key",
                    ));
                }
            }
        }

        if let (Some(width), Some(height)) = (width, height) {
            Ok(QSize::new(width, height))
        } else {
            Err(serde::de::Error::missing_field(
                "missing width or height as key",
            ))
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(QSizeVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for QSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("width", &self.width())?;
        map.serialize_entry("height", &self.height())?;
        map.end()
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    fn test_serde_deserialize() {
        let test_data: QSize = serde_json::from_str(r#"{"width":1,"height":2}"#).unwrap();
        assert_eq!(test_data, QSize::new(1, 2));
    }

    #[test]
    fn test_serde_serialize() {
        let test_data = QSize::new(1, 2);
        let data_string = serde_json::to_string(&test_data).unwrap();
        assert_eq!(data_string, r#"{"width":1,"height":2}"#);
    }
}
