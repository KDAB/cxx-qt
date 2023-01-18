// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmargins.h");
        type QMargins = crate::QMargins;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;
        include!("cxx-qt-lib/qrect.h");
        type QRect = super::QRect;
        include!("cxx-qt-lib/qsize.h");
        type QSize = crate::QSize;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Adds dx1, dy1, dx2 and dy2 respectively to the existing coordinates of the rectangle.
        fn adjust(self: &mut QRect, dx1: i32, dy1: i32, dx2: i32, dy2: i32);

        /// Returns a new rectangle with dx1, dy1, dx2 and dy2 added respectively to the existing coordinates of this rectangle.
        fn adjusted(self: &QRect, dx1: i32, dy1: i32, dx2: i32, dy2: i32) -> QRect;

        /// Returns the y-coordinate of the rectangle's bottom edge.
        ///
        /// Note that for historical reasons this function returns top() + height() - 1; use y() + height() to retrieve the true y-coordinate.
        fn bottom(self: &QRect) -> i32;

        /// Returns the position of the rectangle's bottom-left corner.
        ///
        /// Note that for historical reasons this function returns QPoint(left(), top() + height() - 1).
        #[rust_name = "bottom_left"]
        fn bottomLeft(self: &QRect) -> QPoint;

        /// Returns the position of the rectangle's bottom-right corner.
        ///
        /// Note that for historical reasons this function returns QPoint(left() + width() -1, top() + height() - 1).
        #[rust_name = "bottom_right"]
        fn bottomRight(self: &QRect) -> QPoint;

        /// Returns the center point of the rectangle.
        fn center(self: &QRect) -> QPoint;

        /// Returns true if the given point is inside or on the edge of the rectangle, otherwise returns false.
        /// If proper is true, this function only returns true if the given point is inside the rectangle (i.e., not on the edge).
        fn contains(self: &QRect, point: &QPoint, proper: bool) -> bool;

        /// Returns the height of the rectangle.
        fn height(self: &QRect) -> i32;

        /// Returns the intersection of this rectangle and the given rectangle. Note that r.intersected(s) is equivalent to r & s.
        fn intersected(self: &QRect, rectangle: &QRect) -> QRect;

        /// Returns true if this rectangle intersects with the given rectangle (i.e., there is at least one pixel that is within both rectangles), otherwise returns false.
        fn intersects(self: &QRect, rectangle: &QRect) -> bool;

        /// Returns true if the rectangle is empty, otherwise returns false.
        ///
        /// An empty rectangle has a left() > right() or top() > bottom(). An empty rectangle is not valid (i.e., isEmpty() == !isValid()).
        #[rust_name = "is_empty"]
        fn isEmpty(self: &QRect) -> bool;

        /// Returns true if the rectangle is a null rectangle, otherwise returns false.
        ///
        /// A null rectangle has both the width and the height set to 0 (i.e., right() == left() - 1 and bottom() == top() - 1). A null rectangle is also empty, and hence is not valid.
        #[rust_name = "is_null"]
        fn isNull(self: &QRect) -> bool;

        /// Returns true if the rectangle is valid, otherwise returns false.
        ///
        /// A valid rectangle has a left() <= right() and top() <= bottom(). Note that non-trivial operations like intersections are not defined for invalid rectangles. A valid rectangle is not empty (i.e., isValid() == !isEmpty()).
        #[rust_name = "is_valid"]
        fn isValid(self: &QRect) -> bool;

        /// Returns the x-coordinate of the rectangle's left edge. Equivalent to x().
        fn left(self: &QRect) -> i32;

        /// Returns a rectangle grown by the margins.
        #[rust_name = "margins_added"]
        fn marginsAdded(self: &QRect, margins: &QMargins) -> QRect;

        /// Removes the margins from the rectangle, shrinking it.
        #[rust_name = "margins_removed"]
        fn marginsRemoved(self: &QRect, margins: &QMargins) -> QRect;

        /// Moves the rectangle vertically, leaving the rectangle's bottom edge at the given y coordinate. The rectangle's size is unchanged.
        #[rust_name = "move_bottom"]
        fn moveBottom(self: &mut QRect, y: i32);

        /// Moves the rectangle, leaving the bottom-left corner at the given position. The rectangle's size is unchanged.
        #[rust_name = "move_bottom_left"]
        fn moveBottomLeft(self: &mut QRect, position: &QPoint);

        /// Moves the rectangle, leaving the bottom-right corner at the given position. The rectangle's size is unchanged.
        #[rust_name = "move_bottom_right"]
        fn moveBottomRight(self: &mut QRect, position: &QPoint);

        /// Moves the rectangle, leaving the center point at the given position. The rectangle's size is unchanged.
        #[rust_name = "move_center"]
        fn moveCenter(self: &mut QRect, position: &QPoint);

        /// Moves the rectangle horizontally, leaving the rectangle's left edge at the given x coordinate. The rectangle's size is unchanged.
        #[rust_name = "move_left"]
        fn moveLeft(self: &mut QRect, x: i32);

        /// Moves the rectangle horizontally, leaving the rectangle's right edge at the given x coordinate. The rectangle's size is unchanged.
        #[rust_name = "move_right"]
        fn moveRight(self: &mut QRect, x: i32);

        /// Moves the rectangle, leaving the top-left corner at the given position.
        #[rust_name = "move_to"]
        fn moveTo(self: &mut QRect, position: &QPoint);

        /// Moves the rectangle vertically, leaving the rectangle's top edge at the given y coordinate. The rectangle's size is unchanged.
        #[rust_name = "move_top"]
        fn moveTop(self: &mut QRect, y: i32);

        /// Moves the rectangle, leaving the top-left corner at the given position. The rectangle's size is unchanged.
        #[rust_name = "move_top_left"]
        fn moveTopLeft(self: &mut QRect, position: &QPoint);

        /// Moves the rectangle, leaving the top-right corner at the given position. The rectangle's size is unchanged.
        #[rust_name = "move_top_right"]
        fn moveTopRight(self: &mut QRect, position: &QPoint);

        /// Returns a normalized rectangle; i.e., a rectangle that has a non-negative width and height.
        fn normalized(self: &QRect) -> QRect;

        /// Returns the x-coordinate of the rectangle's right edge.
        ///
        /// Note that for historical reasons this function returns left() + width() - 1; use x() + width() to retrieve the true x-coordinate.
        fn right(self: &QRect) -> i32;

        /// Sets the bottom edge of the rectangle to the given y coordinate.
        /// May change the height, but will never change the top edge of the rectangle.
        #[rust_name = "set_bottom"]
        fn setBottom(self: &mut QRect, y: i32);

        /// Set the bottom-left corner of the rectangle to the given position.
        /// May change the size, but will never change the top-right corner of the rectangle.
        #[rust_name = "set_bottom_left"]
        fn setBottomLeft(self: &mut QRect, position: &QPoint);

        /// Set the bottom-right corner of the rectangle to the given position.
        /// May change the size, but will never change the top-left corner of the rectangle.
        #[rust_name = "set_bottom_right"]
        fn setBottomRight(self: &mut QRect, position: &QPoint);

        /// Sets the coordinates of the rectangle's top-left corner to (x1, y1), and the coordinates of its bottom-right corner to (x2, y2).
        #[rust_name = "set_coords"]
        fn setCoords(self: &mut QRect, x1: i32, y1: i32, x2: i32, y2: i32);

        /// Sets the height of the rectangle to the given height. The bottom edge is changed, but not the top one.
        #[rust_name = "set_height"]
        fn setHeight(self: &mut QRect, h: i32);

        /// Sets the left edge of the rectangle to the given x coordinate. May change the width, but will never change the right edge of the rectangle.
        #[rust_name = "set_left"]
        fn setLeft(self: &mut QRect, x: i32);

        /// Sets the coordinates of the rectangle's top-left corner to (x, y), and its size to the given width and height.
        #[rust_name = "set_rect"]
        fn setRect(self: &mut QRect, x: i32, y: i32, width: i32, height: i32);

        /// Sets the right edge of the rectangle to the given x coordinate. May change the width, but will never change the left edge of the rectangle.
        #[rust_name = "set_right"]
        fn setRight(self: &mut QRect, x: i32);

        /// Sets the size of the rectangle to the given size. The top-left corner is not moved.
        #[rust_name = "set_size"]
        fn setSize(self: &mut QRect, size: &QSize);

        /// Sets the top edge of the rectangle to the given y coordinate. May change the height, but will never change the bottom edge of the rectangle.
        #[rust_name = "set_top"]
        fn setTop(self: &mut QRect, y: i32);

        /// Set the top-left corner of the rectangle to the given position. May change the size, but will never change the bottom-right corner of the rectangle.
        #[rust_name = "set_top_left"]
        fn setTopLeft(self: &mut QRect, position: &QPoint);

        /// Set the top-right corner of the rectangle to the given position. May change the size, but will never change the bottom-left corner of the rectangle.
        #[rust_name = "set_top_right"]
        fn setTopRight(self: &mut QRect, position: &QPoint);

        /// Sets the width of the rectangle to the given width. The right edge is changed, but not the left one.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QRect, w: i32);

        /// Sets the left edge of the rectangle to the given x coordinate. May change the width, but will never change the right edge of the rectangle.
        #[rust_name = "set_x"]
        fn setX(self: &mut QRect, x: i32);

        /// Sets the top edge of the rectangle to the given y coordinate. May change the height, but will never change the bottom edge of the rectangle.
        #[rust_name = "set_y"]
        fn setY(self: &mut QRect, y: i32);

        /// Returns the size of the rectangle.
        fn size(self: &QRect) -> QSize;

        /// Returns the y-coordinate of the rectangle's top edge. Equivalent to y().
        fn top(self: &QRect) -> i32;

        /// Returns the position of the rectangle's top-left corner.
        #[rust_name = "top_left"]
        fn topLeft(self: &QRect) -> QPoint;

        /// Returns the position of the rectangle's top-right corner.
        ///
        /// Note that for historical reasons this function returns QPoint(left() + width() -1, top()).
        #[rust_name = "top_right"]
        fn topRight(self: &QRect) -> QPoint;

        /// Moves the rectangle offset.x() along the x axis and offset.y() along the y axis, relative to the current position.
        fn translate(self: &mut QRect, offset: &QPoint);

        /// Returns a copy of the rectangle that is translated offset.x() along the x axis and offset.y() along the y axis, relative to the current position.
        fn translated(self: &QRect, offset: &QPoint) -> QRect;

        /// Returns a copy of the rectangle that has its width and height exchanged.
        fn transposed(self: &QRect) -> QRect;

        /// Returns the bounding rectangle of this rectangle and the given rectangle.
        fn united(self: &QRect, rectangle: &QRect) -> QRect;

        /// Returns the width of the rectangle.
        fn width(self: &QRect) -> i32;

        /// Returns the x-coordinate of the rectangle's left edge.
        fn x(self: &QRect) -> i32;

        /// Returns the y-coordinate of the rectangle's top edge.
        fn y(self: &QRect) -> i32;
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
        #[doc(hidden)]
        #[rust_name = "qrect_to_qstring"]
        fn toQString(value: &QRect) -> QString;
    }
}

/// The QRect struct defines a rectangle in the plane using integer precision.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl fmt::Display for QRect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qrect_to_qstring(self))
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QRect is trivial.
unsafe impl ExternType for QRect {
    type Id = type_id!("QRect");
    type Kind = cxx::kind::Trivial;
}
