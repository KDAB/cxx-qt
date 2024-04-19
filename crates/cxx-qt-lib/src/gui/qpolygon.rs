// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::QRect;
use core::mem::MaybeUninit;
use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type FillRule = crate::FillRule;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qpolygonf.h");
        #[allow(dead_code)]
        type QPolygonF = crate::QPolygonF;

        include!("cxx-qt-lib/qpolygon.h");
        type QPolygon = super::QPolygon;

        /// Returns the bounding rectangle of the polygon, or QRect(0, 0, 0, 0) if the polygon is empty.
        #[rust_name = "bounding_rect"]
        fn boundingRect(self: &QPolygon) -> QRect;

        /// Returns true if the given point is inside the polygon according to the specified fillRule; otherwise returns false.
        #[rust_name = "contains_point"]
        fn containsPoint(self: &QPolygon, point: &QPoint, fillRule: FillRule) -> bool;

        /// Returns a polygon which is the intersection of this polygon and r.
        fn intersected(self: &QPolygon, r: &QPolygon) -> QPolygon;

        /// Returns true if the current polygon intersects at any point the given polygon p.
        /// Also returns true if the current polygon contains or is contained by any part of p.
        fn intersects(self: &QPolygon, p: &QPolygon) -> bool;

        /// Returns the point at the given index.
        fn point(self: &QPolygon, index: i32) -> QPoint;

        /// Sets the point at the given index to the given point.
        #[rust_name = "set_point"]
        fn setPoint(self: &mut QPolygon, index: i32, point: &QPoint);

        /// Returns a polygon which is r subtracted from this polygon.
        fn subtracted(self: &QPolygon, r: &QPolygon) -> QPolygon;

        /// Returns this polygon as a polygon with floating point accuracy.
        /// since Qt 6.4.
        #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_4))]
        #[rust_name = "to_polygonf"]
        fn toPolygonF(self: &QPolygon) -> QPolygonF;

        /// Translates all points in the polygon by (dx, dy).
        fn translate(self: &mut QPolygon, dx: i32, dy: i32);

        /// Returns a copy of the polygon that is translated by (dx, dy).
        fn translated(self: &QPolygon, dx: i32, dy: i32) -> QPolygon;

        /// Returns a polygon which is the union of this polygon and r.
        fn united(self: &QPolygon, r: &QPolygon) -> QPolygon;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qpolygon_init_default"]
        fn construct() -> QPolygon;

        #[doc(hidden)]
        #[rust_name = "qpolygon_init_qrect"]
        fn construct(rect: &QRect, closed: bool) -> QPolygon;

        #[doc(hidden)]
        #[rust_name = "qpolygon_drop"]
        fn drop(pen: &mut QPolygon);

        #[doc(hidden)]
        #[rust_name = "qpolygon_clone"]
        fn construct(p: &QPolygon) -> QPolygon;

        #[doc(hidden)]
        #[rust_name = "qpolygon_eq"]
        fn operatorEq(a: &QPolygon, b: &QPolygon) -> bool;

        #[doc(hidden)]
        #[rust_name = "qpolygon_to_qstring"]
        fn toQString(value: &QPolygon) -> QString;
    }
}

/// The QPolygon class provides a list of QPoint.
#[repr(C)]
pub struct QPolygon {
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 QPolygon has one pointer as a member
    /// Qt6 QPolygon has one member, which contains two pointers and a size_t
    #[cfg(cxxqt_qt_version_major = "5")]
    _space: MaybeUninit<usize>,
    #[cfg(cxxqt_qt_version_major = "6")]
    _space: MaybeUninit<[usize; 3]>,
}

impl Default for QPolygon {
    /// Constructs a copy of the given polygon.
    fn default() -> Self {
        ffi::qpolygon_init_default()
    }
}

impl Drop for QPolygon {
    fn drop(&mut self) {
        ffi::qpolygon_drop(self);
    }
}

impl Clone for QPolygon {
    fn clone(&self) -> Self {
        ffi::qpolygon_clone(self)
    }
}

impl QPolygon {
    /// Constructs a polygon from the given rectangle. If closed is false, the polygon
    /// just contains the four points of the rectangle ordered clockwise, otherwise the
    /// polygon's fifth point is set to rectangle.topLeft().
    pub fn new(rect: &QRect, closed: bool) -> Self {
        ffi::qpolygon_init_qrect(rect, closed)
    }
}

impl PartialEq for QPolygon {
    fn eq(&self, other: &Self) -> bool {
        ffi::qpolygon_eq(self, other)
    }
}

impl std::fmt::Display for QPolygon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", ffi::qpolygon_to_qstring(self))
    }
}

impl Eq for QPolygon {}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QPolygon {
    type Id = type_id!("QPolygon");
    type Kind = cxx::kind::Trivial;
}
