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

        /// Adds the given path to this path as a closed subpath.
        #[rust_name = "add_path"]
        fn addPath(self: &mut QPainterPath, path: &QPainterPath);

        /// Adds the given rectangle to this path as a closed subpath.
        #[rust_name = "add_rect"]
        fn addRect(self: &mut QPainterPath, rectangle: &QRectF);

        /// Returns the bounding rectangle of this painter path as a rectangle with floating point precision.
        #[rust_name = "bounding_rect"]
        fn boundingRect(self: &QPainterPath) -> QRectF;

        /// Clears the path elements stored.
        fn clear(self: &mut QPainterPath);

        /// Returns the current position of the path.
        #[rust_name = "current_position"]
        fn currentPosition(self: &QPainterPath) -> QPointF;

        /// Returns true if either there are no elements in this path, 
        /// or if the only element is a MoveToElement; otherwise returns false.
        #[rust_name = "is_empty"]
        fn isEmpty(self: &QPainterPath) -> bool;

        /// Returns a simplified version of this path.
        fn simplified(self: &QPainterPath) -> QPainterPath;

        /// Translates all elements in the path by (dx, dy).
        fn translate(self: &mut QPainterPath, dx: f64, dy: f64);
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

#[derive(Clone)]
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
