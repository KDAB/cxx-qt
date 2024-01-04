// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type PenStyle = crate::PenStyle;
        type PenCapStyle = crate::PenCapStyle;
        type PenJoinStyle = crate::PenJoinStyle;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qpen.h");
        type QPen = super::QPen;
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;

        /// Returns the pen's cap style.
        #[rust_name = "cap_style"]
        fn capStyle(self: &QPen) -> PenCapStyle;

        /// Returns the color of this pen's brush.
        fn color(self: &QPen) -> QColor;

        /// Returns true if the pen is cosmetic; otherwise returns false.
        #[rust_name = "is_comestic"]
        fn isCosmetic(self: &QPen) -> bool;

        /// Returns true if the pen has a solid fill, otherwise false.
        #[rust_name = "is_solid"]
        fn isSolid(self: &QPen) -> bool;

        /// Returns the pen's join style.
        #[rust_name = "join_style"]
        fn joinStyle(self: &QPen) -> PenJoinStyle;

        /// Sets the pen's cap style to the given style. The default value is Qt::SquareCap.
        #[rust_name = "set_cap_style"]
        fn setCapStyle(self: &mut QPen, style: PenCapStyle);

        /// Sets the pen's join style to the given style. The default value is Qt::BevelJoin.
        #[rust_name = "set_join_style"]
        fn setJoinStyle(self: &mut QPen, style: PenJoinStyle);

        /// Sets the pen style to the given style.
        #[rust_name = "set_style"]
        fn setStyle(self: &mut QPen, style: PenStyle);

        /// Sets the pen width to the given width in pixels with integer precision.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QPen, width: i32);

        /// Returns the pen style.
        fn style(self: &QPen) -> PenStyle;

        /// Returns the pen width with integer precision.
        fn width(self: &QPen) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qpen_init_default"]
        fn construct() -> QPen;

        #[doc(hidden)]
        #[rust_name = "qpen_drop"]
        fn drop(pen: &mut QPen);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct QPen {
    #[cfg(qt_version_major = "5")]
    _cspec: MaybeUninit<[i32; 2]>,
    #[cfg(qt_version_major = "6")]
    _cspec: MaybeUninit<i32>,
}

impl Default for QPen {
    /// Constructs a default black solid line pen with 1 width.
    fn default() -> Self {
        ffi::qpen_init_default()
    }
}

impl Drop for QPen {
    fn drop(&mut self) {
        ffi::qpen_drop(self);
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QPen {
    type Id = type_id!("QPen");
    type Kind = cxx::kind::Trivial;
}
