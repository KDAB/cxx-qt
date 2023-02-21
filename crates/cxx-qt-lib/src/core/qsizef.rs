// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qsizef.h");
        include!("cxx-qt-lib/qstring.h");

        type QSizeF = super::QSizeF;
        type QString = crate::QString;

        /// Returns the height.
        fn height(self: &QSizeF) -> f64;
        /// Returns the width.
        fn width(self: &QSizeF) -> f64;

        /// Sets the height to the given height.
        #[rust_name = "set_height"]
        fn setHeight(self: &mut QSizeF, h: f64);
        /// Sets the width to the given width.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QSizeF, w: f64);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qsizef_init_default"]
        fn construct() -> QSizeF;
        #[doc(hidden)]
        #[rust_name = "qsizef_init"]
        fn construct(w: f64, h: f64) -> QSizeF;
        #[doc(hidden)]
        #[rust_name = "qsizef_to_qstring"]
        fn toQString(value: &QSizeF) -> QString;
        #[doc(hidden)]
        #[rust_name = "qsizef_plus"]
        fn operatorPlus(a: &QSizeF, b: &QSizeF) -> QSizeF;
        #[doc(hidden)]
        #[rust_name = "qsizef_minus"]
        fn operatorMinus(a: &QSizeF, b: &QSizeF) -> QSizeF;
        #[doc(hidden)]
        #[rust_name = "qsizef_mul"]
        fn operatorMul(a: f64, b: &QSizeF) -> QSizeF;
        #[doc(hidden)]
        #[rust_name = "qsizef_div"]
        fn operatorDiv(a: f64, b: &QSizeF) -> QSizeF;
    }
}

/// The QSizeF class defines the size of a two-dimensional object using floating point precision.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct QSizeF {
    w: f64,
    h: f64,
}

impl QSizeF {
    /// Constructs a size with the given width and height.
    pub fn new(w: f64, h: f64) -> Self {
        ffi::qsizef_init(w, h)
    }
}

impl Default for QSizeF {
    /// Constructs an invalid size.
    fn default() -> Self {
        ffi::qsizef_init_default()
    }
}

impl fmt::Display for QSizeF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qsizef_to_qstring(self))
    }
}

impl std::ops::Add for QSizeF {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ffi::qsizef_plus(&self, &other)
    }
}

impl std::ops::Sub for QSizeF {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ffi::qsizef_minus(&self, &other)
    }
}

impl std::ops::Mul<f64> for QSizeF {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        ffi::qsizef_mul(rhs, &self)
    }
}

impl std::ops::Div<f64> for QSizeF {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        ffi::qsizef_div(rhs, &self)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QSizeF is trivial.
unsafe impl ExternType for QSizeF {
    type Id = type_id!("QSizeF");
    type Kind = cxx::kind::Trivial;
}

#[cfg(feature = "serde")]
use serde::ser::SerializeMap;

#[cfg(feature = "serde")]
struct QSizeFVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for QSizeFVisitor {
    type Value = QSizeF;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("QSizeF")
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
            Ok(QSizeF::new(width, height))
        } else {
            Err(serde::de::Error::missing_field(
                "missing width or height as key",
            ))
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QSizeF {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(QSizeFVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for QSizeF {
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
        let test_data: QSizeF = serde_json::from_str(r#"{"width":1.0,"height":2.0}"#).unwrap();
        assert_eq!(test_data, QSizeF::new(1.0, 2.0));
    }

    #[test]
    fn test_serde_serialize() {
        let test_data = QSizeF::new(1.0, 2.0);
        let data_string = serde_json::to_string(&test_data).unwrap();
        assert_eq!(data_string, r#"{"width":1.0,"height":2.0}"#);
    }
}
