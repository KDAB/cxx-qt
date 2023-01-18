// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmarginsf.h");
        type QMarginsF = crate::QMarginsF;
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = crate::QPointF;
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;
        include!("cxx-qt-lib/qrectf.h");
        type QRectF = super::QRectF;
        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = crate::QSizeF;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Adds dx1, dy1, dx2 and dy2 respectively to the existing coordinates of the rectangle. All parameters must be finite.
        fn adjust(self: &mut QRectF, dx1: f64, dy1: f64, dx2: f64, dy2: f64);

        /// Returns a new rectangle with dx1, dy1, dx2 and dy2 added respectively to the existing coordinates of this rectangle. All parameters must be finite.
        fn adjusted(self: &QRectF, dx1: f64, dy1: f64, dx2: f64, dy2: f64) -> QRectF;

        /// Returns the y-coordinate of the rectangle's bottom edge.
        fn bottom(self: &QRectF) -> f64;

        /// Returns the position of the rectangle's bottom-left corner.
        #[rust_name = "bottom_left"]
        fn bottomLeft(self: &QRectF) -> QPointF;

        /// Returns the position of the rectangle's bottom-right corner.
        #[rust_name = "bottom_right"]
        fn bottomRight(self: &QRectF) -> QPointF;

        /// Returns the center point of the rectangle.
        fn center(self: &QRectF) -> QPointF;

        /// Returns true if the given point is inside or on the edge of the rectangle; otherwise returns false.
        fn contains(self: &QRectF, point: &QPointF) -> bool;

        /// Returns the height of the rectangle.
        fn height(self: &QRectF) -> f64;

        /// Returns the intersection of this rectangle and the given rectangle. Note that r.intersected(s) is equivalent to r & s.
        fn intersected(self: &QRectF, rectangle: &QRectF) -> QRectF;

        /// Returns true if this rectangle intersects with the given rectangle (i.e. there is a non-empty area of overlap between them), otherwise returns false.
        fn intersects(self: &QRectF, rectangle: &QRectF) -> bool;

        /// Returns true if the rectangle is empty, otherwise returns false.
        ///
        /// An empty rectangle has width() <= 0 or height() <= 0. An empty rectangle is not valid (i.e., isEmpty() == !isValid()).
        #[rust_name = "is_empty"]
        fn isEmpty(self: &QRectF) -> bool;

        /// Returns true if the rectangle is a null rectangle, otherwise returns false.
        ///
        /// A null rectangle has both the width and the height set to 0. A null rectangle is also empty, and hence not valid.
        #[rust_name = "is_null"]
        fn isNull(self: &QRectF) -> bool;

        /// Returns true if the rectangle is valid, otherwise returns false.
        ///
        /// A valid rectangle has a width() > 0 and height() > 0. Note that non-trivial operations like intersections are not defined for invalid rectangles. A valid rectangle is not empty (i.e., isValid() == !isEmpty()).
        #[rust_name = "is_valid"]
        fn isValid(self: &QRectF) -> bool;

        /// Returns the x-coordinate of the rectangle's left edge. Equivalent to x().
        fn left(self: &QRectF) -> f64;

        /// Returns a rectangle grown by the margins.
        #[rust_name = "margins_added"]
        fn marginsAdded(self: &QRectF, margins: &QMarginsF) -> QRectF;

        /// Removes the margins from the rectangle, shrinking it.
        #[rust_name = "margins_removed"]
        fn marginsRemoved(self: &QRectF, margins: &QMarginsF) -> QRectF;

        /// Moves the rectangle vertically, leaving the rectangle's bottom edge at the given finite y coordinate. The rectangle's size is unchanged.
        #[rust_name = "move_bottom"]
        fn moveBottom(self: &mut QRectF, y: f64);

        /// Moves the rectangle, leaving the bottom-left corner at the given position. The rectangle's size is unchanged.
        #[rust_name = "move_bottom_left"]
        fn moveBottomLeft(self: &mut QRectF, position: &QPointF);

        /// Moves the rectangle, leaving the bottom-right corner at the given position. The rectangle's size is unchanged.
        #[rust_name = "move_bottom_right"]
        fn moveBottomRight(self: &mut QRectF, position: &QPointF);

        /// Moves the rectangle, leaving the center point at the given position. The rectangle's size is unchanged.
        #[rust_name = "move_center"]
        fn moveCenter(self: &mut QRectF, position: &QPointF);

        /// Moves the rectangle horizontally, leaving the rectangle's left edge at the given finite x coordinate. The rectangle's size is unchanged.
        #[rust_name = "move_left"]
        fn moveLeft(self: &mut QRectF, x: f64);

        /// Moves the rectangle horizontally, leaving the rectangle's right edge at the given finite x coordinate. The rectangle's size is unchanged.
        #[rust_name = "move_right"]
        fn moveRight(self: &mut QRectF, x: f64);

        /// Moves the rectangle, leaving the top-left corner at the given position.
        #[rust_name = "move_to"]
        fn moveTo(self: &mut QRectF, position: &QPointF);

        /// Moves the rectangle vertically, leaving the rectangle's top line at the given finite y coordinate. The rectangle's size is unchanged.
        #[rust_name = "move_top"]
        fn moveTop(self: &mut QRectF, y: f64);

        /// Moves the rectangle, leaving the top-left corner at the given position. The rectangle's size is unchanged.
        #[rust_name = "move_top_left"]
        fn moveTopLeft(self: &mut QRectF, position: &QPointF);

        /// Moves the rectangle, leaving the top-right corner at the given position. The rectangle's size is unchanged.
        #[rust_name = "move_top_right"]
        fn moveTopRight(self: &mut QRectF, position: &QPointF);

        /// Returns a normalized rectangle; i.e., a rectangle that has a non-negative width and height.
        fn normalized(self: &QRectF) -> QRectF;

        /// Returns the x-coordinate of the rectangle's right edge.
        fn right(self: &QRectF) -> f64;

        /// Sets the bottom edge of the rectangle to the given finite y coordinate.
        /// May change the height, but will never change the top edge of the rectangle.
        #[rust_name = "set_bottom"]
        fn setBottom(self: &mut QRectF, y: f64);

        /// Set the bottom-left corner of the rectangle to the given position.
        /// May change the size, but will never change the top-right corner of the rectangle.
        #[rust_name = "set_bottom_left"]
        fn setBottomLeft(self: &mut QRectF, position: &QPointF);

        /// Set the bottom-right corner of the rectangle to the given position.
        /// May change the size, but will never change the top-left corner of the rectangle.
        #[rust_name = "set_bottom_right"]
        fn setBottomRight(self: &mut QRectF, position: &QPointF);

        /// Sets the coordinates of the rectangle's top-left corner to (x1, y1),
        /// and the coordinates of its bottom-right corner to (x2, y2). All parameters must be finite.
        #[rust_name = "set_coords"]
        fn setCoords(self: &mut QRectF, x1: f64, y1: f64, x2: f64, y2: f64);

        /// Sets the height of the rectangle to the given finite height. The bottom edge is changed, but not the top one.
        #[rust_name = "set_height"]
        fn setHeight(self: &mut QRectF, h: f64);

        /// Sets the left edge of the rectangle to the given finite x coordinate.
        /// May change the width, but will never change the right edge of the rectangle.
        #[rust_name = "set_left"]
        fn setLeft(self: &mut QRectF, x: f64);

        /// Sets the coordinates of the rectangle's top-left corner to (x, y), and its size to the given width and height. All parameters must be finite.
        #[rust_name = "set_rect"]
        fn setRect(self: &mut QRectF, x: f64, y: f64, width: f64, height: f64);

        /// Sets the right edge of the rectangle to the given finite x coordinate.
        /// May change the width, but will never change the left edge of the rectangle.
        #[rust_name = "set_right"]
        fn setRight(self: &mut QRectF, x: f64);

        /// Sets the size of the rectangle to the given finite size. The top-left corner is not moved.
        #[rust_name = "set_size"]
        fn setSize(self: &mut QRectF, size: &QSizeF);

        /// Sets the top edge of the rectangle to the given finite y coordinate.
        /// May change the height, but will never change the bottom edge of the rectangle.
        #[rust_name = "set_top"]
        fn setTop(self: &mut QRectF, y: f64);

        /// Set the top-left corner of the rectangle to the given position.
        /// May change the size, but will never change the bottom-right corner of the rectangle.
        #[rust_name = "set_top_left"]
        fn setTopLeft(self: &mut QRectF, position: &QPointF);

        /// Set the top-right corner of the rectangle to the given position.
        /// May change the size, but will never change the bottom-left corner of the rectangle.
        #[rust_name = "set_top_right"]
        fn setTopRight(self: &mut QRectF, position: &QPointF);

        /// Sets the width of the rectangle to the given finite width. The right edge is changed, but not the left one.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QRectF, w: f64);

        /// Sets the left edge of the rectangle to the given finite x coordinate.
        /// May change the width, but will never change the right edge of the rectangle.
        #[rust_name = "set_x"]
        fn setX(self: &mut QRectF, x: f64);

        /// Sets the top edge of the rectangle to the given finite y coordinate.
        /// May change the height, but will never change the bottom edge of the rectangle.
        #[rust_name = "set_y"]
        fn setY(self: &mut QRectF, y: f64);

        /// Returns the size of the rectangle.
        fn size(self: &QRectF) -> QSizeF;

        /// Returns a QRect based on the values of this rectangle that is the smallest possible integer rectangle that completely contains this rectangle.
        #[rust_name = "to_aligned_rect"]
        fn toAlignedRect(self: &QRectF) -> QRect;

        /// Returns a QRect based on the values of this rectangle.
        /// Note that the coordinates in the returned rectangle are rounded to the nearest integer.
        #[rust_name = "to_rect"]
        fn toRect(self: &QRectF) -> QRect;

        /// Returns the y-coordinate of the rectangle's top edge. Equivalent to y().
        fn top(self: &QRectF) -> f64;

        /// Returns the position of the rectangle's top-left corner.
        #[rust_name = "top_left"]
        fn topLeft(self: &QRectF) -> QPointF;

        /// Returns the position of the rectangle's top-right corner.
        #[rust_name = "top_right"]
        fn topRight(self: &QRectF) -> QPointF;

        /// Moves the rectangle offset.x() along the x axis and offset.y() along the y axis, relative to the current position.
        fn translate(self: &mut QRectF, offset: &QPointF);

        /// Returns a copy of the rectangle that is translated offset.x() along the x axis and offset.y() along the y axis, relative to the current position.
        fn translated(self: &QRectF, offset: &QPointF) -> QRectF;

        /// Returns a copy of the rectangle that has its width and height exchanged.
        fn transposed(self: &QRectF) -> QRectF;

        /// Returns the bounding rectangle of this rectangle and the given rectangle.
        fn united(self: &QRectF, rectangle: &QRectF) -> QRectF;

        /// Returns the width of the rectangle.
        fn width(self: &QRectF) -> f64;

        /// Returns the x-coordinate of the rectangle's left edge.
        fn x(self: &QRectF) -> f64;

        /// Returns the y-coordinate of the rectangle's top edge.
        fn y(self: &QRectF) -> f64;
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
        #[doc(hidden)]
        #[rust_name = "qrectf_from_qrect"]
        fn construct(rectangle: &QRect) -> QRectF;
        #[doc(hidden)]
        #[rust_name = "qrectf_to_qstring"]
        fn toQString(value: &QRectF) -> QString;
        #[doc(hidden)]
        #[rust_name = "qrectf_plus"]
        fn operatorPlus(a: &QRectF, b: &QMarginsF) -> QRectF;
        #[doc(hidden)]
        #[rust_name = "qrectf_minus"]
        fn operatorMinus(a: &QRectF, b: &QMarginsF) -> QRectF;
    }
}

