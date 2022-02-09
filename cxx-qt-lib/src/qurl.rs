// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// We are only using references to QUrl so it is actually ffi safe as far as we are concerned
#![allow(improper_ctypes)]

use cxx::{memory::UniquePtrTarget, type_id, ExternType};
use std::{
    ffi::c_void,
    marker::{PhantomData, PhantomPinned},
    mem::MaybeUninit,
};

extern "C" {
    #[link_name = "cxxqt1$qurl$init$from$qurl"]
    fn qurl_init_from_qurl(this: &mut MaybeUninit<cxx::UniquePtr<QUrl>>, qurl: &QUrl);
    #[link_name = "cxxqt1$qurl$init$from$string"]
    fn qurl_init_from_string(
        this: &mut MaybeUninit<cxx::UniquePtr<QUrl>>,
        ptr: *const u8,
        len: usize,
    );
    #[link_name = "cxxqt1$qurl$to$rust$string"]
    fn qurl_to_rust_string(qt: &QUrl, rust: &mut String);
}

/// Binding to Qt `QUrl`.
///
/// # Invariants
///
/// As an invariant of this API and the static analysis of the cxx::bridge
/// macro, in Rust code we can never obtain a `QUrl` by value. Qt's QUrl
/// requires a move constructor and may hold internal pointers, which is not
/// compatible with Rust's move behavior. Instead in Rust code we will only ever
/// look at a QUrl through a reference or smart pointer, as in `&QUrl`
/// or `UniquePtr<QUrl>`.
#[repr(C)]
pub struct QUrl {
    _pinned: PhantomData<PhantomPinned>,
}

impl QUrl {
    /// Create a new Rust Url from this QUrl.
    /// This is a copy operation so any changes will not propagate to the original QUrl.
    pub fn to_rust(&self) -> Url {
        Url::from_qurl(self)
    }
}

// Safety:
//
// The code in this file ensures that QUrl can only ever be allocated
// on the stack in pinned form which avoids the pitfalls of trying to
// move this type that has a non-trivial move constructor.
unsafe impl ExternType for QUrl {
    type Id = type_id!("QUrl");
    type Kind = cxx::kind::Opaque;
}

extern "C" {
    #[link_name = "cxxqt1$unique_ptr$qurl$null"]
    fn unique_ptr_qurl_null(this: *mut MaybeUninit<*mut c_void>);
    #[link_name = "cxxqt1$unique_ptr$qurl$raw"]
    fn unique_ptr_qurl_raw(this: *mut MaybeUninit<*mut c_void>, raw: *mut QUrl);
    #[link_name = "cxxqt1$unique_ptr$qurl$get"]
    fn unique_ptr_qurl_get(this: *const MaybeUninit<*mut c_void>) -> *const QUrl;
    #[link_name = "cxxqt1$unique_ptr$qurl$release"]
    fn unique_ptr_qurl_release(this: *mut MaybeUninit<*mut c_void>) -> *mut QUrl;
    #[link_name = "cxxqt1$unique_ptr$qurl$drop"]
    fn unique_ptr_qurl_drop(this: *mut MaybeUninit<*mut c_void>);
}

// Safety: TODO
unsafe impl UniquePtrTarget for QUrl {
    #[doc(hidden)]
    fn __typename(f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("QUrl")
    }

    #[doc(hidden)]
    fn __null() -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unsafe {
            unique_ptr_qurl_null(&mut repr);
        }
        repr
    }

    #[doc(hidden)]
    unsafe fn __raw(raw: *mut Self) -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unique_ptr_qurl_raw(&mut repr, raw);
        repr
    }

    #[doc(hidden)]
    unsafe fn __get(repr: MaybeUninit<*mut c_void>) -> *const Self {
        unique_ptr_qurl_get(&repr)
    }

    #[doc(hidden)]
    unsafe fn __release(mut repr: MaybeUninit<*mut c_void>) -> *mut Self {
        unique_ptr_qurl_release(&mut repr)
    }

    #[doc(hidden)]
    unsafe fn __drop(mut repr: MaybeUninit<*mut c_void>) {
        unique_ptr_qurl_drop(&mut repr)
    }
}

pub struct Url {
    // Note that once map_qt_value is removed later, this can become private again
    #[doc(hidden)]
    pub(crate) inner: cxx::UniquePtr<QUrl>,
}

impl Url {
    /// Construct a Rust Url from an existing UniquePtr<QUrl> this is a move operation
    ///
    /// This is used in QVariant::value so that we don't need to perform another copy
    pub(crate) fn from_unique_ptr(ptr: cxx::UniquePtr<QUrl>) -> Self {
        Self { inner: ptr }
    }

    pub fn from_qurl(qurl: &QUrl) -> Self {
        Self {
            // Safety: TODO
            inner: unsafe {
                let mut ptr = MaybeUninit::<cxx::UniquePtr<QUrl>>::zeroed();
                qurl_init_from_qurl(&mut ptr, qurl);
                ptr.assume_init()
            },
        }
    }

    // TODO: other QUrl methods
    //
    // fragment: Option<String>,
    // host: Option<String>,
    // password: Option<String>,
    // path: Option<String>,
    // port: Option<u16>,
    // query: Option<String>,
    // scheme: Option<String>,
    // userName: Option<String>,

    pub fn string(&self) -> String {
        let mut s = String::new();
        unsafe { qurl_to_rust_string(&self.inner, &mut s) };
        s
    }
}

impl std::str::FromStr for Url {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, std::convert::Infallible> {
        Ok(Self {
            // Safety: TODO
            inner: unsafe {
                let mut ptr = MaybeUninit::<cxx::UniquePtr<QUrl>>::zeroed();
                qurl_init_from_string(&mut ptr, s.as_ptr(), s.len());
                ptr.assume_init()
            },
        })
    }
}

impl crate::ToUniquePtr for Url {
    type CppType = QUrl;

    /// Retrieve the UniquePtr to the Qt QUrl of this Rust Url
    /// so that this object can be passed back to C++.
    fn to_unique_ptr(self) -> cxx::UniquePtr<QUrl> {
        self.inner
    }
}

impl From<&QUrl> for Url {
    fn from(qurl: &QUrl) -> Self {
        qurl.to_rust()
    }
}
