// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

use crate::{QMarginsF, QRect};

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qmarginsf.h");
        type QMarginsF = crate::QMarginsF;
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = crate::QPointF;
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;
        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = crate::QSizeF;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qrectf.h");
    }

    unsafe extern "C++" {
        type QRectF = super::QRectF;

        /// Adds `dx1`, `dy1`, `dx2` and `dy2` respectively to the existing coordinates of the rectangle. All parameters must be finite.
        fn adjust(&mut self, dx1: f64, dy1: f64, dx2: f64, dy2: f64);

        /// Returns a new rectangle with `dx1`, `dy1`, `dx2` and `dy2` added respectively to the existing coordinates of this rectangle. All parameters must be finite.
        fn adjusted(&self, dx1: f64, dy1: f64, dx2: f64, dy2: f64) -> QRectF;

        /// Returns the y-coordinate of the rectangle's bottom edge.
        fn bottom(&self) -> f64;

        /// Returns the position of the rectangle's bottom-left corner.
        #[rust_name = "bottom_left"]
        fn bottomLeft(&self) -> QPointF;

        /// Returns the position of the rectangle's bottom-right corner.
        #[rust_name = "bottom_right"]
        fn bottomRight(&self) -> QPointF;

        /// Returns the center point of the rectangle.
        fn center(&self) -> QPointF;

        /// Returns `true` if the given point is inside or on the edge of the rectangle; otherwise returns `false`.
        fn contains(&self, point: &QPointF) -> bool;

        /// Returns the height of the rectangle.
        fn height(&self) -> f64;

        /// Returns the intersection of this rectangle and the given rectangle. Note that `r.intersected(s)` is equivalent to `r & s`.
        fn intersected(&self, rectangle: &QRectF) -> QRectF;

        /// Returns `true` if this rectangle intersects with the given rectangle (i.e. there is a non-empty area of overlap between them), otherwise returns `false`.
        fn intersects(&self, rectangle: &QRectF) -> bool;

        /// Returns `true` if the rectangle is empty, otherwise returns `false`.
        ///
        /// An empty rectangle has `self.width() <= 0` or `self.height() <= 0`. An empty rectangle is not valid (i.e., `self.is_empty() == !self.is_valid()`).
        #[rust_name = "is_empty"]
        fn isEmpty(&self) -> bool;

        /// Returns `true` if the rectangle is a null rectangle, otherwise returns `false`.
        ///
        /// A null rectangle has both the width and the height set to 0. A null rectangle is also empty, and hence not valid.
        #[rust_name = "is_null"]
        fn isNull(&self) -> bool;

        /// Returns `true` if the rectangle is valid, otherwise returns `false`.
        ///
        /// A valid rectangle has a `self.width() > 0` and `self.height() > 0`. Note that non-trivial operations like intersections are not defined for invalid rectangles. A valid rectangle is not empty (i.e., `self.is_valid() == !self.is_empty()`).
        #[rust_name = "is_valid"]
        fn isValid(&self) -> bool;

        /// Returns the x-coordinate of the rectangle's left edge. Equivalent to `self.x()`.
        fn left(&self) -> f64;

        /// Returns a rectangle grown by the `margins`.
        #[rust_name = "margins_added"]
        fn marginsAdded(&self, margins: &QMarginsF) -> QRectF;

        /// Removes the `margins` from the rectangle, shrinking it.
        #[rust_name = "margins_removed"]
        fn marginsRemoved(&self, margins: &QMarginsF) -> QRectF;

        /// Moves the rectangle vertically, leaving the rectangle's bottom edge at the given finite `y` coordinate. The rectangle's size is unchanged.
        #[rust_name = "move_bottom"]
        fn moveBottom(&mut self, y: f64);

        /// Moves the rectangle, leaving the bottom-left corner at the given `position`. The rectangle's size is unchanged.
        #[rust_name = "move_bottom_left"]
        fn moveBottomLeft(&mut self, position: &QPointF);

        /// Moves the rectangle, leaving the bottom-right corner at the given `position`. The rectangle's size is unchanged.
        #[rust_name = "move_bottom_right"]
        fn moveBottomRight(&mut self, position: &QPointF);

        /// Moves the rectangle, leaving the center point at the given `position`. The rectangle's size is unchanged.
        #[rust_name = "move_center"]
        fn moveCenter(&mut self, position: &QPointF);

        /// Moves the rectangle horizontally, leaving the rectangle's left edge at the given finite `x` coordinate. The rectangle's size is unchanged.
        #[rust_name = "move_left"]
        fn moveLeft(&mut self, x: f64);

        /// Moves the rectangle horizontally, leaving the rectangle's right edge at the given finite `x` coordinate. The rectangle's size is unchanged.
        #[rust_name = "move_right"]
        fn moveRight(&mut self, x: f64);

        /// Moves the rectangle, leaving the top-left corner at the given `position`.
        #[rust_name = "move_to"]
        fn moveTo(&mut self, position: &QPointF);

        /// Moves the rectangle vertically, leaving the rectangle's top line at the given finite `y` coordinate. The rectangle's size is unchanged.
        #[rust_name = "move_top"]
        fn moveTop(&mut self, y: f64);

        /// Moves the rectangle, leaving the top-left corner at the given `position`. The rectangle's size is unchanged.
        #[rust_name = "move_top_left"]
        fn moveTopLeft(&mut self, position: &QPointF);

        /// Moves the rectangle, leaving the top-right corner at the given `position`. The rectangle's size is unchanged.
        #[rust_name = "move_top_right"]
        fn moveTopRight(&mut self, position: &QPointF);

        /// Returns a normalized rectangle; i.e., a rectangle that has a non-negative width and height.
        fn normalized(&self) -> QRectF;

        /// Returns the x-coordinate of the rectangle's right edge.
        fn right(&self) -> f64;

        /// Sets the bottom edge of the rectangle to the given finite `y` coordinate.
        /// May change the height, but will never change the top edge of the rectangle.
        #[rust_name = "set_bottom"]
        fn setBottom(&mut self, y: f64);

        /// Set the bottom-left corner of the rectangle to the given `position`.
        /// May change the size, but will never change the top-right corner of the rectangle.
        #[rust_name = "set_bottom_left"]
        fn setBottomLeft(&mut self, position: &QPointF);

        /// Set the bottom-right corner of the rectangle to the given `position`.
        /// May change the size, but will never change the top-left corner of the rectangle.
        #[rust_name = "set_bottom_right"]
        fn setBottomRight(&mut self, position: &QPointF);

        /// Sets the coordinates of the rectangle's top-left corner to (`x1`, `y1`),
        /// and the coordinates of its bottom-right corner to (`x2`, `y2`). All parameters must be finite.
        #[rust_name = "set_coords"]
        fn setCoords(&mut self, x1: f64, y1: f64, x2: f64, y2: f64);

        /// Sets the height of the rectangle to the given finite `height`. The bottom edge is changed, but not the top one.
        #[rust_name = "set_height"]
        fn setHeight(&mut self, h: f64);

        /// Sets the left edge of the rectangle to the given finite `x` coordinate.
        /// May change the width, but will never change the right edge of the rectangle.
        #[rust_name = "set_left"]
        fn setLeft(&mut self, x: f64);

        /// Sets the coordinates of the rectangle's top-left corner to (`x`, `y`), and its size to the given `width` and `height`. All parameters must be finite.
        #[rust_name = "set_rect"]
        fn setRect(&mut self, x: f64, y: f64, width: f64, height: f64);

        /// Sets the right edge of the rectangle to the given finite `x` coordinate.
        /// May change the width, but will never change the left edge of the rectangle.
        #[rust_name = "set_right"]
        fn setRight(&mut self, x: f64);

        /// Sets the size of the rectangle to the given finite `size`. The top-left corner is not moved.
        #[rust_name = "set_size"]
        fn setSize(&mut self, size: &QSizeF);

        /// Sets the top edge of the rectangle to the given finite `y` coordinate.
        /// May change the height, but will never change the bottom edge of the rectangle.
        #[rust_name = "set_top"]
        fn setTop(&mut self, y: f64);

        /// Set the top-left corner of the rectangle to the given `position`.
        /// May change the size, but will never change the bottom-right corner of the rectangle.
        #[rust_name = "set_top_left"]
        fn setTopLeft(&mut self, position: &QPointF);

        /// Set the top-right corner of the rectangle to the given `position`.
        /// May change the size, but will never change the bottom-left corner of the rectangle.
        #[rust_name = "set_top_right"]
        fn setTopRight(&mut self, position: &QPointF);

        /// Sets the width of the rectangle to the given finite `width`. The right edge is changed, but not the left one.
        #[rust_name = "set_width"]
        fn setWidth(&mut self, w: f64);

        /// Sets the left edge of the rectangle to the given finite `x` coordinate.
        /// May change the width, but will never change the right edge of the rectangle.
        #[rust_name = "set_x"]
        fn setX(&mut self, x: f64);

        /// Sets the top edge of the rectangle to the given finite `y` coordinate.
        /// May change the height, but will never change the bottom edge of the rectangle.
        #[rust_name = "set_y"]
        fn setY(&mut self, y: f64);

        /// Returns the size of the rectangle.
        fn size(&self) -> QSizeF;

        /// Returns a `QRect` based on the values of this rectangle that is the smallest possible integer rectangle that completely contains this rectangle.
        #[rust_name = "to_aligned_rect"]
        fn toAlignedRect(&self) -> QRect;

        /// Returns a `QRect` based on the values of this rectangle.
        /// Note that the coordinates in the returned rectangle are rounded to the nearest integer.
        #[rust_name = "to_rect"]
        fn toRect(&self) -> QRect;

        /// Returns the y-coordinate of the rectangle's top edge. Equivalent to `self.y()`.
        fn top(&self) -> f64;

        /// Returns the position of the rectangle's top-left corner.
        #[rust_name = "top_left"]
        fn topLeft(&self) -> QPointF;

        /// Returns the position of the rectangle's top-right corner.
        #[rust_name = "top_right"]
        fn topRight(&self) -> QPointF;

        /// Moves the rectangle `offset.x()` along the x axis and `offset.y()` along the y axis, relative to the current position.
        fn translate(&mut self, offset: &QPointF);

        /// Returns a copy of the rectangle that is translated `offset.x()` along the x axis and `offset.y()` along the y axis, relative to the current position.
        fn translated(&self, offset: &QPointF) -> QRectF;

        /// Returns a copy of the rectangle that has its width and height exchanged.
        fn transposed(&self) -> QRectF;

        /// Returns the bounding rectangle of this rectangle and the given `rectangle`.
        fn united(&self, rectangle: &QRectF) -> QRectF;

        /// Returns the width of the rectangle.
        fn width(&self) -> f64;

        /// Returns the x-coordinate of the rectangle's left edge.
        fn x(&self) -> f64;

        /// Returns the y-coordinate of the rectangle's top edge.
        fn y(&self) -> f64;
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
        #[rust_name = "qrectf_to_debug_qstring"]
        fn toDebugQString(value: &QRectF) -> QString;
        #[doc(hidden)]
        #[rust_name = "qrectf_plus"]
        fn operatorPlus(a: &QRectF, b: &QMarginsF) -> QRectF;
        #[doc(hidden)]
        #[rust_name = "qrectf_minus"]
        fn operatorMinus(a: &QRectF, b: &QMarginsF) -> QRectF;
    }
}

