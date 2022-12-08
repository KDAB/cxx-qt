// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qrectf.h");

        type QRectF = super::QRectF;

        /// Returns the height of the rectangle.
        fn height(self: &QRectF) -> f64;
        /// Returns the width of the rectangle.
        fn width(self: &QRectF) -> f64;
        /// Returns the x-coordinate of the rectangle's left edge.
        fn x(self: &QRectF) -> f64;
        /// Returns the y-coordinate of the rectangle's top edge.
        fn y(self: &QRectF) -> f64;

        /// Sets the height of the rectangle to the given height. The bottom edge is changed, but not the top one.
        #[rust_name = "set_height"]
        fn setHeight(self: &mut QRectF, h: f64);
        /// Sets the width of the rectangle to the given width. The right edge is changed, but not the left one.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QRectF, w: f64);
        /// Sets the left edge of the rectangle to the given x coordinate. May change the width, but will never change the right edge of the rectangle.
        #[rust_name = "set_x"]
        fn setX(self: &mut QRectF, x: f64);
        /// Sets the top edge of the rectangle to the given y coordinate. May change the height, but will never change the bottom edge of the rectangle.
        #[rust_name = "set_y"]
        fn setY(self: &mut QRectF, y: f64);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qrectf_init_default"]
        fn construct() -> QRectF;
        #[doc(hidden)]
        #[rust_name = "qrectf_init"]
        fn construct(x: f64, y: f64, width: f64, height: f64) -> QRectF;
    }
}

/// The QRectF struct defines a rectangle in the plane using floating point precision.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct QRectF {
    xp: f64,
    yp: f64,
    w: f64,
    h: f64,
}

impl QRectF {
    /// Constructs a rectangle with (x, y) as its top-left corner and the given width and height.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        ffi::qrectf_init(x, y, width, height)
    }
}

impl Default for QRectF {
    /// Constructs a null rectangle.
    fn default() -> Self {
        ffi::qrectf_init_default()
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QRectF is trivial.
unsafe impl ExternType for QRectF {
    type Id = type_id!("QRectF");
    type Kind = cxx::kind::Trivial;
}
