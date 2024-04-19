// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::QPoint;
use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qline.h");
        type QLine = super::QLine;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qlinef.h");
        #[allow(dead_code)]
        type QLineF = crate::QLineF;

        /// Returns the line's start point.
        fn p1(self: &QLine) -> QPoint;

        /// Returns the line's end point.
        fn p2(self: &QLine) -> QPoint;

        /// Returns the x-coordinate of the line's start point.
        fn x1(self: &QLine) -> i32;

        /// Returns the x-coordinate of the line's end point.
        fn x2(self: &QLine) -> i32;

        /// Returns the y-coordinate of the line's start point.
        fn y1(self: &QLine) -> i32;

        /// Returns the y-coordinate of the line's end point.
        fn y2(self: &QLine) -> i32;

        /// Returns the center point of this line. This is equivalent to (p1() + p2()) / 2, except it will never overflow.
        fn center(self: &QLine) -> QPoint;

        /// Returns the horizontal component of the line's vector.
        fn dx(self: &QLine) -> i32;

        /// Returns the vertical component of the line's vector.
        fn dy(self: &QLine) -> i32;

        /// Returns true if the line does not have distinct start and end points; otherwise returns false.
        #[rust_name = "is_null"]
        fn isNull(self: &QLine) -> bool;

        /// Sets the starting point of this line to p1.
        #[rust_name = "set_p1"]
        fn setP1(self: &mut QLine, p1: &QPoint);

        /// Sets the end point of this line to p2.
        #[rust_name = "set_p2"]
        fn setP2(self: &mut QLine, p1: &QPoint);

        /// Sets this line to the start in x1, y1 and end in x2, y2.
        #[rust_name = "set_line"]
        fn setLine(self: &mut QLine, x1: i32, y1: i32, x2: i32, y2: i32);

        /// Sets the start point of this line to p1 and the end point of this line to p2.
        #[rust_name = "set_points"]
        fn setPoints(self: &mut QLine, p1: &QPoint, p2: &QPoint);

        /// Returns this line as a line with floating point accuracy.
        /// since Qt 6.4.
        #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_4))]
        #[rust_name = "to_linef"]
        fn toLineF(self: &QLine) -> QLineF;

        /// Translates this line by the given offset.
        fn translate(self: &mut QLine, offset: &QPoint);

        /// Returns this line translated by the given offset.
        fn translated(self: &QLine, offset: &QPoint) -> QLine;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qline_default"]
        fn construct() -> QLine;

        #[doc(hidden)]
        #[rust_name = "qline_new"]
        fn construct(pt1: QPoint, pt2: QPoint) -> QLine;

        #[doc(hidden)]
        #[rust_name = "qline_to_qstring"]
        fn toQString(value: &QLine) -> QString;
    }
}

/// The QLine class provides a two-dimensional vector using integer precision
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct QLine {
    pt1: QPoint,
    pt2: QPoint,
}

impl QLine {
    /// Constructs a line object that represents the line between p1 and p2.
    pub fn new(pt1: QPoint, pt2: QPoint) -> Self {
        ffi::qline_new(pt1, pt2)
    }
}

impl Default for QLine {
    /// Constructs a null line.
    fn default() -> Self {
        ffi::qline_default()
    }
}

impl fmt::Display for QLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qline_to_qstring(self))
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QLine is trivial.
unsafe impl ExternType for QLine {
    type Id = type_id!("QLine");
    type Kind = cxx::kind::Trivial;
}
