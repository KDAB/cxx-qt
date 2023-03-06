// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qrectf.h");
        include!("cxx-qt-lib/qstring.h");
        include!("cxx-qt-lib/qmarginsf.h");

        type QRectF = super::QRectF;
        type QString = crate::QString;
        type QMarginsF = crate::QMarginsF;

        /// Returns the height of the rectangle.
        fn height(self: &QRectF) -> f64;
        /// Returns the width of the rectangle.
        fn width(self: &QRectF) -> f64;
        /// Returns the x-coordinate of the rectangle's left edge.
        fn x(self: &QRectF) -> f64;
        /// Returns the y-coordinate of the rectangle's top edge.
        fn y(self: &QRectF) -> f64;

        /// Sets the height of the rectangle to the given height. The bottom edge is changed, but not the top one.
        #[rust_name = "set_height"]
        fn setHeight(self: &mut QRectF, h: f64);
        /// Sets the width of the rectangle to the given width. The right edge is changed, but not the left one.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QRectF, w: f64);
        /// Sets the left edge of the rectangle to the given x coordinate. May change the width, but will never change the right edge of the rectangle.
        #[rust_name = "set_x"]
        fn setX(self: &mut QRectF, x: f64);
        /// Sets the top edge of the rectangle to the given y coordinate. May change the height, but will never change the bottom edge of the rectangle.
        #[rust_name = "set_y"]
        fn setY(self: &mut QRectF, y: f64);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qrectf_init_default"]
        fn construct() -> QRectF;
        #[doc(hidden)]
        #[rust_name = "qrectf_init"]
        fn construct(x: f64, y: f64, width: f64, height: f64) -> QRectF;
        #[doc(hidden)]
        #[rust_name = "qrectf_to_qstring"]
        fn toQString(value: &QRectF) -> QString;
        #[doc(hidden)]
        #[rust_name = "qrectf_plus"]
        fn operatorPlus(a: &QRectF, b: &QMarginsF) -> QRectF;
        #[doc(hidden)]
        #[rust_name = "qrectf_minus"]
        fn operatorMinus(a: &QRectF, b: &QMarginsF) -> QRectF;
    }
}

/// The QRectF struct defines a rectangle in the plane using floating point precision.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct QRectF {
    xp: f64,
    yp: f64,
    w: f64,
    h: f64,
}

impl QRectF {
    /// Constructs a rectangle with (x, y) as its top-left corner and the given width and height.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        ffi::qrectf_init(x, y, width, height)
    }
}

impl Default for QRectF {
    /// Constructs a null rectangle.
    fn default() -> Self {
        ffi::qrectf_init_default()
    }
}

impl fmt::Display for QRectF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qrectf_to_qstring(self))
    }
}

type QMarginsF = crate::QMarginsF;
impl std::ops::Add<QMarginsF> for QRectF {
    type Output = Self;
    fn add(self, other: QMarginsF) -> Self {
        ffi::qrectf_plus(&self, &other)
    }
}

impl std::ops::Sub<QMarginsF> for QRectF {
    type Output = Self;
    fn sub(self, other: QMarginsF) -> Self {
        ffi::qrectf_minus(&self, &other)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QRectF is trivial.
unsafe impl ExternType for QRectF {
    type Id = type_id!("QRectF");
    type Kind = cxx::kind::Trivial;
}

#[cfg(feature = "serde")]
use serde::ser::SerializeMap;

#[cfg(feature = "serde")]
struct QRectFVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for QRectFVisitor {
    type Value = QRectF;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("QRectF")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut x = None;
        let mut y = None;
        let mut width = None;
        let mut height = None;

        while let Some((key, value)) = map.next_entry()? {
            match key {
                "x" => x = Some(value),
                "y" => y = Some(value),
                "width" => width = Some(value),
                "height" => height = Some(value),
                others => {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(others),
                        &"expected either x, y, height, or width as a key",
                    ));
                }
            }
        }

        if let (Some(x), Some(y), Some(width), Some(height)) = (x, y, width, height) {
            Ok(QRectF::new(x, y, width, height))
        } else {
            Err(serde::de::Error::missing_field(
                "missing x, y, height, or width as key",
            ))
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QRectF {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(QRectFVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for QRectF {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("x", &self.x())?;
        map.serialize_entry("y", &self.y())?;
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
        let test_data: QRectF =
            serde_json::from_str(r#"{"x":1.0,"y":2.0,"width":3.0,"height":4.0}"#).unwrap();
        assert_eq!(test_data, QRectF::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn test_serde_serialize() {
        let test_data = QRectF::new(1.0, 2.0, 3.0, 4.0);
        let data_string = serde_json::to_string(&test_data).unwrap();
        assert_eq!(data_string, r#"{"x":1.0,"y":2.0,"width":3.0,"height":4.0}"#);
    }
}
