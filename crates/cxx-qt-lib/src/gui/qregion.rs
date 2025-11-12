// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qregion.h");
        type QRegion = super::QRegion;

        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;

        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;

        /// Returns the bounding rectangle of this region. An empty region gives a rectangle that is [`QRect::is_null`].
        #[rust_name = "bounding_rect"]
        fn boundingRect(self: &QRegion) -> QRect;

        /// Returns `true` if the region overlaps the rectangle `r`; otherwise returns `false`.
        fn contains(self: &QRegion, r: &QRect) -> bool;

        /// Returns a region which is the intersection of this region and `r`.
        fn intersected(self: &QRegion, r: &QRegion) -> QRegion;

        /// Returns `true` if the region is empty; otherwise returns `false`. An empty region is a region that contains no points.
        #[rust_name = "is_empty"]
        fn isEmpty(self: &QRegion) -> bool;

        /// Returns `true` if the region is empty; otherwise returns `false`. An empty region is a region that contains no points.
        /// This function is the same as [`is_empty`](Self::is_empty).
        #[rust_name = "is_null"]
        fn isNull(self: &QRegion) -> bool;

        /// Returns the number of rectangles that this region is composed of.
        #[rust_name = "rect_count"]
        fn rectCount(self: &QRegion) -> i32;

        /// Returns a region which is `r` subtracted from this region.
        fn subtracted(self: &QRegion, r: &QRegion) -> QRegion;

        /// Translates the region `point.x()` along the x axis and `point.y()` along the y axis, relative to the current position.
        /// Positive values move the region to the right and down.
        ///
        /// Translates to the given `point`.
        fn translate(self: &mut QRegion, point: &QPoint);

        /// Returns a copy of the region that is translated `p.x()` along the x axis and `p.y()` along the y axis,
        /// relative to the current position. Positive values move the rectangle to the right and down.
        fn translated(self: &QRegion, p: &QPoint) -> QRegion;

        /// Returns a region which is the union of this region and `r`.
        fn united(self: &QRegion, r: &QRegion) -> QRegion;

        /// Returns a region which is the exclusive or (XOR) of this region and `r`.
        fn xored(self: &QRegion, r: &QRegion) -> QRegion;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qregion_init_default"]
        fn construct() -> QRegion;

        #[doc(hidden)]
        #[rust_name = "qregion_drop"]
        fn drop(r: &mut QRegion);

        #[doc(hidden)]
        #[rust_name = "qregion_clone"]
        fn construct(r: &QRegion) -> QRegion;
    }
}

/// The `QRegion` class specifies a clip region for a painter.
///
/// Qt Documentation: [QRegion](https://doc.qt.io/qt/qregion.html#details)
#[repr(C)]
pub struct QRegion {
    _cspec: MaybeUninit<usize>,
}

impl Default for QRegion {
    /// Constructs an empty region.
    fn default() -> Self {
        ffi::qregion_init_default()
    }
}

impl Drop for QRegion {
    fn drop(&mut self) {
        ffi::qregion_drop(self);
    }
}

impl Clone for QRegion {
    fn clone(&self) -> Self {
        ffi::qregion_clone(self)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QRegion {
    type Id = type_id!("QRegion");
    type Kind = cxx::kind::Trivial;
}
