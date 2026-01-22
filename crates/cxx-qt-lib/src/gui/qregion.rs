// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    /// Specifies the shape of the region to be created.
    #[namespace = "rust::cxxqtlib1"]
    #[repr(i32)]
    enum QRegionRegionType {
        /// the region covers the entire rectangle.
        Rectangle,
        /// The region is an ellipse inside the rectangle.
        Ellipse,
    }

    #[namespace = "Qt"]
    extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type FillRule = crate::FillRule;
    }

    #[namespace = "rust::cxxqtlib1"]
    extern "C++" {
        include!("cxx-qt-lib/qregion.h");
        type QRegionRegionType;
    }

    unsafe extern "C++" {
        type QRegion = super::QRegion;

        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;

        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;

        include!("cxx-qt-lib/qpolygon.h");
        type QPolygon = crate::QPolygon;

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
        #[rust_name = "qregion_init_qrect_regiontype"]
        fn construct(r: &QRect, t: QRegionRegionType) -> QRegion;
        #[doc(hidden)]
        #[rust_name = "qregion_init_qpolygon_fillrule"]
        fn construct(a: &QPolygon, fill_rule: FillRule) -> QRegion;

        #[doc(hidden)]
        #[rust_name = "qregion_drop"]
        fn drop(r: &mut QRegion);

        #[doc(hidden)]
        #[rust_name = "qregion_clone"]
        fn construct(r: &QRegion) -> QRegion;
    }
}

pub use ffi::QRegionRegionType;

use crate::{FillRule, QPolygon, QRect};

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

impl QRegion {
    /// Create a region based on the rectangle `r` with region type `t`.
    ///
    /// If the rectangle is invalid a null region will be created.
    pub fn from_rect(r: &QRect, t: QRegionRegionType) -> Self {
        ffi::qregion_init_qrect_regiontype(r, t)
    }

    /// Constructs a polygon region from the point array `a` with the fill rule specified by `fill_rule`.
    ///
    /// If `fill_rule` is [`FillRule::WindingFill`], the polygon region is defined using the winding algorithm; if it is [`FillRule::OddEvenFill`], the odd-even fill algorithm is used.
    ///
    /// **Warning:** This constructor can be used to create complex regions that will slow down painting when used.
    pub fn from_polygon(a: &QPolygon, fill_rule: FillRule) -> Self {
        ffi::qregion_init_qpolygon_fillrule(a, fill_rule)
    }
}

impl From<&QRect> for QRegion {
    /// Create a region based on the rectangle `r` with region type [`QRegionRegionType::Rectangle`].
    ///
    /// If the rectangle is invalid a null region will be created.
    fn from(r: &QRect) -> Self {
        ffi::qregion_init_qrect_regiontype(r, QRegionRegionType::Rectangle)
    }
}
impl From<QRect> for QRegion {
    /// Create a region based on the rectangle `r` with region type [`QRegionRegionType::Rectangle`].
    ///
    /// If the rectangle is invalid a null region will be created.
    fn from(r: QRect) -> Self {
        Self::from(&r)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QRegion {
    type Id = type_id!("QRegion");
    type Kind = cxx::kind::Trivial;
}
