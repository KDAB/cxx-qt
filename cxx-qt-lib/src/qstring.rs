// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// We are only using references to String so it is actually ffi safe as far as we are concerned
#![allow(improper_ctypes)]

use cxx::{memory::UniquePtrTarget, type_id, ExternType};
use std::{
    ffi::c_void,
    marker::{PhantomData, PhantomPinned},
    mem::MaybeUninit,
};

// This file is largely based on https://github.com/dtolnay/cxx/blob/master/src/cxx_string.rs
// and has merely been adapted for Qt's QString.

extern "C" {
    #[link_name = "cxxqt1$qstring$init$from$rust$string"]
    fn qstring_init_from_rust_string(
        ptr: &mut MaybeUninit<cxx::UniquePtr<QString>>,
        ptr: *const u8,
        len: usize,
    );
    #[link_name = "cxxqt1$qstring$to$rust$string"]
    fn qstring_to_rust_string(qt: &QString, rust: &mut String);
}

/// Binding to Qt `QString`.
///
/// # Invariants
///
/// As an invariant of this API and the static analysis of the cxx::bridge
/// macro, in Rust code we can never obtain a `QString` by value. Qt's QString
/// requires a move constructor and may hold internal pointers, which is not
/// compatible with Rust's move behavior. Instead in Rust code we will only ever
/// look at a QString through a reference or smart pointer, as in `&QString`
/// or `UniquePtr<QString>`.
#[repr(C)]
pub struct QString {
    _pinned: PhantomData<PhantomPinned>,
}

impl QString {
    /// Create a new Rust string from this QString. This operation
    /// needs to convert the UTF-16 data in the QString to UTF-8
    /// data and thus likely needs to an allocate. This is essentially
    /// a copy and so any changes will not propagate to the QString.
    pub fn to_rust(&self) -> String {
        let mut s = String::new();
        // Safety:
        //
        // Given that a QString can only be constructed using [`let_qstring!`] macro,
        // it is safe to assume that self is a valid QString reference which makes this
        // function call safe.
        unsafe { qstring_to_rust_string(self, &mut s) };
        s
    }
}

// Safety:
//
// The code in this file ensures that QString can only ever be allocated
// on the stack in pinned form which avoids the pitfalls of trying to
// move this type that has a non-trivial move constructor.
unsafe impl ExternType for QString {
    type Id = type_id!("QString");
    type Kind = cxx::kind::Opaque;
}

extern "C" {
    #[link_name = "cxxqt1$unique_ptr$qstring$null"]
    fn unique_ptr_qstring_null(this: *mut MaybeUninit<*mut c_void>);
    #[link_name = "cxxqt1$unique_ptr$qstring$raw"]
    fn unique_ptr_qstring_raw(this: *mut MaybeUninit<*mut c_void>, raw: *mut QString);
    #[link_name = "cxxqt1$unique_ptr$qstring$get"]
    fn unique_ptr_qstring_get(this: *const MaybeUninit<*mut c_void>) -> *const QString;
    #[link_name = "cxxqt1$unique_ptr$qstring$release"]
    fn unique_ptr_qstring_release(this: *mut MaybeUninit<*mut c_void>) -> *mut QString;
    #[link_name = "cxxqt1$unique_ptr$qstring$drop"]
    fn unique_ptr_qstring_drop(this: *mut MaybeUninit<*mut c_void>);
}

unsafe impl UniquePtrTarget for QString {
    #[doc(hidden)]
    fn __typename(f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("QString")
    }

    #[doc(hidden)]
    fn __null() -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unsafe {
            unique_ptr_qstring_null(&mut repr);
        }
        repr
    }

    #[doc(hidden)]
    unsafe fn __raw(raw: *mut Self) -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unique_ptr_qstring_raw(&mut repr, raw);
        repr
    }

    #[doc(hidden)]
    unsafe fn __get(repr: MaybeUninit<*mut c_void>) -> *const Self {
        unique_ptr_qstring_get(&repr)
    }

    #[doc(hidden)]
    unsafe fn __release(mut repr: MaybeUninit<*mut c_void>) -> *mut Self {
        unique_ptr_qstring_release(&mut repr)
    }

    #[doc(hidden)]
    unsafe fn __drop(mut repr: MaybeUninit<*mut c_void>) {
        unique_ptr_qstring_drop(&mut repr)
    }
}

impl From<&QString> for String {
    fn from(qstring: &QString) -> Self {
        qstring.to_rust()
    }
}

impl crate::ToUniquePtr for &String {
    type CppType = QString;

    fn to_unique_ptr(self) -> cxx::UniquePtr<QString> {
        unsafe {
            let mut ptr = MaybeUninit::<cxx::UniquePtr<QString>>::zeroed();
            qstring_init_from_rust_string(&mut ptr, self.as_ptr(), self.len());
            ptr.assume_init()
        }
    }
}

impl crate::ToUniquePtr for &str {
    type CppType = QString;

    /// Retrieve the UniquePtr to the Qt QString of this Rust String
    /// so that this object can be passed back to C++.
    fn to_unique_ptr(self) -> cxx::UniquePtr<QString> {
        unsafe {
            let mut ptr = MaybeUninit::<cxx::UniquePtr<QString>>::zeroed();
            qstring_init_from_rust_string(&mut ptr, self.as_ptr(), self.len());
            ptr.assume_init()
        }
    }
}
