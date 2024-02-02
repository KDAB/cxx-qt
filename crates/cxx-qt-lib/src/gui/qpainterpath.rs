// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::QPointF;
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type FillRule = crate::FillRule;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qpainterpath.h");
        type QPainterPath = super::QPainterPath;
        include!("cxx-qt-lib/qrectf.h");
        type QRectF = crate::QRectF;
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = crate::QPointF;
        include!("cxx-qt-lib/qfont.h");
        type QFont = crate::QFont;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qpolygonf.h");
        type QPolygonF = crate::QPolygonF;

        /// Creates an ellipse within the specified boundingRectangle and adds it to the painter
        /// path as a closed subpath.
        #[rust_name = "add_ellipse"]
        fn addEllipse(self: &mut QPainterPath, boundingRectangle: &QRectF);

        /// Adds the given path to this path as a closed subpath.
        #[rust_name = "add_path"]
        fn addPath(self: &mut QPainterPath, path: &QPainterPath);

        /// Adds the given polygon to the path as an (unclosed) subpath.
        #[rust_name = "add_polygon"]
        fn addPolygon(self: &mut QPainterPath, polygon: &QPolygonF);

        /// Adds the given rectangle to this path as a closed subpath.
        #[rust_name = "add_rect"]
        fn addRect(self: &mut QPainterPath, rectangle: &QRectF);

        /// Creates a move to that lies on the arc that occupies the given rectangle at angle.
        #[rust_name = "arc_move_to"]
        fn arcMoveTo(self: &mut QPainterPath, rectangle: &QRectF, angle: f64);

        /// Adds the given text to this path as a set of closed subpaths created from the font supplied.
        /// The subpaths are positioned so that the left end of the text's baseline lies at the specified point.
        #[rust_name = "add_text"]
        fn addText(self: &mut QPainterPath, point: &QPointF, font: &QFont, text: &QString);

        /// Creates an arc that occupies the given rectangle, beginning at the specified
        /// startAngle and extending sweepLength degrees counter-clockwise.
        #[rust_name = "arc_to"]
        fn arcTo(self: &mut QPainterPath, rectangle: &QRectF, startAngle: f64, sweepLength: f64);

        /// Returns the bounding rectangle of this painter path as a rectangle with floating point precision.
        #[rust_name = "bounding_rect"]
        fn boundingRect(self: &QPainterPath) -> QRectF;

        /// Returns the number of elements allocated by the QPainterPath.
        fn capacity(self: &QPainterPath) -> i32;

        /// Clears the path elements stored.
        fn clear(self: &mut QPainterPath);

        /// Closes the current subpath by drawing a line to the beginning of the subpath,
        /// automatically starting a new path. The current point of the new path is (0, 0).
        #[rust_name = "close_subpath"]
        fn closeSubpath(self: &mut QPainterPath);

        /// Connects the given path to this path by adding a line from the last element of
        /// this path to the first element of the given path.
        #[rust_name = "connect_path"]
        fn connectPath(self: &mut QPainterPath, path: &QPainterPath);

        /// Returns true if the given point is inside the path, otherwise returns false.
        fn contains(self: &QPainterPath, point: &QPointF) -> bool;

        /// Returns the rectangle containing all the points and control points in this path.
        #[rust_name = "control_point_rect"]
        fn controlPointRect(self: &QPainterPath) -> QRectF;

        /// Adds a cubic Bezier curve between the current position and the given endPoint using the control
        /// points specified by c1, and c2.
        /// After the curve is added, the current position is updated to be at the end point of the curve.
        #[rust_name = "cubic_to"]
        fn cubicTo(self: &mut QPainterPath, c1: &QPointF, c2: &QPointF, endPoint: &QPointF);

        /// Returns the current position of the path.
        #[rust_name = "current_position"]
        fn currentPosition(self: &QPainterPath) -> QPointF;

        /// Returns the number of path elements in the painter path.
        #[rust_name = "element_count"]
        fn elementCount(self: &QPainterPath) -> i32;

        /// Returns the painter path's currently set fill rule.
        #[rust_name = "fill_rule"]
        fn fillRule(self: &QPainterPath) -> FillRule;

        /// Returns a path which is the intersection of this path's fill area and p's fill area.
        /// Bezier curves may be flattened to line segments due to numerical instability of doing bezier curve intersections.
        fn intersected(self: &QPainterPath, p: &QPainterPath) -> QPainterPath;

        /// Returns true if the current path intersects at any point the given path p.
        /// Also returns true if the current path contains or is contained by any part of p.
        fn intersects(self: &QPainterPath, p: &QPainterPath) -> bool;

        /// Returns true if either there are no elements in this path,
        /// or if the only element is a MoveToElement; otherwise returns false.
        #[rust_name = "is_empty"]
        fn isEmpty(self: &QPainterPath) -> bool;

        /// Returns the length of the current path.
        fn length(self: &QPainterPath) -> f64;

        /// Adds a straight line from the current position to the given endPoint.
        /// After the line is drawn, the current position is updated to be at the end point of the line.
        #[rust_name = "line_to"]
        fn lineTo(self: &mut QPainterPath, endPoint: &QPointF);

        /// Adds a quadratic Bezier curve between the current position and the given endPoint
        /// with the control point specified by c.
        /// After the curve is added, the current point is updated to be at the end point of the curve.
        #[rust_name = "quad_to"]
        fn quadTo(self: &mut QPainterPath, c: &QPointF, endPoint: &QPointF);

        /// Reserves a given amount of elements in QPainterPath's internal memory.
        fn reserve(self: &mut QPainterPath, size: i32);

        /// Sets the fill rule of the painter path to the given fillRule. Qt provides two methods for filling paths:
        #[rust_name = "set_fill_rule"]
        fn setFillRule(self: &mut QPainterPath, fillRule: FillRule);

        /// Returns a simplified version of this path.
        fn simplified(self: &QPainterPath) -> QPainterPath;

        /// Returns a path which is p's fill area subtracted from this path's fill area.
        fn subtracted(self: &QPainterPath, painterpath: &QPainterPath) -> QPainterPath;

        /// Translates all elements in the path by (dx, dy).
        fn translate(self: &mut QPainterPath, dx: f64, dy: f64);

        /// Returns a copy of the path that is translated by the given offset.
        fn translated(self: &QPainterPath, offset: &QPointF) -> QPainterPath;

        /// Creates and returns a reversed copy of the path.
        #[rust_name = "to_reversed"]
        fn toReversed(self: &QPainterPath) -> QPainterPath;

        /// Returns a path which is the union of this path's fill area and p's fill area.
        fn united(self: &QPainterPath, painterpath: &QPainterPath) -> QPainterPath;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qpainterpath_init_default"]
        fn construct() -> QPainterPath;

        #[doc(hidden)]
        #[rust_name = "qpainterpath_clone"]
        fn construct(path: &QPainterPath) -> QPainterPath;

        #[doc(hidden)]
        #[rust_name = "qpainterpath_from_qpointf"]
        fn construct(point: &QPointF) -> QPainterPath;

        #[doc(hidden)]
        #[rust_name = "qpainterpath_drop"]
        fn drop(painterPath: &mut QPainterPath);
    }
}

#[repr(C)]
pub struct QPainterPath {
    _cspec: MaybeUninit<i32>,
}

impl Default for QPainterPath {
    fn default() -> Self {
        ffi::qpainterpath_init_default()
    }
}

impl Drop for QPainterPath {
    fn drop(&mut self) {
        ffi::qpainterpath_drop(self);
    }
}

impl Clone for QPainterPath {
    /// Constructs a copy of other.
    fn clone(&self) -> Self {
        ffi::qpainterpath_clone(self)
    }
}

impl From<&QPointF> for QPainterPath {
    /// Creates a QPainterPath object with the given startPoint as its current position.
    fn from(p: &QPointF) -> Self {
        ffi::qpainterpath_from_qpointf(p)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QPainterPath {
    type Id = type_id!("QPainterPath");
    type Kind = cxx::kind::Trivial;
}
