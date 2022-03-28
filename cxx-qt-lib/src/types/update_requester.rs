// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
mod ffi {
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type UpdateRequester;

        #[rust_name = "request_update"]
        fn requestUpdate(self: &UpdateRequester) -> bool;
    }

    impl UniquePtr<UpdateRequester> {}
}

pub type UpdateRequesterCpp = ffi::UpdateRequester;

// # Safety
//
// The underlying C++ class has been designed to be thread safe
// as it uses invokeMethod, so can be sent to other threads.
unsafe impl Send for UpdateRequesterCpp {}

pub struct UpdateRequester {
    inner: cxx::UniquePtr<UpdateRequesterCpp>,
}

impl UpdateRequester {
    pub fn from_unique_ptr(ptr: cxx::UniquePtr<UpdateRequesterCpp>) -> Self {
        Self { inner: ptr }
    }

    pub fn request_update(&self) {
        if let Some(inner) = self.inner.as_ref() {
            inner.request_update();
        }
    }
}
