// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

use crate::QMargins;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmargins.h");
        type QMargins = crate::QMargins;
        include!("cxx-qt-lib/qmarginsf.h");
        type QMarginsF = super::QMarginsF;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Returns the bottom margin.
        fn bottom(self: &QMarginsF) -> f64;

        /// Returns `true` if all margins are very close to 0; otherwise returns `false`.
        #[rust_name = "is_null"]
        fn isNull(self: &QMarginsF) -> bool;

        /// Returns the left margin.
        fn left(self: &QMarginsF) -> f64;

        /// Returns the right margin.
        fn right(self: &QMarginsF) -> f64;

        /// Sets the bottom margin to `abottom` (which must be finite).
        #[rust_name = "set_bottom"]
        fn setBottom(self: &mut QMarginsF, abottom: f64);

        /// Sets the left margin to `aleft` (which must be finite).
        #[rust_name = "set_left"]
        fn setLeft(self: &mut QMarginsF, aleft: f64);

        /// Sets the right margin to `aright` (which must be finite).
        #[rust_name = "set_right"]
        fn setRight(self: &mut QMarginsF, aright: f64);

        /// Sets the top margin to `atop` (which must be finite).
        #[rust_name = "set_top"]
        fn setTop(self: &mut QMarginsF, atop: f64);

        /// Returns an integer-based copy of this margins object.
        ///
        /// Note that the components in the returned margins will be rounded to the nearest integer.
        #[rust_name = "to_margins"]
        fn toMargins(self: &QMarginsF) -> QMargins;

        /// Returns the top margin.
        fn top(self: &QMarginsF) -> f64;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qmarginsf_default"]
        fn construct() -> QMarginsF;
        #[doc(hidden)]
        #[rust_name = "qmarginsf_from_qmargin"]
        fn construct(margins: &QMargins) -> QMarginsF;
        #[doc(hidden)]
        #[rust_name = "qmarginsf_new"]
        fn construct(left: f64, top: f64, right: f64, bottom: f64) -> QMarginsF;
        #[doc(hidden)]
        #[rust_name = "qmarginsf_to_debug_qstring"]
        fn toDebugQString(value: &QMarginsF) -> QString;
        #[doc(hidden)]
        #[rust_name = "qmarginsf_plus"]
        fn operatorPlus(a: &QMarginsF, b: &QMarginsF) -> QMarginsF;
        #[doc(hidden)]
        #[rust_name = "qmarginsf_plus_f64"]
        fn operatorPlus(a: &QMarginsF, b: &f64) -> QMarginsF;
        #[doc(hidden)]
        #[rust_name = "qmarginsf_minus"]
        fn operatorMinus(a: &QMarginsF, b: &QMarginsF) -> QMarginsF;
        #[doc(hidden)]
        #[rust_name = "qmarginsf_minus_f64"]
        fn operatorMinus(a: &QMarginsF, b: &f64) -> QMarginsF;
        #[doc(hidden)]
        #[rust_name = "qmarginsf_mul"]
        fn operatorMul(a: f64, b: &QMarginsF) -> QMarginsF;
        #[doc(hidden)]
        #[rust_name = "qmarginsf_div"]
        fn operatorDiv(a: f64, b: &QMarginsF) -> QMarginsF;
    }
}

/// The `QMarginsF` class defines the four margins of a rectangle.
///
/// Qt Documentation: [QMarginsF](https://doc.qt.io/qt/qmarginsf.html#details)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct QMarginsF {
    left: f64,
    top: f64,
    right: f64,
    bottom: f64,
}

impl QMarginsF {
    /// Constructs margins with the given `left`, `top`, `right`, and `bottom`. All parameters must be finite.
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        ffi::qmarginsf_new(left, top, right, bottom)
    }
}

impl Default for QMarginsF {
    /// Constructs a margins object with all margins set to 0.
    fn default() -> Self {
        ffi::qmarginsf_default()
    }
}

impl fmt::Display for QMarginsF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qmarginsf_to_debug_qstring(self).fmt(f)
    }
}

impl std::ops::Add for QMarginsF {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ffi::qmarginsf_plus(&self, &other)
    }
}

impl std::ops::Add<f64> for QMarginsF {
    type Output = Self;
    fn add(self, other: f64) -> Self {
        ffi::qmarginsf_plus_f64(&self, &other)
    }
}

impl std::ops::Sub for QMarginsF {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ffi::qmarginsf_minus(&self, &other)
    }
}

impl std::ops::Sub<f64> for QMarginsF {
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        ffi::qmarginsf_minus_f64(&self, &other)
    }
}

impl std::ops::Mul<f64> for QMarginsF {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        ffi::qmarginsf_mul(rhs, &self)
    }
}

impl std::ops::Div<f64> for QMarginsF {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        ffi::qmarginsf_div(rhs, &self)
    }
}

impl From<&QMargins> for QMarginsF {
    /// Constructs margins copied from the given `margins`.
    fn from(margins: &QMargins) -> Self {
        ffi::qmarginsf_from_qmargin(margins)
    }
}
impl From<QMargins> for QMarginsF {
    /// Constructs margins copied from the given `margins`.
    fn from(margins: QMargins) -> Self {
        Self::from(&margins)
    }
}

impl From<&QMarginsF> for QMargins {
    /// Returns an integer-based copy of `margins`.
    ///
    /// Note that the components in the returned margins will be rounded to the nearest integer.
    fn from(margins: &QMarginsF) -> Self {
        margins.to_margins()
    }
}
impl From<QMarginsF> for QMargins {
    /// Returns an integer-based copy of `margins`.
    ///
    /// Note that the components in the returned margins will be rounded to the nearest integer.
    fn from(margins: QMarginsF) -> Self {
        Self::from(&margins)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QMarginsF is trivial.
unsafe impl ExternType for QMarginsF {
    type Id = type_id!("QMarginsF");
    type Kind = cxx::kind::Trivial;
}
