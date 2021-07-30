// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// We are only using references to String so it is actually ffi safe as far as we are concerned
#![allow(improper_ctypes)]

use crate::actually_private::Private;
use cxx::{type_id, ExternType};
use std::{
    marker::{PhantomData, PhantomPinned},
    mem::MaybeUninit,
    pin::Pin,
};

// This file is largely based on https://github.com/dtolnay/cxx/blob/master/src/cxx_string.rs
// and has merely been adapted for Qt's QString.

extern "C" {
    #[link_name = "cxxqt1$qstring$init"]
    fn qstring_init(this: &mut MaybeUninit<QString>, ptr: *const u8, len: usize);
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

/// Construct a Qt QString on the Rust stack.
///
/// # Syntax
///
/// In statement position:
///
/// ```
/// # use cxx_qt_lib::let_qstring;
/// # let expression = "";
/// let_qstring!(var = expression);
/// ```
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
/// ```
/// use cxx::{let_qstring, QString};
///
/// fn f(s: &QString) {/* ... */}
///
/// fn main() {
///     let_cxx_string!(s = "example");
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

    // Safety:
    //
    // Calling this function twice on the same StackQString is unsafe
    // and leads to undefined behaviour. It is therefore recommended
    // to not use this function directly and instead use the [`let_qstring!`]
    // macro which ensures that safe behaviour.
    pub unsafe fn init(&mut self, value: impl AsRef<[u8]>) -> Pin<&mut QString> {
        let value = value.as_ref();
        let this = &mut *self.space.as_mut_ptr().cast::<MaybeUninit<QString>>();
        qstring_init(this, value.as_ptr(), value.len());
        Pin::new_unchecked(&mut *this.as_mut_ptr())
    }
}

impl Drop for StackQString {
    fn drop(&mut self) {
        // Safety:
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

// Safety:
//
// The code in this file ensures that QString can only ever be allocated
// on the stack in pinned form which avoids the pitfalls of trying to
// move this type that has a non-trivial move constructor.
unsafe impl ExternType for QString {
    type Id = type_id!("QString");
    type Kind = cxx::kind::Opaque;
}
