// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::QPointF;
use cxx::{type_id, ExternType};
use std::fmt;
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type FillRule = crate::FillRule;
        type SizeMode = crate::SizeMode;
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
        include!("cxx-qt-lib/qregion.h");
        type QRegion = crate::QRegion;

        /// Creates an ellipse within the specified `bounding_rectangle` and adds it to the painter
        /// path as a closed subpath.
        ///
        /// The ellipse is composed of a clockwise curve, starting and finishing at zero degrees (the 3 o'clock position).
        #[rust_name = "add_ellipse"]
        fn addEllipse(self: &mut QPainterPath, bounding_rectangle: &QRectF);

        /// Adds the given `path` to this path as a closed subpath.
        #[rust_name = "add_path"]
        fn addPath(self: &mut QPainterPath, path: &QPainterPath);

        /// Adds the given `polygon` to the path as an (unclosed) subpath.
        ///
        /// Note that the current position after the polygon has been added, is the last point in `polygon`. To draw a line back to the first point, use [`close_subpath`](Self::close_subpath).
        #[rust_name = "add_polygon"]
        fn addPolygon(self: &mut QPainterPath, polygon: &QPolygonF);

        /// Adds the given `rectangle` to this path as a closed subpath.
        ///
        /// The `rectangle` is added as a clockwise set of lines. The painter path's current position after the `rectangle` has been added is at the top-left corner of the rectangle.
        #[rust_name = "add_rect"]
        fn addRect(self: &mut QPainterPath, rectangle: &QRectF);

        /// Adds the given `region` to the path by adding each rectangle in the region as a separate closed subpath.
        #[rust_name = "add_region"]
        fn addRegion(self: &mut QPainterPath, region: &QRegion);

        /// Adds the given rectangle `rect` with rounded corners to the path.
        ///
        /// The `x_radius` and `y_radius` arguments specify the radii of the ellipses defining the corners of the rounded rectangle. When `mode` is [`SizeMode::RelativeSize`], `x_radius` and `y_radius` are specified in percentage of half the rectangle's width and height respectively, and should be in the range 0.0 to 100.0.
        #[rust_name = "add_rounded_rect"]
        fn addRoundedRect(
            self: &mut QPainterPath,
            rect: &QRectF,
            x_radius: f64,
            y_radius: f64,
            mode: SizeMode,
        );

        /// Creates a move to that lies on the arc that occupies the given `rectangle` at `angle`.
        ///
        /// Angles are specified in degrees. Clockwise arcs can be specified using negative angles.
        #[rust_name = "arc_move_to"]
        fn arcMoveTo(self: &mut QPainterPath, rectangle: &QRectF, angle: f64);

        /// Adds the given `text` to this path as a set of closed subpaths created from the `font` supplied.
        /// The subpaths are positioned so that the left end of the text's baseline lies at the specified `point`.
        ///
        /// Some fonts may yield overlapping subpaths and will require the [`FillRule::WindingFill`] fill rule for correct rendering.
        #[rust_name = "add_text"]
        fn addText(self: &mut QPainterPath, point: &QPointF, font: &QFont, text: &QString);

        /// Creates an arc that occupies the given `rectangle`, beginning at the specified
        /// `start_angle` and extending `sweep_length` degrees counter-clockwise.
        ///
        /// Angles are specified in degrees. Clockwise arcs can be specified using negative angles.
        ///
        /// Note that this function connects the starting point of the arc to the current position if they are not already connected. After the arc has been added, the current position is the last point in arc. To draw a line back to the first point, use [`close_subpath`](Self::close_subpath).
        #[rust_name = "arc_to"]
        fn arcTo(self: &mut QPainterPath, rectangle: &QRectF, start_angle: f64, sweep_length: f64);

        /// Returns the bounding rectangle of this painter path as a rectangle with floating point precision.
        #[rust_name = "bounding_rect"]
        fn boundingRect(self: &QPainterPath) -> QRectF;

        /// Returns the number of elements allocated by the `QPainterPath`.
        fn capacity(self: &QPainterPath) -> i32;

        /// Clears the path elements stored.
        ///
        /// This allows the path to reuse previous memory allocations.
        fn clear(self: &mut QPainterPath);

        /// Closes the current subpath by drawing a line to the beginning of the subpath,
        /// automatically starting a new path. The current point of the new path is (0, 0).
        ///
        /// If the subpath does not contain any elements, this function does nothing.
        #[rust_name = "close_subpath"]
        fn closeSubpath(self: &mut QPainterPath);

        /// Connects the given `path` to this path by adding a line from the last element of
        /// this path to the first element of the given path.
        #[rust_name = "connect_path"]
        fn connectPath(self: &mut QPainterPath, path: &QPainterPath);

        /// Returns `true` if the given `point` is inside the path, otherwise returns `false`.
        fn contains(self: &QPainterPath, point: &QPointF) -> bool;

        /// Returns the rectangle containing all the points and control points in this path.
        ///
        /// This function is significantly faster to compute than the exact [`bounding_rect`](Self::bounding_rect), and the returned rectangle is always a superset of the rectangle returned by [`bounding_rect`](Self::bounding_rect).
        #[rust_name = "control_point_rect"]
        fn controlPointRect(self: &QPainterPath) -> QRectF;

        /// Adds a cubic Bezier curve between the current position and the given `end_point` using the control
        /// points specified by `c1`, and `c2`.
        /// After the curve is added, the current position is updated to be at the end point of the curve.
        #[rust_name = "cubic_to"]
        fn cubicTo(self: &mut QPainterPath, c1: &QPointF, c2: &QPointF, end_point: &QPointF);

        /// Returns the current position of the path.
        #[rust_name = "current_position"]
        fn currentPosition(self: &QPainterPath) -> QPointF;

        /// Returns the number of path elements in the painter path.
        #[rust_name = "element_count"]
        fn elementCount(self: &QPainterPath) -> i32;

        /// Returns the painter path's currently set fill rule.
        #[rust_name = "fill_rule"]
        fn fillRule(self: &QPainterPath) -> FillRule;

        /// Returns a path which is the intersection of this path's fill area and `p`'s fill area.
        /// Bezier curves may be flattened to line segments due to numerical instability of doing bezier curve intersections.
        fn intersected(self: &QPainterPath, p: &QPainterPath) -> QPainterPath;

        /// Returns `true` if the current path intersects at any point the given path `p`.
        /// Also returns `true` if the current path contains or is contained by any part of `p`.
        fn intersects(self: &QPainterPath, p: &QPainterPath) -> bool;

        /// Returns `true` if either there are no elements in this path,
        /// or if the only element is a [MoveToElement](https://doc.qt.io/qt/qpainterpath.html#ElementType-enum); otherwise returns `false`.
        #[rust_name = "is_empty"]
        fn isEmpty(self: &QPainterPath) -> bool;

        /// Returns the length of the current path.
        fn length(self: &QPainterPath) -> f64;

        /// Adds a straight line from the current position to the given `end_point`.
        /// After the line is drawn, the current position is updated to be at the end point of the line.
        #[rust_name = "line_to"]
        fn lineTo(self: &mut QPainterPath, end_point: &QPointF);

        /// Adds a quadratic Bezier curve between the current position and the given `end_point`
        /// with the control point specified by `c`.
        /// After the curve is added, the current point is updated to be at the end point of the curve.
        #[rust_name = "quad_to"]
        fn quadTo(self: &mut QPainterPath, c: &QPointF, endPoint: &QPointF);

        /// Reserves a given amount of elements in `QPainterPath`'s internal memory.
        ///
        /// Attempts to allocate memory for at least `size` elements.
        fn reserve(self: &mut QPainterPath, size: i32);

        /// Sets the fill rule of the painter path to the given `fill_rule`.
        #[rust_name = "set_fill_rule"]
        fn setFillRule(self: &mut QPainterPath, fill_rule: FillRule);

        /// Returns a simplified version of this path.
        /// This implies merging all subpaths that intersect, and returning a path containing no intersecting edges. Consecutive parallel lines will also be merged. The simplified path will always use the default fill rule, [`FillRule::OddEvenFill`]. Bezier curves may be flattened to line segments due to numerical instability of doing bezier curve intersections.
        fn simplified(self: &QPainterPath) -> QPainterPath;

        /// Returns a path which is `p`'s fill area subtracted from this path's fill area.
        ///
        /// Set operations on paths will treat the paths as areas. Non-closed paths will be treated as implicitly closed. Bezier curves may be flattened to line segments due to numerical instability of doing bezier curve intersections.
        fn subtracted(self: &QPainterPath, painterpath: &QPainterPath) -> QPainterPath;

        /// Translates all elements in the path by (`dx`, `dy`).
        fn translate(self: &mut QPainterPath, dx: f64, dy: f64);

        /// Returns a copy of the path that is translated by the given `offset`.
        fn translated(self: &QPainterPath, offset: &QPointF) -> QPainterPath;

        /// Creates and returns a reversed copy of the path.
        ///
        /// It is the order of the elements that is reversed: If a `QPainterPath` is composed by calling the moveTo(), lineTo() and cubicTo() functions in the specified order, the reversed copy is composed by calling cubicTo(), lineTo() and moveTo().
        #[rust_name = "to_reversed"]
        fn toReversed(self: &QPainterPath) -> QPainterPath;

        /// Returns a path which is the union of this path's fill area and `p`'s fill area.
        fn united(self: &QPainterPath, p: &QPainterPath) -> QPainterPath;
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

        #[doc(hidden)]
        #[rust_name = "qpainterpath_eq"]
        fn operatorEq(a: &QPainterPath, b: &QPainterPath) -> bool;

        #[doc(hidden)]
        #[rust_name = "qpainterpath_to_debug_qstring"]
        fn toDebugQString(value: &QPainterPath) -> QString;
    }
}

/// The `QPainterPath` class provides a container for painting operations, enabling graphical shapes to be constructed and reused.
///
/// Qt Documentation: [QPainterPath](https://doc.qt.io/qt/qpainterpath.html#details)
#[repr(C)]
pub struct QPainterPath {
    _cspec: MaybeUninit<usize>,
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
    /// Constructs a copy of this `QPainterPath`.
    fn clone(&self) -> Self {
        ffi::qpainterpath_clone(self)
    }
}

impl From<&QPointF> for QPainterPath {
    /// Creates a `QPainterPath` object with the given `start_point` as its current position.
    fn from(start_point: &QPointF) -> Self {
        ffi::qpainterpath_from_qpointf(start_point)
    }
}

impl PartialEq for QPainterPath {
    fn eq(&self, other: &Self) -> bool {
        ffi::qpainterpath_eq(self, other)
    }
}

impl fmt::Display for QPainterPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qpainterpath_to_debug_qstring(self).fmt(f)
    }
}

impl fmt::Debug for QPainterPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qpainterpath_to_debug_qstring(self).fmt(f)
    }
}

impl Eq for QPainterPath {}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QPainterPath {
    type Id = type_id!("QPainterPath");
    type Kind = cxx::kind::Trivial;
}
