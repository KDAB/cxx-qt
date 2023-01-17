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

        /// Returns true if both the x and y coordinates are set to 0, otherwise returns false.
        #[rust_name = "is_null"]
        fn isNull(self: &QPoint) -> bool;

        /// Returns the sum of the absolute values of x() and y(),
        /// traditionally known as the "Manhattan length" of the vector from the origin to the point.
        #[rust_name = "manhattan_length"]
        fn manhattanLength(self: &QPoint) -> i32;

        /// Sets the x coordinate of this point to the given x coordinate.
        #[rust_name = "set_x"]
        fn setX(self: &mut QPoint, x: i32);

        /// Sets the y coordinate of this point to the given y coordinate.
        #[rust_name = "set_y"]
        fn setY(self: &mut QPoint, y: i32);

        /// Returns a point with x and y coordinates exchanged
        fn transposed(self: &QPoint) -> QPoint;

        /// Returns the x coordinate of this point.
        fn x(self: &QPoint) -> i32;

        /// Returns the y coordinate of this point.
        fn y(self: &QPoint) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qpoint_dot_product"]
        fn qpointDotProduct(p1: &QPoint, p2: &QPoint) -> i32;
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
    /// Returns the dot product of p1 and p2.
    pub fn dot_product(p1: &QPoint, p2: &QPoint) -> i32 {
        ffi::qpoint_dot_product(p1, p2)
    }

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

// Safety:
//
// Static checks on the C++ side ensure that QPoint is trivial.
unsafe impl ExternType for QPoint {
    type Id = type_id!("QPoint");
    type Kind = cxx::kind::Trivial;
}
