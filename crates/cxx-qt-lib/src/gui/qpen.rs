// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpen.h");
        type QPen = super::QPen;
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;

        /// Returns true if the pen is cosmetic; otherwise returns false.
        #[rust_name = "is_comestic"]
        fn isCosmetic(pen: &QPen) -> bool;

        /// Returns true if the pen has a solid fill, otherwise false.
        #[rust_name = "is_solid"]
        fn isSolid(pen: &QPen) -> bool;

        /// Returns the color of this pen's brush.
        fn color(pen: &QPen) -> QColor;

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

#[repr(C)]
pub struct QPen {
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
