// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
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
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = crate::QPointF;
        include!("cxx-qt-lib/qrectf.h");
        type QRectF = crate::QRectF;
        include!("cxx-qt-lib/qpolygon.h");
        type QPolygon = crate::QPolygon;

        include!("cxx-qt-lib/qpolygonf.h");
        type QPolygonF = super::QPolygonF;

        /// Returns the bounding rectangle of the polygon, or QRectF(0, 0, 0, 0) if the polygon is empty.
        #[rust_name = "bounding_rect"]
        fn boundingRect(self: &QPolygonF) -> QRectF;

        /// Returns true if the given point is inside the polygon according to the specified fillRule; otherwise returns false.
        #[rust_name = "contains_point"]
        fn containsPoint(self: &QPolygonF, point: &QPointF, fillRule: FillRule) -> bool;

        /// Returns a polygon which is the intersection of this polygon and r.
        fn intersected(self: &QPolygonF, r: &QPolygonF) -> QPolygonF;

        /// Returns true if the current polygon intersects at any point the given polygon p.
        /// Also returns true if the current polygon contains or is contained by any part of p.
        fn intersects(self: &QPolygonF, p: &QPolygonF) -> bool;

        /// Returns true if the polygon is closed; otherwise returns false.
        #[rust_name = "is_closed"]
        fn isClosed(self: &QPolygonF) -> bool;

        /// Returns a polygon which is r subtracted from this polygon.
        fn subtracted(self: &QPolygonF, r: &QPolygonF) -> QPolygonF;

        /// Creates and returns a QPolygon by converting each QPointF to a QPoint.
        #[rust_name = "to_polygon"]
        fn toPolygon(self: &QPolygonF) -> QPolygon;

        /// Translates all points in the polygon by (dx, dy).
        fn translate(self: &mut QPolygonF, dx: f64, dy: f64);

        /// Returns a copy of the polygon that is translated by (dx, dy).
        fn translated(self: &QPolygonF, dx: f64, dy: f64) -> QPolygonF;

        /// Returns a polygon which is the union of this polygon and r.
        fn united(self: &QPolygonF, r: &QPolygonF) -> QPolygonF;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qpolygonf_init_default"]
        fn construct() -> QPolygonF;

        #[doc(hidden)]
        #[rust_name = "qpolygonf_drop"]
        fn drop(pen: &mut QPolygonF);

        #[doc(hidden)]
        #[rust_name = "qpolygonf_clone"]
        fn construct(p: &QPolygonF) -> QPolygonF;
    }
}

/// The QPolygonF class provides a list of QPointF.
#[repr(C)]
pub struct QPolygonF {
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 QPolygon has one pointer as a member
    /// Qt6 QPolygon has one member, which contains two pointers and a size_t
    #[cfg(qt_version_major = "5")]
    _space: MaybeUninit<usize>,
    #[cfg(qt_version_major = "6")]
    _space: MaybeUninit<[usize; 3]>,
}

impl Default for QPolygonF {
    /// Constructs a copy of the given polygon.
    fn default() -> Self {
        ffi::qpolygonf_init_default()
    }
}

impl Drop for QPolygonF {
    fn drop(&mut self) {
        ffi::qpolygonf_drop(self);
    }
}

impl Clone for QPolygonF {
    fn clone(&self) -> Self {
        ffi::qpolygonf_clone(self)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QPolygonF {
    type Id = type_id!("QPolygonF");
    type Kind = cxx::kind::Trivial;
}
