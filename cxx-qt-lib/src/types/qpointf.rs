// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QPointF = super::QPointF;

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

        #[doc(hidden)]
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qpointf_init_default"]
        fn qpointfInitDefault() -> QPointF;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qpointf_init"]
        fn qpointfInit(x: f64, y: f64) -> QPointF;
    }
}

/// The QPointF struct defines a point in the plane using floating point precision.
#[derive(Debug, Clone, Copy)]
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

// Safety:
//
// Static checks on the C++ side ensure that QPointF is trivial.
unsafe impl ExternType for QPointF {
    type Id = type_id!("QPointF");
    type Kind = cxx::kind::Trivial;
}

#[doc(hidden)]
impl From<&QPointF> for QPointF {
    // TODO: in the future remove at least the deref to a clone and potentially remove this ?
    fn from(qpointf: &QPointF) -> Self {
        *qpointf
    }
}
