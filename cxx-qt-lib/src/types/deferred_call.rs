// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::os::raw::{c_char, c_void};

#[cxx::bridge]
mod ffi {
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type DeferredCall = super::DeferredCall;

        #[rust_name = "update"]
        fn update(self: &DeferredCall) -> bool;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeferredCall {
    method: *mut c_char,
    obj: *mut c_void,
}

unsafe impl ExternType for ffi::DeferredCall {
    type Id = type_id!("rust::cxxqtlib1::DeferredCall");
    type Kind = cxx::kind::Trivial;
}

// # Safety
//
// The underlying C++ class has been designed to be thread safe and we only
// store a pointer to it which is valid from any thread.
unsafe impl Send for DeferredCall {}
unsafe impl Sync for DeferredCall {}
