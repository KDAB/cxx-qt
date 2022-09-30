// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qpoint.h");

        type QPoint = super::QPoint;

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
        #[doc(hidden)]
        #[rust_name = "qpoint_init_default"]
        fn qpointInitDefault() -> QPoint;
        #[doc(hidden)]
        #[rust_name = "qpoint_init"]
        fn qpointInit(x: i32, y: i32) -> QPoint;
    }
}

/// The QPoint struct defines a point in the plane using integer precision.
#[derive(Debug, Clone)]
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

// Safety:
//
// Static checks on the C++ side ensure that QPoint is trivial.
unsafe impl ExternType for QPoint {
    type Id = type_id!("QPoint");
    type Kind = cxx::kind::Trivial;
}
