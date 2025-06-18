// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmargins.h");
        type QMargins = super::QMargins;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qmarginsf.h");
        #[allow(dead_code)]
        type QMarginsF = crate::QMarginsF;

        /// Returns the bottom margin.
        fn bottom(self: &QMargins) -> i32;

        /// Returns `true` if all margins are is 0; otherwise returns `false`.
        #[rust_name = "is_null"]
        fn isNull(self: &QMargins) -> bool;

        /// Returns the left margin.
        fn left(self: &QMargins) -> i32;

        /// Returns the right margin.
        fn right(self: &QMargins) -> i32;

        /// Sets the bottom margin to `bottom`.
        #[rust_name = "set_bottom"]
        fn setBottom(self: &mut QMargins, bottom: i32);

        /// Sets the left margin to `left`.
        #[rust_name = "set_left"]
        fn setLeft(self: &mut QMargins, left: i32);

        /// Sets the right margin to `right`.
        #[rust_name = "set_right"]
        fn setRight(self: &mut QMargins, right: i32);

        /// Sets the top margin to `top`.
        #[rust_name = "set_top"]
        fn setTop(self: &mut QMargins, top: i32);

        /// Returns these margins as margins with floating point accuracy.
        ///
        /// This function was introduced in Qt 6.4.
        #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_4))]
        #[rust_name = "to_marginsf"]
        fn toMarginsF(self: &QMargins) -> QMarginsF;

        /// Returns the top margin.
        fn top(self: &QMargins) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qmargins_default"]
        fn construct() -> QMargins;
        #[doc(hidden)]
        #[rust_name = "qmargins_new"]
        fn construct(left: i32, top: i32, right: i32, bottom: i32) -> QMargins;
        #[doc(hidden)]
        #[rust_name = "qmargins_to_debug_qstring"]
        fn toDebugQString(value: &QMargins) -> QString;
        #[doc(hidden)]
        #[rust_name = "qmargins_plus"]
        fn operatorPlus(a: &QMargins, b: &QMargins) -> QMargins;
        #[doc(hidden)]
        #[rust_name = "qmargins_plus_i32"]
        fn operatorPlus(a: &QMargins, b: &i32) -> QMargins;
        #[doc(hidden)]
        #[rust_name = "qmargins_minus"]
        fn operatorMinus(a: &QMargins, b: &QMargins) -> QMargins;
        #[doc(hidden)]
        #[rust_name = "qmargins_minus_i32"]
        fn operatorMinus(a: &QMargins, b: &i32) -> QMargins;
        #[doc(hidden)]
        #[rust_name = "qmargins_mul_i32"]
        fn operatorMul(a: i32, b: &QMargins) -> QMargins;
        #[doc(hidden)]
        #[rust_name = "qmargins_mul_f64"]
        fn operatorMul(a: f64, b: &QMargins) -> QMargins;
        #[doc(hidden)]
        #[rust_name = "qmargins_div_i32"]
        fn operatorDiv(a: i32, b: &QMargins) -> QMargins;
        #[doc(hidden)]
        #[rust_name = "qmargins_div_f64"]
        fn operatorDiv(a: f64, b: &QMargins) -> QMargins;
    }
}

/// The `QMargins` class defines the four margins of a rectangle.
///
/// Qt Documentation: [QMargins](https://doc.qt.io/qt/qmargins.html#details)
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct QMargins {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl QMargins {
    /// Constructs margins with the given `left`, `top`, `right`, and `bottom`.
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        ffi::qmargins_new(left, top, right, bottom)
    }
}

impl Default for QMargins {
    /// Constructs a margins object with all margins set to 0.
    fn default() -> Self {
        ffi::qmargins_default()
    }
}

impl fmt::Display for QMargins {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qmargins_to_debug_qstring(self).fmt(f)
    }
}

impl std::ops::Add for QMargins {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ffi::qmargins_plus(&self, &other)
    }
}

impl std::ops::Add<i32> for QMargins {
    type Output = Self;
    fn add(self, other: i32) -> Self {
        ffi::qmargins_plus_i32(&self, &other)
    }
}

impl std::ops::Sub for QMargins {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ffi::qmargins_minus(&self, &other)
    }
}

impl std::ops::Sub<i32> for QMargins {
    type Output = Self;
    fn sub(self, other: i32) -> Self {
        ffi::qmargins_minus_i32(&self, &other)
    }
}

impl std::ops::Mul<i32> for QMargins {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        ffi::qmargins_mul_i32(rhs, &self)
    }
}

impl std::ops::Mul<f64> for QMargins {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        ffi::qmargins_mul_f64(rhs, &self)
    }
}

impl std::ops::Div<i32> for QMargins {
    type Output = Self;
    fn div(self, rhs: i32) -> Self {
        ffi::qmargins_div_i32(rhs, &self)
    }
}

impl std::ops::Div<f64> for QMargins {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        ffi::qmargins_div_f64(rhs, &self)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QMargins is trivial.
unsafe impl ExternType for QMargins {
    type Id = type_id!("QMargins");
    type Kind = cxx::kind::Trivial;
}
