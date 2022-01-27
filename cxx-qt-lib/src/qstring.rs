// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// We are only using references to String so it is actually ffi safe as far as we are concerned
#![allow(improper_ctypes)]

use crate::actually_private::Private;
use cxx::{memory::UniquePtrTarget, type_id, ExternType};
use std::{
    ffi::c_void,
    marker::{PhantomData, PhantomPinned},
    mem::MaybeUninit,
    pin::Pin,
};

// This file is largely based on https://github.com/dtolnay/cxx/blob/master/src/cxx_string.rs
// and has merely been adapted for Qt's QString.

extern "C" {
    #[link_name = "cxxqt1$qstring$init"]
    fn qstring_init(this: &mut MaybeUninit<QString>, ptr: *const u8, len: usize);
    #[link_name = "cxxqt1$qstring$init$unique$ptr"]
    fn qstring_init_unique_ptr(
        this: &mut MaybeUninit<cxx::UniquePtr<QString>>,
        ptr: *const u8,
        len: usize,
    );
    #[link_name = "cxxqt1$qstring$to_rust_string"]
    fn qstring_to_rust_string(qt: &QString, rust: &mut String);
    #[link_name = "cxxqt1$qstring$drop"]
    fn qstring_drop(this: &mut MaybeUninit<QString>);
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

// TODO: figure out how to make Syntax and Example compile as code
// and then change ```ignore back to ```

/// Construct a QString on the Rust stack.
///
/// # Syntax
///
/// In statement position:
///
/// ```ignore
/// # use cxx_qt_lib::let_qstring;
/// # let expression = "";
/// let_qstring!(var = expression);
/// ```
///
///
/// The `expression` may have any type that implements `AsRef<[u8]>`. Commonly
/// it will be a string literal, but for example `&[u8]` and `String` would work
/// as well.
///
/// The macro expands to something resembling `let $var: Pin<&mut QString> =
/// /*???*/;`. The resulting [`Pin`] can be deref'd to `&QString` as needed.
///
/// # Example
///
/// ```ignore
/// use cxx_qt_lib::{let_qstring, QString};
///
///
/// fn f(s: &QString) {/* ... */}
///
/// fn main() {
///     let_qstring!(s = "example");
///     f(&s);
/// }
/// ```
#[macro_export]
macro_rules! let_qstring {
    ($var:ident = $value:expr $(,)?) => {
        let mut stack_qstring = $crate::private::StackQString::new();
        #[allow(unused_mut, unused_unsafe)]
        let mut $var = match $value {
            let_qstring => unsafe { stack_qstring.init(let_qstring) },
        };
    };
}

/// Constructs a UniquePtr<QString> on the stack similar to `let_qstring` above.
///
/// Note that the UniquePtr must be passed to C++ and released there, as we
/// don't implement Drop on the Rust side to avoid double free
#[macro_export]
macro_rules! let_qstring_unique_ptr {
    ($var:ident = $value:expr $(,)?) => {
        let mut stack_qstring = $crate::private::StackQStringUniquePtr::new();
        #[allow(unused_mut, unused_unsafe)]
        let mut $var = match $value {
            let_qstring_unique_ptr => unsafe {
                stack_qstring.init_unique_ptr(let_qstring_unique_ptr)
            },
        };
    };
}

impl QString {
    /// `QString` is not constructible via `new`.
    /// Instead, use the [`let_qstring!`] macro.
    pub fn new<T: Private>() -> Self {
        unreachable!()
    }

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

#[doc(hidden)]
#[repr(C)]
pub struct StackQString {
    // Static assertions in cxx_qt.cpp validate that this
    // is large enough and aligned enough.
    space: MaybeUninit<usize>,
}

// We could have implemented QString so that it contains a "space" field itself,
// but having a separate StackQString ensures better safety. This is because
// we can use macro hygiene to place a StackQString on the stack without
// giving the user a way to name said StackQString and thus prevent them
// from gaining direct access to the "space" field and doing something unsafe.
//
// Instead, the macro ensures that users can only get a handle to the underlying
// data through a safe Pin<&mut QString> which does not expose the data directly.

#[allow(missing_docs)]
impl StackQString {
    pub fn new() -> Self {
        StackQString {
            space: MaybeUninit::uninit(),
        }
    }

    /// # Safety
    ///
    /// Calling this function twice on the same StackQString is unsafe
    /// and leads to undefined behaviour. It is therefore recommended
    /// to not use this function directly and instead use the [`let_qstring!`]
    /// macro which ensures that safe behaviour.
    pub unsafe fn init(&mut self, value: impl AsRef<[u8]>) -> Pin<&mut QString> {
        let value = value.as_ref();
        let this = &mut *self.space.as_mut_ptr().cast::<MaybeUninit<QString>>();
        qstring_init(this, value.as_ptr(), value.len());
        Pin::new_unchecked(&mut *this.as_mut_ptr())
    }
}

impl Default for StackQString {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StackQString {
    fn drop(&mut self) {
        // # Safety
        //
        // This simply calls ~QString on self.space which is safe as long
        // as self.space contains a valid QString. Using the [`let_qstring!`]
        // macro guarantees that this will be the case.
        unsafe {
            let this = &mut *self.space.as_mut_ptr().cast::<MaybeUninit<QString>>();
            qstring_drop(this);
        }
    }
}

// Similar to StackQString above but is a UniquePtr version
//
// We don't Drop a StackQStringUniquePtr as this is released on the C++ side
#[doc(hidden)]
#[repr(C)]
pub struct StackQStringUniquePtr {
    // Static assertions in cxx_qt.cpp validate that this
    // is large enough and aligned enough.
    space: MaybeUninit<usize>,
}

#[allow(missing_docs)]
impl StackQStringUniquePtr {
    pub fn new() -> Self {
        StackQStringUniquePtr {
            space: MaybeUninit::uninit(),
        }
    }

    pub unsafe fn init_unique_ptr(mut self, value: impl AsRef<[u8]>) -> cxx::UniquePtr<QString> {
        let value = value.as_ref();
        let this = &mut *self
            .space
            .as_mut_ptr()
            .cast::<MaybeUninit<cxx::UniquePtr<QString>>>();
        qstring_init_unique_ptr(this, value.as_ptr(), value.len());
        std::ptr::read(this.as_mut_ptr())
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

impl From<&QString> for String {
    fn from(qstring: &QString) -> Self {
        qstring.to_rust()
    }
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
