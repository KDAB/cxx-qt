// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpainterpath.h");
        type QPainterPath = super::QPainterPath;
        include!("cxx-qt-lib/qrectf.h");
        type QRectF = crate::QRectF;
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = crate::QPointF;

        /// Creates an arc that occupies the given rectangle,
        /// beginning at the specified startAngle and extending sweepLength degrees counter-clockwise.
        #[rust_name = "arc_to"]
        fn arcTo(self: &mut QPainterPath, rectangle: &QRectF, startAngle: f64, sweepLength: f64);

        /// Creates an ellipse within the specified boundingRectangle and adds it to the painter
        /// path as a closed subpath.
        #[rust_name = "add_ellipse"]
        fn addEllipse(self: &mut QPainterPath, boundingRectangle: &QRectF);

        /// Adds the given path to this path as a closed subpath.
        #[rust_name = "add_path"]
        fn addPath(self: &mut QPainterPath, path: &QPainterPath);

        /// Adds the given rectangle to this path as a closed subpath.
        #[rust_name = "add_rect"]
        fn addRect(self: &mut QPainterPath, rectangle: &QRectF);

        /// Creates a move to that lies on the arc that occupies the given rectangle at angle.
        #[rust_name = "arc_move_to"]
        fn arcMoveTo(self: &mut QPainterPath, rectangle: &QRectF, angle: f64);

        /// Returns the bounding rectangle of this painter path as a rectangle with floating point precision.
        #[rust_name = "bounding_rect"]
        fn boundingRect(self: &QPainterPath) -> QRectF;

        /// Clears the path elements stored.
        fn clear(self: &mut QPainterPath);

        /// Closes the current subpath by drawing a line to the beginning of the subpath,
        /// automatically starting a new path. The current point of the new path is (0, 0).
        fn closeSubpath(self: &mut QPainterPath);

        /// Returns the current position of the path.
        #[rust_name = "current_position"]
        fn currentPosition(self: &QPainterPath) -> QPointF;

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

        /// Adds a straight line from the current position to the given endPoint.
        /// After the line is drawn, the current position is updated to be at the end point of the line.
        #[rust_name = "line_to"]
        fn lineTo(self: &mut QPainterPath, endPoint: &QPointF);

        /// Returns a simplified version of this path.
        fn simplified(self: &QPainterPath) -> QPainterPath;

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
        #[rust_name = "qpainterpath_drop"]
        fn drop(pen: &mut QPainterPath);
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

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QPainterPath {
    type Id = type_id!("QPainterPath");
    type Kind = cxx::kind::Trivial;
}
