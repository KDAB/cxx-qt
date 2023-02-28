// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        include!("cxx-qt-lib/qstring.h");

        type QPoint = super::QPoint;
        type QString = crate::QString;

        /// Returns the x coordinate of this point.
        fn x(self: &QPoint) -> i32;
        /// Returns the y coordinate of this point.
        fn y(self: &QPoint) -> i32;

        /// Sets the x coordinate of this point to the given x coordinate.
        #[rust_name = "set_x"]
        fn setX(self: &mut QPoint, x: i32);
        /// Sets the y coordinate of this point to the given y coordinate.
        #[rust_name = "set_y"]
        fn setY(self: &mut QPoint, y: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qpoint_init_default"]
        fn construct() -> QPoint;
        #[doc(hidden)]
        #[rust_name = "qpoint_init"]
        fn construct(x: i32, y: i32) -> QPoint;
        #[doc(hidden)]
        #[rust_name = "qpoint_to_qstring"]
        fn toQString(value: &QPoint) -> QString;
        #[doc(hidden)]
        #[rust_name = "qpoint_plus"]
        fn operatorPlus(a: &QPoint, b: &QPoint) -> QPoint;
        #[doc(hidden)]
        #[rust_name = "qpoint_minus"]
        fn operatorMinus(a: &QPoint, b: &QPoint) -> QPoint;
        #[doc(hidden)]
        #[rust_name = "qpoint_mul_f32"]
        fn operatorMul(a: f32, b: &QPoint) -> QPoint;
        #[doc(hidden)]
        #[rust_name = "qpoint_mul_f64"]
        fn operatorMul(a: f64, b: &QPoint) -> QPoint;
        #[doc(hidden)]
        #[rust_name = "qpoint_mul_i32"]
        fn operatorMul(a: i32, b: &QPoint) -> QPoint;
        #[doc(hidden)]
        #[rust_name = "qpoint_div"]
        fn operatorDiv(a: f64, b: &QPoint) -> QPoint;
    }
}

/// The QPoint struct defines a point in the plane using integer precision.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct QPoint {
    x: i32,
    y: i32,
}

impl QPoint {
    /// Constructs a point with the given coordinates (x, y).
    pub fn new(x: i32, y: i32) -> Self {
        ffi::qpoint_init(x, y)
    }
}

impl Default for QPoint {
    /// Constructs a null point, i.e. with coordinates (0, 0)
    fn default() -> Self {
        ffi::qpoint_init_default()
    }
}

impl fmt::Display for QPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qpoint_to_qstring(self))
    }
}

impl std::ops::Add for QPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ffi::qpoint_plus(&self, &other)
    }
}

impl std::ops::Sub for QPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ffi::qpoint_minus(&self, &other)
    }
}

impl std::ops::Mul<f32> for QPoint {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        ffi::qpoint_mul_f32(rhs, &self)
    }
}

impl std::ops::Mul<f64> for QPoint {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        ffi::qpoint_mul_f64(rhs, &self)
    }
}

impl std::ops::Mul<i32> for QPoint {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        ffi::qpoint_mul_i32(rhs, &self)
    }
}

impl std::ops::Div<f64> for QPoint {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        ffi::qpoint_div(rhs, &self)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QPoint is trivial.
unsafe impl ExternType for QPoint {
    type Id = type_id!("QPoint");
    type Kind = cxx::kind::Trivial;
}
