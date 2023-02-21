// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qrect.h");
        include!("cxx-qt-lib/qstring.h");
        include!("cxx-qt-lib/qmargins.h");

        type QRect = super::QRect;
        type QString = crate::QString;
        type QMargins = crate::QMargins;

        /// Returns the height of the rectangle.
        fn height(self: &QRect) -> i32;
        /// Returns the width of the rectangle.
        fn width(self: &QRect) -> i32;
        /// Returns the x-coordinate of the rectangle's left edge.
        fn x(self: &QRect) -> i32;
        /// Returns the y-coordinate of the rectangle's top edge.
        fn y(self: &QRect) -> i32;

        /// Sets the height of the rectangle to the given height. The bottom edge is changed, but not the top one.
        #[rust_name = "set_height"]
        fn setHeight(self: &mut QRect, h: i32);
        /// Sets the width of the rectangle to the given width. The right edge is changed, but not the left one.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QRect, w: i32);
        /// Sets the left edge of the rectangle to the given x coordinate. May change the width, but will never change the right edge of the rectangle.
        #[rust_name = "set_x"]
        fn setX(self: &mut QRect, x: i32);
        /// Sets the top edge of the rectangle to the given y coordinate. May change the height, but will never change the bottom edge of the rectangle.
        #[rust_name = "set_y"]
        fn setY(self: &mut QRect, y: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qrect_init_default"]
        fn construct() -> QRect;
        #[doc(hidden)]
        #[rust_name = "qrect_init"]
        fn construct(x: i32, y: i32, width: i32, height: i32) -> QRect;
        #[doc(hidden)]
        #[rust_name = "qrect_to_qstring"]
        fn toQString(value: &QRect) -> QString;
        #[doc(hidden)]
        #[rust_name = "qrect_plus"]
        fn operatorPlus(a: &QRect, b: &QMargins) -> QRect;
        #[doc(hidden)]
        #[rust_name = "qrect_minus"]
        fn operatorMinus(a: &QRect, b: &QMargins) -> QRect;
    }
}

/// The QRect struct defines a rectangle in the plane using integer precision.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct QRect {
    // Note that Qt stores QRect as two points rather than a point and size (which QRectF is)
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl QRect {
    /// Constructs a rectangle with (x, y) as its top-left corner and the given width and height.
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        ffi::qrect_init(x, y, width, height)
    }
}

impl Default for QRect {
    /// Constructs a null rectangle.
    fn default() -> Self {
        ffi::qrect_init_default()
    }
}

impl fmt::Display for QRect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qrect_to_qstring(self))
    }
}

type QMargins = crate::QMargins;
impl std::ops::Add<QMargins> for QRect {
    type Output = Self;
    fn add(self, other: QMargins) -> Self {
        ffi::qrect_plus(&self, &other)
    }
}

impl std::ops::Sub<QMargins> for QRect {
    type Output = Self;
    fn sub(self, other: QMargins) -> Self {
        ffi::qrect_minus(&self, &other)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QRect is trivial.
unsafe impl ExternType for QRect {
    type Id = type_id!("QRect");
    type Kind = cxx::kind::Trivial;
}

#[cfg(feature = "serde")]
use serde::ser::SerializeMap;

#[cfg(feature = "serde")]
struct QRectVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for QRectVisitor {
    type Value = QRect;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("QRect")
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
            Ok(QRect::new(x, y, width, height))
        } else {
            Err(serde::de::Error::missing_field(
                "missing x, y, height, or width as key",
            ))
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QRect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(QRectVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for QRect {
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
        let test_data: QRect =
            serde_json::from_str(r#"{"x":1,"y":2,"width":3,"height":4}"#).unwrap();
        assert_eq!(test_data, QRect::new(1, 2, 3, 4));
    }

    #[test]
    fn test_serde_serialize() {
        let test_data = QRect::new(1, 2, 3, 4);
        let data_string = serde_json::to_string(&test_data).unwrap();
        assert_eq!(data_string, r#"{"x":1,"y":2,"width":3,"height":4}"#);
    }
}
