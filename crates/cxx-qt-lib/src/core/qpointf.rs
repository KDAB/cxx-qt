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
        include!("cxx-qt-lib/qpointf.h");
        include!("cxx-qt-lib/qstring.h");

        type QPointF = super::QPointF;
        type QString = crate::QString;

        /// Returns the x coordinate of this point.
        fn x(self: &QPointF) -> f64;
        /// Returns the y coordinate of this point.
        fn y(self: &QPointF) -> f64;

        /// Sets the x coordinate of this point to the given x coordinate.
        #[rust_name = "set_x"]
        fn setX(self: &mut QPointF, x: f64);
        /// Sets the y coordinate of this point to the given y coordinate.
        #[rust_name = "set_y"]
        fn setY(self: &mut QPointF, y: f64);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qpointf_init_default"]
        fn construct() -> QPointF;
        #[doc(hidden)]
        #[rust_name = "qpointf_init"]
        fn construct(x: f64, y: f64) -> QPointF;
        #[doc(hidden)]
        #[rust_name = "qpointf_to_qstring"]
        fn toQString(value: &QPointF) -> QString;
        #[doc(hidden)]
        #[rust_name = "qpointf_plus"]
        fn operatorPlus(a: &QPointF, b: &QPointF) -> QPointF;
        #[doc(hidden)]
        #[rust_name = "qpointf_minus"]
        fn operatorMinus(a: &QPointF, b: &QPointF) -> QPointF;
        #[doc(hidden)]
        #[rust_name = "qpointf_mul"]
        fn operatorMul(a: f64, b: &QPointF) -> QPointF;
        #[doc(hidden)]
        #[rust_name = "qpointf_div"]
        fn operatorDiv(a: f64, b: &QPointF) -> QPointF;
    }
}

/// The QPointF struct defines a point in the plane using floating point precision.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct QPointF {
    x: f64,
    y: f64,
}

impl QPointF {
    /// Constructs a point with the given coordinates (x, y).
    pub fn new(x: f64, y: f64) -> Self {
        ffi::qpointf_init(x, y)
    }
}

impl Default for QPointF {
    /// Constructs a null point, i.e. with coordinates (0.0, 0.0)
    fn default() -> Self {
        ffi::qpointf_init_default()
    }
}

impl fmt::Display for QPointF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qpointf_to_qstring(self))
    }
}

impl std::ops::Add for QPointF {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ffi::qpointf_plus(&self, &other)
    }
}

impl std::ops::Sub for QPointF {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ffi::qpointf_minus(&self, &other)
    }
}

impl std::ops::Mul<f64> for QPointF {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        ffi::qpointf_mul(rhs, &self)
    }
}

impl std::ops::Div<f64> for QPointF {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        ffi::qpointf_div(rhs, &self)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QPointF is trivial.
unsafe impl ExternType for QPointF {
    type Id = type_id!("QPointF");
    type Kind = cxx::kind::Trivial;
}
