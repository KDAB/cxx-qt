// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::QPointF;
use crate::QLine;
use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlinef.h");
        type QLineF = super::QLineF;
        include!("cxx-qt-lib/qline.h");
        type QLine = crate::QLine;
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = crate::QPointF;

        /// Returns the line's start point.
        fn p1(self: &QLineF) -> QPointF;

        /// Returns the line's end point.
        fn p2(self: &QLineF) -> QPointF;

        /// Returns the x-coordinate of the line's start point.
        fn x1(self: &QLineF) -> f64;

        /// Returns the x-coordinate of the line's end point.
        fn x2(self: &QLineF) -> f64;

        /// Returns the y-coordinate of the line's start point.
        fn y1(self: &QLineF) -> f64;

        /// Returns the y-coordinate of the line's end point.
        fn y2(self: &QLineF) -> f64;

        /// Returns the center point of this line. This is equivalent to (p1() + p2()) / 2, except it will never overflow.
        fn center(self: &QLineF) -> QPointF;

        /// Returns the horizontal component of the line's vector.
        fn dx(self: &QLineF) -> f64;

        /// Returns the vertical component of the line's vector.
        fn dy(self: &QLineF) -> f64;

        /// Returns true if the line does not have distinct start and end points; otherwise returns false.
        #[rust_name = "is_null"]
        fn isNull(self: &QLineF) -> bool;

        /// Sets the starting point of this line to p1.
        #[rust_name = "set_p1"]
        fn setP1(self: &mut QLineF, p1: &QPointF);

        /// Sets the end point of this line to p2.
        #[rust_name = "set_p2"]
        fn setP2(self: &mut QLineF, p1: &QPointF);

        /// Sets this line to the start in x1, y1 and end in x2, y2.
        #[rust_name = "set_line"]
        fn setLine(self: &mut QLineF, x1: f64, y1: f64, x2: f64, y2: f64);

        /// Sets the start point of this line to p1 and the end point of this line to p2.
        #[rust_name = "set_points"]
        fn setPoints(self: &mut QLineF, p1: &QPointF, p2: &QPointF);

        /// Translates this line by the given offset.
        fn translate(self: &mut QLineF, offset: &QPointF);

        /// Returns this line translated by the given offset.
        fn translated(self: &QLineF, offset: &QPointF) -> QLineF;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qlinef_default"]
        fn construct() -> QLineF;

        #[doc(hidden)]
        #[rust_name = "qlinef_new"]
        fn construct(pt1: QPointF, pt2: QPointF) -> QLineF;
        #[doc(hidden)]
        #[rust_name = "qlinef_from_qline"]
        fn construct(line: &QLine) -> QLineF;
    }
}

/// The QLineF class provides a two-dimensional vector using floating point precision.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct QLineF {
    pt1: QPointF,
    pt2: QPointF,
}

impl QLineF {
    /// Constructs a line object that represents the line between pt1 and pt2.
    pub fn new(pt1: QPointF, pt2: QPointF) -> Self {
        ffi::qlinef_new(pt1, pt2)
    }
}

impl Default for QLineF {
    /// Constructs a default qlinef
    fn default() -> Self {
        ffi::qlinef_default()
    }
}

impl From<&ffi::QLine> for QLineF {
    /// Construct a QLineF object from the given integer-based line.
    fn from(line: &QLine) -> Self {
        ffi::qlinef_from_qline(line)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QLineF is trivial.
unsafe impl ExternType for QLineF {
    type Id = type_id!("QLineF");
    type Kind = cxx::kind::Trivial;
}
