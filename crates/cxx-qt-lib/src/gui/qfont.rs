// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qfont.h");
        type QFont = super::QFont;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qfont_init_default"]
        fn construct() -> QFont;

        #[doc(hidden)]
        #[rust_name = "qfont_drop"]
        fn drop(pen: &mut QFont);        
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct QFont {
    _cspec: MaybeUninit<i32>,
    _resolve_mask: MaybeUninit<u16>,
}

impl Default for QFont {
    fn default() -> Self {
        ffi::qfont_init_default()
    }
}

impl Drop for QFont {
    fn drop(&mut self) {
        ffi::qfont_drop(self);
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QFont {
    type Id = type_id!("QFont");
    type Kind = cxx::kind::Trivial;
}