/// The QRectF struct defines a rectangle in the plane using floating point precision.
#[derive(Debug, Clone, PartialEq)]
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

impl fmt::Display for QRectF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qrectf_to_qstring(self))
    }
}

impl From<&ffi::QRect> for QRectF {
    /// Constructs a QRectF rectangle from the given QRect rectangle.
    fn from(rectangle: &ffi::QRect) -> Self {
        ffi::qrectf_from_qrect(rectangle)
    }
}

impl From<&QRectF> for ffi::QRect {
    /// Returns a QRect based on the values of this rectangle.
    /// Note that the coordinates in the returned rectangle are rounded to the nearest integer.
    fn from(value: &QRectF) -> Self {
        value.to_rect()
    }
}

impl std::ops::Add<ffi::QMarginsF> for QRectF {
    type Output = Self;
    fn add(self, other: ffi::QMarginsF) -> Self {
        ffi::qrectf_plus(&self, &other)
    }
}

impl std::ops::Sub<ffi::QMarginsF> for QRectF {
    type Output = Self;
    fn sub(self, other: ffi::QMarginsF) -> Self {
        ffi::qrectf_minus(&self, &other)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QRectF is trivial.
unsafe impl ExternType for QRectF {
    type Id = type_id!("QRectF");
    type Kind = cxx::kind::Trivial;
}
