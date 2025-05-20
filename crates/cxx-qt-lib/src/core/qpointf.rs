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
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = super::QPointF;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Returns true if both the x and y coordinates are set to 0.0 (ignoring the sign); otherwise returns false.
        #[rust_name = "is_null"]
        fn isNull(self: &QPointF) -> bool;

        /// Returns the sum of the absolute values of x() and y(),
        /// traditionally known as the "Manhattan length" of the vector from the origin to the point.
        #[rust_name = "manhattan_length"]
        fn manhattanLength(self: &QPointF) -> f64;

        /// Sets the x coordinate of this point to the given finite x coordinate.
        #[rust_name = "set_x"]
        fn setX(self: &mut QPointF, x: f64);

        /// Sets the y coordinate of this point to the given finite y coordinate.
        #[rust_name = "set_y"]
        fn setY(self: &mut QPointF, y: f64);

        /// Rounds the coordinates of this point to the nearest integer,
        /// and returns a QPoint object with the rounded coordinates.
        #[rust_name = "to_point"]
        fn toPoint(self: &QPointF) -> QPoint;

        /// Returns a point with x and y coordinates exchanged
        fn transposed(self: &QPointF) -> QPointF;

        /// Returns the x coordinate of this point.
        fn x(self: &QPointF) -> f64;

        /// Returns the y coordinate of this point.
        fn y(self: &QPointF) -> f64;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qpointf_dot_product"]
        fn qpointfDotProduct(p1: &QPointF, p2: &QPointF) -> f64;
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
        #[rust_name = "qpointf_from_qpoint"]
        fn construct(point: &QPoint) -> QPointF;
        #[doc(hidden)]
        #[rust_name = "qpointf_to_debug_qstring"]
        fn toDebugQString(value: &QPointF) -> QString;
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
    /// Returns the dot product of p1 and p2.
    pub fn dot_product(p1: &QPointF, p2: &QPointF) -> f64 {
        ffi::qpointf_dot_product(p1, p2)
    }

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
        ffi::qpointf_to_debug_qstring(self).fmt(f)
    }
}

impl From<&ffi::QPoint> for QPointF {
    /// Constructs a copy of the given point.
    fn from(point: &ffi::QPoint) -> Self {
        ffi::qpointf_from_qpoint(point)
    }
}

impl From<QPointF> for ffi::QPoint {
    /// Rounds the coordinates of this point to the nearest integer,
    /// and returns a QPoint object with the rounded coordinates.
    fn from(value: QPointF) -> Self {
        value.to_point()
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
