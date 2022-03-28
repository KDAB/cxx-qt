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

        type UpdateRequester = super::UpdateRequester;

        #[rust_name = "request_update"]
        fn requestUpdate(self: &UpdateRequester) -> bool;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UpdateRequester {
    method: *mut c_char,
    obj: *mut c_void,
}

unsafe impl ExternType for ffi::UpdateRequester {
    type Id = type_id!("rust::cxxqtlib1::UpdateRequester");
    type Kind = cxx::kind::Trivial;
}

// # Safety
//
// The underlying C++ class has been designed to be thread safe and we only
// store a pointer to it which is valid from any thread.
unsafe impl Send for UpdateRequester {}
unsafe impl Sync for UpdateRequester {}