/// The `QRectF` class defines a rectangle in the plane using floating point precision.
///
/// Qt Documentation: [QRectF](https://doc.qt.io/qt/qrectf.html#details)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct QRectF {
    xp: f64,
    yp: f64,
    w: f64,
    h: f64,
}

impl QRectF {
    /// Constructs a rectangle with (`x`, `y`) as its top-left corner and the given `width` and `height`.
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
        ffi::qrectf_to_debug_qstring(self).fmt(f)
    }
}

impl From<&QRect> for QRectF {
    /// Constructs a `QRectF` rectangle from the given `QRect` rectangle.
    fn from(rectangle: &QRect) -> Self {
        ffi::qrectf_from_qrect(rectangle)
    }
}
impl From<QRect> for QRectF {
    /// Constructs a `QRectF` rectangle from the given `QRect` rectangle.
    fn from(rectangle: QRect) -> Self {
        Self::from(&rectangle)
    }
}

impl From<&QRectF> for QRect {
    /// Returns a `QRect` based on the values of this rectangle.
    /// Note that the coordinates in the returned rectangle are rounded to the nearest integer.
    fn from(rectangle: &QRectF) -> Self {
        rectangle.to_rect()
    }
}
impl From<QRectF> for QRect {
    /// Returns a `QRect` based on the values of this rectangle.
    /// Note that the coordinates in the returned rectangle are rounded to the nearest integer.
    fn from(rectangle: QRectF) -> Self {
        Self::from(&rectangle)
    }
}

impl std::ops::Add<QMarginsF> for QRectF {
    type Output = Self;
    fn add(self, other: QMarginsF) -> Self {
        ffi::qrectf_plus(&self, &other)
    }
}

impl std::ops::Sub<QMarginsF> for QRectF {
    type Output = Self;
    fn sub(self, other: QMarginsF) -> Self {
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
