// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type AspectRatioMode = crate::AspectRatioMode;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qmargins.h");
        type QMargins = crate::QMargins;
        include!("cxx-qt-lib/qsize.h");
        type QSize = super::QSize;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Returns a size holding the minimum width and height of this size and the given otherSize.
        #[rust_name = "bounded_to"]
        fn boundedTo(self: &QSize, other_size: &QSize) -> QSize;

        /// Returns a size holding the maximum width and height of this size and the given otherSize.
        #[rust_name = "expanded_to"]
        fn expandedTo(self: &QSize, other_size: &QSize) -> QSize;

        /// Returns the height.
        fn height(self: &QSize) -> i32;

        /// Returns true if either of the width and height is less than or equal to 0; otherwise returns false.
        #[rust_name = "is_empty"]
        fn isEmpty(self: &QSize) -> bool;

        /// Returns true if both the width and height is 0; otherwise returns false.
        #[rust_name = "is_null"]
        fn isNull(self: &QSize) -> bool;

        /// Returns true if both the width and height is equal to or greater than 0; otherwise returns false.
        #[rust_name = "is_valid"]
        fn isValid(self: &QSize) -> bool;

        /// Returns the size that results from growing this size by margins.
        #[rust_name = "grown_by"]
        fn grownBy(self: &QSize, margins: QMargins) -> QSize;

        /// Scales the size to a rectangle with the given size, according to the specified mode.
        fn scale(self: &mut QSize, size: &QSize, mode: AspectRatioMode);

        /// Return a size scaled to a rectangle with the given size s, according to the specified mode.
        fn scaled(self: &QSize, s: &QSize, mode: AspectRatioMode) -> QSize;

        /// Sets the height to the given height.
        #[rust_name = "set_height"]
        fn setHeight(self: &mut QSize, height: i32);

        /// Sets the width to the given width.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QSize, width: i32);

        /// Returns the size that results from shrinking this size by margins.
        #[rust_name = "shrunk_by"]
        fn shrunkBy(self: &QSize, margins: QMargins) -> QSize;

        /// Swaps the width and height values.
        fn transpose(self: &mut QSize);

        /// Returns a QSize with width and height swapped.
        fn transposed(self: &QSize) -> QSize;

        /// Returns the width.
        fn width(self: &QSize) -> i32;
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
    }
}

/// The QSize struct defines the size of a two-dimensional object using integer point precision.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct QSize {
    width: i32,
    height: i32,
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

// Safety:
//
// Static checks on the C++ side ensure that QSize is trivial.
unsafe impl ExternType for QSize {
    type Id = type_id!("QSize");
    type Kind = cxx::kind::Trivial;
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
