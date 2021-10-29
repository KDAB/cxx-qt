// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// We are only using references to QVariant so it is actually ffi safe as far as we are concerned
#![allow(improper_ctypes)]

use crate::actually_private::Private;
use cxx::{type_id, ExternType};
use std::{
    marker::{PhantomData, PhantomPinned},
    mem::MaybeUninit,
    pin::Pin,
};

#[repr(u8)]
#[allow(dead_code)]
enum QVariantType {
    Unsupported = 0,
    String = 1,
    Int = 2,
    Bool = 3,
}

extern "C" {
    #[link_name = "cxxqt1$qvariant$init$from$int"]
    fn qvariant_init_from_int(this: &mut MaybeUninit<QVariant>, i: i32);
    #[link_name = "cxxqt1$qvariant$init$from$str"]
    fn qvariant_init_from_str(this: &mut MaybeUninit<QVariant>, s: &str);
    #[link_name = "cxxqt1$qvariant$init$from$bool"]
    fn qvariant_init_from_bool(this: &mut MaybeUninit<QVariant>, b: bool);
    #[link_name = "cxxqt1$qvariant$get$type"]
    fn qvariant_get_type(this: &QVariant) -> QVariantType;
    #[link_name = "cxxqt1$qvariant$copy$to$string"]
    fn qvariant_copy_to_string(this: &QVariant, s: &mut String);
    #[link_name = "cxxqt1$qvariant$to$int"]
    fn qvariant_to_int(this: &QVariant) -> i32;
    #[link_name = "cxxqt1$qvariant$to$bool"]
    fn qvariant_to_bool(this: &QVariant) -> bool;
    #[link_name = "cxxqt1$qvariant$drop"]
    fn qvariant_drop(this: &mut MaybeUninit<QVariant>);
}

/// Binding to Qt `QVariant`.
///
/// # Invariants
///
/// As an invariant of this API and the static analysis of the cxx::bridge
/// macro, in Rust code we can never obtain a `QVariant` by value. Qt's QVariant
/// requires a move constructor and may hold internal pointers, which is not
/// compatible with Rust's move behavior. Instead in Rust code we will only ever
/// look at a QVariant through a reference or smart pointer, as in `&QVariant`
/// or `UniquePtr<QVariant>`.
#[repr(C)]
pub struct QVariant {
    _pinned: PhantomData<PhantomPinned>,
}

// TODO: figure out how to make Syntax and Example compile as code
// and then change ```ignore back to ```

/// Construct a QVariant on the Rust stack.
///
/// # Syntax
///
/// In statement position:
///
/// ```ignore
/// # use cxx_qt_lib::{let_qvariant, Variant};
/// # let expression = Variant::Int(123);
/// let_qvariant!(var = expression);
/// ```
///
///
/// The `expression` must refer to a `&cxx_qt_lib::Variant`.
///
/// The macro expands to something resembling `let $var: Pin<&mut QVariant> =
/// /*???*/;`. The resulting [`Pin`] can be deref'd to `&QVariant` as needed.
///
/// # Example
///
/// ```ignore
/// use cxx_qt_lib::{let_qvariant, QVariant};
///
///
/// fn f(s: &QVariant) {/* ... */}
///
/// fn main() {
///     let_qvariant!(s = "example");
///     f(&s);
/// }
/// ```
#[macro_export]
macro_rules! let_qvariant {
    ($var:ident = $value:expr $(,)?) => {
        let mut stack_qvariant = $crate::private::StackQVariant::new();
        #[allow(unused_mut, unused_unsafe)]
        let mut $var = match $value {
            let_qvariant => unsafe { stack_qvariant.init(let_qvariant) },
        };
    };
}

impl QVariant {
    /// `QVariant` is not constructible via `new`.
    /// Instead, use the [`let_qvariant!`] macro.
    pub fn new<T: Private>() -> Self {
        unreachable!()
    }

    /// Create a new Rust Variant from this QVariant.
    /// This is a copy operation so any changes will not propagate to the original QVariant.
    pub fn to_rust(&self) -> Option<Variant> {
        // Given that a QVariant can only be constructed using [`let_qvariant!`] macro,
        // it is safe to assume that self is a valid QVariant reference which makes these
        // function calls safe.
        match unsafe { qvariant_get_type(self) } {
            QVariantType::Unsupported => None,
            QVariantType::String => {
                let mut s = String::new();
                unsafe { qvariant_copy_to_string(self, &mut s) };
                Some(Variant::String(s))
            }
            QVariantType::Int => Some(Variant::Int(unsafe { qvariant_to_int(self) })),
            QVariantType::Bool => Some(Variant::Bool(unsafe { qvariant_to_bool(self) })),
        }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct StackQVariant {
    // Static assertions in cxx_qt.cpp validate that this
    // is large enough and aligned enough.
    space: MaybeUninit<[usize; 2]>,
}

// We could have implemented QVariant so that it contains a "space" field itself,
// but having a separate StackQVariant ensures better safety. This is because
// we can use macro hygiene to place a StackQVariant on the stack without
// giving the user a way to name said StackQVariant and thus prevent them
// from gaining direct access to the "space" field and doing something unsafe.
//
// Instead, the macro ensures that users can only get a handle to the underlying
// data through a safe Pin<&mut QVariant> which does not expose the data directly.

#[allow(missing_docs)]
impl StackQVariant {
    pub fn new() -> Self {
        StackQVariant {
            space: MaybeUninit::uninit(),
        }
    }

    /// # Safety
    ///
    /// Calling this function twice on the same StackQVariant is unsafe
    /// and leads to undefined behaviour. It is therefore recommended
    /// to not use this function directly and instead use the [`let_qvariant!`]
    /// macro which ensures that safe behaviour.
    pub unsafe fn init(&mut self, value: &Variant) -> Pin<&mut QVariant> {
        let this = &mut *self.space.as_mut_ptr().cast::<MaybeUninit<QVariant>>();

        match value {
            Variant::String(s) => qvariant_init_from_str(this, s),
            Variant::Int(i) => qvariant_init_from_int(this, *i),
            Variant::Bool(b) => qvariant_init_from_bool(this, *b),
        }

        Pin::new_unchecked(&mut *this.as_mut_ptr())
    }
}

impl Default for StackQVariant {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StackQVariant {
    fn drop(&mut self) {
        // # Safety
        //
        // This simply calls ~QVariant on self.space which is safe as long
        // as self.space contains a valid QVariant. Using the [`let_qvariant!`]
        // macro guarantees that this will be the case.
        unsafe {
            let this = &mut *self.space.as_mut_ptr().cast::<MaybeUninit<QVariant>>();
            qvariant_drop(this);
        }
    }
}

// Safety:
//
// The code in this file ensures that QVariant can only ever be allocated
// on the stack in pinned form which avoids the pitfalls of trying to
// move this type that has a non-trivial move constructor.
unsafe impl ExternType for QVariant {
    type Id = type_id!("QVariant");
    type Kind = cxx::kind::Opaque;
}

pub enum Variant {
    String(String),
    Int(i32),
    Bool(bool),
}

impl From<&QVariant> for Option<Variant> {
    fn from(qvariant: &QVariant) -> Self {
        qvariant.to_rust()
    }
}
