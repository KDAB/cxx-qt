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
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qpen_init_default"]
        fn construct() -> QPen;
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct QPen {
    _cspec: MaybeUninit<i32>,
    _ct: MaybeUninit<[u16; 5]>,
    _padding: MaybeUninit<u16>,
}

impl Default for QPen {
    /// Constructs a default black solid line pen with 1 width.
    fn default() -> Self {
        ffi::qpen_init_default()
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QPen {
    type Id = type_id!("QPen");
    type Kind = cxx::kind::Trivial;
}
