// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::QLine;
use crate::QPointF;
use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlinef.h");
        type QLineF = super::QLineF;
        include!("cxx-qt-lib/qline.h");
        type QLine = crate::QLine;
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = crate::QPointF;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Returns the angle of the line in degrees.
        fn angle(self: &QLineF) -> f64;

        /// Returns the angle (in degrees) from this line to the given line, taking the direction of the lines into account.
        /// If the lines do not intersect within their range, it is the intersection point of the extended lines that serves as origin (see QLineF::UnboundedIntersection).
        #[rust_name = "angle_to"]
        fn angleTo(self: &QLineF, line: &QLineF) -> f64;

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

        /// Returns the length of the line.
        fn length(self: &QLineF) -> f64;

        /// Returns a line that is perpendicular to this line with the same starting point and length.
        #[rust_name = "normal_vector"]
        fn normalVector(self: &QLineF) -> QLineF;

        /// Returns the point at the parameterized position specified by t. The function returns the line's start point if t = 0, and its end point if t = 1.
        #[rust_name = "point_at"]
        fn pointAt(self: &QLineF, t: f64) -> QPointF;

        /// Sets the angle of the line to the given angle (in degrees). This will change the position of the second point of the line such that the line has the given angle.
        #[rust_name = "set_angle"]
        fn setAngle(self: &mut QLineF, angle: f64);

        /// Sets the length of the line to the given length. QLineF will move the end point - p2() - of the line to give the line its new length, unless length() was previously zero, i
        /// in which case no scaling is attempted. For lines with very short lengths (represented by denormal floating-point values), results may be imprecise.
        #[rust_name = "set_length"]
        fn setLength(self: &mut QLineF, length: f64);

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

        /// Returns an integer based copy of this line.
        #[rust_name = "to_line"]
        fn toLine(self: &QLineF) -> QLine;

        /// Translates this line by the given offset.
        fn translate(self: &mut QLineF, offset: &QPointF);

        /// Returns this line translated by the given offset.
        fn translated(self: &QLineF, offset: &QPointF) -> QLineF;

        /// Returns the unit vector for this line, i.e a line starting at the same point as this line with a length of 1.0, provided the line is non-null.
        #[rust_name = "unit_vector"]
        fn unitVector(self: &QLineF) -> QLineF;
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

        #[doc(hidden)]
        #[rust_name = "qlinef_to_qstring"]
        fn toQString(value: &QLineF) -> QString;
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

impl From<QLineF> for ffi::QLine {
    /// Returns an integer-based copy of this line.
    /// Note that the returned line's start and end points are rounded to the nearest integer.
    fn from(value: QLineF) -> Self {
        value.to_line()
    }
}

impl fmt::Display for QLineF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qlinef_to_qstring(self))
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QLineF is trivial.
unsafe impl ExternType for QLineF {
    type Id = type_id!("QLineF");
    type Kind = cxx::kind::Trivial;
}
