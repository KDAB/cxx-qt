// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qrect.h");

        type QRect = super::QRect;

        /// Returns the height of the rectangle.
        fn height(self: &QRect) -> i32;
        /// Returns the width of the rectangle.
        fn width(self: &QRect) -> i32;
        /// Returns the x-coordinate of the rectangle's left edge.
        fn x(self: &QRect) -> i32;
        /// Returns the y-coordinate of the rectangle's top edge.
        fn y(self: &QRect) -> i32;

        /// Sets the height of the rectangle to the given height. The bottom edge is changed, but not the top one.
        #[rust_name = "set_height"]
        fn setHeight(self: &mut QRect, h: i32);
        /// Sets the width of the rectangle to the given width. The right edge is changed, but not the left one.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QRect, w: i32);
        /// Sets the left edge of the rectangle to the given x coordinate. May change the width, but will never change the right edge of the rectangle.
        #[rust_name = "set_x"]
        fn setX(self: &mut QRect, x: i32);
        /// Sets the top edge of the rectangle to the given y coordinate. May change the height, but will never change the bottom edge of the rectangle.
        #[rust_name = "set_y"]
        fn setY(self: &mut QRect, y: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qrect_init_default"]
        fn construct() -> QRect;
        #[doc(hidden)]
        #[rust_name = "qrect_init"]
        fn construct(x: i32, y: i32, width: i32, height: i32) -> QRect;
    }
}

/// The QRect struct defines a rectangle in the plane using integer precision.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct QRect {
    // Note that Qt stores QRect as two points rather than a point and size (which QRectF is)
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl QRect {
    /// Constructs a rectangle with (x, y) as its top-left corner and the given width and height.
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        ffi::qrect_init(x, y, width, height)
    }
}

impl Default for QRect {
    /// Constructs a null rectangle.
    fn default() -> Self {
        ffi::qrect_init_default()
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QRect is trivial.
unsafe impl ExternType for QRect {
    type Id = type_id!("QRect");
    type Kind = cxx::kind::Trivial;
}
