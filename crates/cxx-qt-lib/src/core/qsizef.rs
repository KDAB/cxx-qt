// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

use crate::QSize;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type AspectRatioMode = crate::AspectRatioMode;
    }

    extern "C++" {
        include!("cxx-qt-lib/qmarginsf.h");
        type QMarginsF = crate::QMarginsF;
        include!("cxx-qt-lib/qsize.h");
        type QSize = crate::QSize;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qsizef.h");
    }

    unsafe extern "C++" {
        type QSizeF = super::QSizeF;

        /// Returns a size holding the minimum width and height of this size and the given `other_size`.
        #[rust_name = "bounded_to"]
        fn boundedTo(&self, other_size: &QSizeF) -> QSizeF;

        /// Returns a size holding the maximum width and height of this size and the given `other_size`.
        #[rust_name = "expanded_to"]
        fn expandedTo(&self, other_size: &QSizeF) -> QSizeF;

        /// Returns the height.
        fn height(&self) -> f64;

        /// Returns `true` if either of the width and height is less than or equal to 0; otherwise returns `false`.
        #[rust_name = "is_empty"]
        fn isEmpty(&self) -> bool;

        /// Returns `true` if both the width and height are 0.0 (ignoring the sign); otherwise returns `false`.
        #[rust_name = "is_null"]
        fn isNull(&self) -> bool;

        /// Returns `true` if both the width and height are equal to or greater than 0; otherwise returns `false`.
        #[rust_name = "is_valid"]
        fn isValid(&self) -> bool;

        /// Returns the size that results from growing this size by `margins`.
        #[rust_name = "grown_by"]
        fn grownBy(&self, margins: QMarginsF) -> QSizeF;

        /// Scales the size to a rectangle with the given `size`, according to the specified `mode`.
        fn scale(&mut self, size: &QSizeF, mode: AspectRatioMode);

        /// Returns a size scaled to a rectangle with the given size `s`, according to the specified `mode`.
        fn scaled(&self, s: &QSizeF, mode: AspectRatioMode) -> QSizeF;

        /// Sets the height to the given finite `height`.
        #[rust_name = "set_height"]
        fn setHeight(&mut self, height: f64);

        /// Sets the width to the given finite `width`.
        #[rust_name = "set_width"]
        fn setWidth(&mut self, width: f64);

        /// Returns the size that results from shrinking this size by `margins`.
        #[rust_name = "shrunk_by"]
        fn shrunkBy(&self, margins: QMarginsF) -> QSizeF;

        /// Returns an integer based copy of this size.
        ///
        /// Note that the coordinates in the returned size will be rounded to the nearest integer.
        #[rust_name = "to_size"]
        fn toSize(&self) -> QSize;

        /// Swaps the width and height values.
        fn transpose(&mut self);

        /// Returns the size with width and height values swapped.
        fn transposed(&self) -> QSizeF;

        /// Returns the width.
        fn width(&self) -> f64;
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
        #[rust_name = "qsizef_from_qsize"]
        fn construct(size: &QSize) -> QSizeF;
        #[doc(hidden)]
        #[rust_name = "qsizef_to_debug_qstring"]
        fn toDebugQString(value: &QSizeF) -> QString;
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

/// The `QSizeF` class defines the size of a two-dimensional object using floating point precision.
///
/// Qt Documentation: [QSizeF](https://doc.qt.io/qt/qsizef.html#details)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct QSizeF {
    width: f64,
    height: f64,
}

impl QSizeF {
    /// Constructs a size with the given `width` and `height`.
    pub fn new(width: f64, height: f64) -> Self {
        ffi::qsizef_init(width, height)
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
        ffi::qsizef_to_debug_qstring(self).fmt(f)
    }
}

impl From<&QSize> for QSizeF {
    /// Constructs a size with floating point accuracy from the given `size`.
    fn from(size: &QSize) -> Self {
        ffi::qsizef_from_qsize(size)
    }
}
impl From<QSize> for QSizeF {
    /// Constructs a size with floating point accuracy from the given `size`.
    fn from(size: QSize) -> Self {
        Self::from(&size)
    }
}

impl From<&QSizeF> for QSize {
    /// Returns an integer based copy of this size.
    ///
    /// Note that the coordinates in the returned size will be rounded to the nearest integer.
    fn from(size: &QSizeF) -> Self {
        size.to_size()
    }
}
impl From<QSizeF> for QSize {
    /// Returns an integer based copy of this size.
    ///
    /// Note that the coordinates in the returned size will be rounded to the nearest integer.
    fn from(size: QSizeF) -> Self {
        Self::from(&size)
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
