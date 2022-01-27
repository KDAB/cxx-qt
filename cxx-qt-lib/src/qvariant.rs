// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// We are only using references to QVariant so it is actually ffi safe as far as we are concerned
#![allow(improper_ctypes)]

use crate::actually_private::Private;
use cxx::{memory::UniquePtrTarget, type_id, ExternType};
use std::{
    ffi::c_void,
    marker::{PhantomData, PhantomPinned},
    mem::{self, ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    pin::Pin,
};

#[repr(u8)]
#[allow(dead_code)]
enum QVariantType {
    Unsupported = 0,
    Bool = 1,
    F32 = 2,
    F64 = 3,
    I8 = 4,
    I16 = 5,
    I32 = 6,
    String = 7,
    U8 = 8,
    U16 = 9,
    U32 = 10,
}

extern "C" {
    #[link_name = "cxxqt1$qvariant$init"]
    fn qvariant_init(this: &mut MaybeUninit<QVariant>);
    #[link_name = "cxxqt1$qvariant$init$from$bool"]
    fn qvariant_init_from_bool(this: &mut MaybeUninit<QVariant>, b: bool);
    #[link_name = "cxxqt1$qvariant$init$from$f32"]
    fn qvariant_init_from_f32(this: &mut MaybeUninit<QVariant>, i: f32);
    #[link_name = "cxxqt1$qvariant$init$from$f64"]
    fn qvariant_init_from_f64(this: &mut MaybeUninit<QVariant>, i: f64);
    #[link_name = "cxxqt1$qvariant$init$from$i8"]
    fn qvariant_init_from_i8(this: &mut MaybeUninit<QVariant>, i: i8);
    #[link_name = "cxxqt1$qvariant$init$from$i16"]
    fn qvariant_init_from_i16(this: &mut MaybeUninit<QVariant>, i: i16);
    #[link_name = "cxxqt1$qvariant$init$from$i32"]
    fn qvariant_init_from_i32(this: &mut MaybeUninit<QVariant>, i: i32);
    #[link_name = "cxxqt1$qvariant$init$from$str"]
    fn qvariant_init_from_str(this: &mut MaybeUninit<QVariant>, s: &str);
    #[link_name = "cxxqt1$qvariant$init$from$u8"]
    fn qvariant_init_from_u8(this: &mut MaybeUninit<QVariant>, i: u8);
    #[link_name = "cxxqt1$qvariant$init$from$u16"]
    fn qvariant_init_from_u16(this: &mut MaybeUninit<QVariant>, i: u16);
    #[link_name = "cxxqt1$qvariant$init$from$u32"]
    fn qvariant_init_from_u32(this: &mut MaybeUninit<QVariant>, i: u32);
    #[link_name = "cxxqt1$qvariant$get$type"]
    fn qvariant_get_type(this: &QVariant) -> QVariantType;
    #[link_name = "cxxqt1$qvariant$to$bool"]
    fn qvariant_to_bool(this: &QVariant) -> bool;
    #[link_name = "cxxqt1$qvariant$to$f32"]
    fn qvariant_to_f32(this: &QVariant) -> f32;
    #[link_name = "cxxqt1$qvariant$to$f64"]
    fn qvariant_to_f64(this: &QVariant) -> f64;
    #[link_name = "cxxqt1$qvariant$to$i8"]
    fn qvariant_to_i8(this: &QVariant) -> i8;
    #[link_name = "cxxqt1$qvariant$to$i16"]
    fn qvariant_to_i16(this: &QVariant) -> i16;
    #[link_name = "cxxqt1$qvariant$to$i32"]
    fn qvariant_to_i32(this: &QVariant) -> i32;
    #[link_name = "cxxqt1$qvariant$copy$to$string"]
    fn qvariant_copy_to_string(this: &QVariant, s: &mut String);
    #[link_name = "cxxqt1$qvariant$to$u8"]
    fn qvariant_to_u8(this: &QVariant) -> u8;
    #[link_name = "cxxqt1$qvariant$to$u16"]
    fn qvariant_to_u16(this: &QVariant) -> u16;
    #[link_name = "cxxqt1$qvariant$to$u32"]
    fn qvariant_to_u32(this: &QVariant) -> u32;
    #[link_name = "cxxqt1$qvariant$assign$qvariant"]
    fn qvariant_assign_qvariant(from: &QVariant, to: &mut QVariant);
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
/// # let expression = Variant::I32(123);
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
    pub fn to_rust(&self) -> Variant {
        // Given that a QVariant can only be constructed using [`let_qvariant!`] macro,
        // it is safe to assume that self is a valid QVariant reference which makes these
        // function calls safe.
        match unsafe { qvariant_get_type(self) } {
            QVariantType::Unsupported => Variant::unsupported(),
            QVariantType::Bool => Variant::from_bool(unsafe { qvariant_to_bool(self) }),
            QVariantType::F32 => Variant::from_f32(unsafe { qvariant_to_f32(self) }),
            QVariantType::F64 => Variant::from_f64(unsafe { qvariant_to_f64(self) }),
            QVariantType::I8 => Variant::from_i8(unsafe { qvariant_to_i8(self) }),
            QVariantType::I16 => Variant::from_i16(unsafe { qvariant_to_i16(self) }),
            QVariantType::I32 => Variant::from_i32(unsafe { qvariant_to_i32(self) }),
            QVariantType::String => {
                let mut s = String::new();
                unsafe { qvariant_copy_to_string(self, &mut s) };
                Variant::from_string(s)
            }
            QVariantType::U8 => Variant::from_u8(unsafe { qvariant_to_u8(self) }),
            QVariantType::U16 => Variant::from_u16(unsafe { qvariant_to_u16(self) }),
            QVariantType::U32 => Variant::from_u32(unsafe { qvariant_to_u32(self) }),
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

        match value.deref() {
            VariantImpl::Unsupported => qvariant_init(this),
            VariantImpl::Bool(b) => qvariant_init_from_bool(this, *b),
            VariantImpl::F32(i) => qvariant_init_from_f32(this, *i),
            VariantImpl::F64(i) => qvariant_init_from_f64(this, *i),
            VariantImpl::I8(i) => qvariant_init_from_i8(this, *i),
            VariantImpl::I16(i) => qvariant_init_from_i16(this, *i),
            VariantImpl::I32(i) => qvariant_init_from_i32(this, *i),
            VariantImpl::String(s) => qvariant_init_from_str(this, s),
            VariantImpl::U8(i) => qvariant_init_from_u8(this, *i),
            VariantImpl::U16(i) => qvariant_init_from_u16(this, *i),
            VariantImpl::U32(i) => qvariant_init_from_u32(this, *i),
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

extern "C" {
    #[link_name = "cxxqt1$unique_ptr$qvariant$null"]
    fn unique_ptr_qvariant_null(this: *mut MaybeUninit<*mut c_void>);
    #[link_name = "cxxqt1$unique_ptr$qvariant$raw"]
    fn unique_ptr_qvariant_raw(this: *mut MaybeUninit<*mut c_void>, raw: *mut QVariant);
    #[link_name = "cxxqt1$unique_ptr$qvariant$get"]
    fn unique_ptr_qvariant_get(this: *const MaybeUninit<*mut c_void>) -> *const QVariant;
    #[link_name = "cxxqt1$unique_ptr$qvariant$release"]
    fn unique_ptr_qvariant_release(this: *mut MaybeUninit<*mut c_void>) -> *mut QVariant;
    #[link_name = "cxxqt1$unique_ptr$qvariant$drop"]
    fn unique_ptr_qvariant_drop(this: *mut MaybeUninit<*mut c_void>);
}

unsafe impl UniquePtrTarget for QVariant {
    #[doc(hidden)]
    fn __typename(f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("QVariant")
    }

    #[doc(hidden)]
    fn __null() -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unsafe {
            unique_ptr_qvariant_null(&mut repr);
        }
        repr
    }

    #[doc(hidden)]
    unsafe fn __raw(raw: *mut Self) -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unique_ptr_qvariant_raw(&mut repr, raw);
        repr
    }

    #[doc(hidden)]
    unsafe fn __get(repr: MaybeUninit<*mut c_void>) -> *const Self {
        unique_ptr_qvariant_get(&repr)
    }

    #[doc(hidden)]
    unsafe fn __release(mut repr: MaybeUninit<*mut c_void>) -> *mut Self {
        unique_ptr_qvariant_release(&mut repr)
    }

    #[doc(hidden)]
    unsafe fn __drop(mut repr: MaybeUninit<*mut c_void>) {
        unique_ptr_qvariant_drop(&mut repr)
    }
}

pub enum VariantImpl {
    Unsupported,
    Bool(bool),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    String(String),
    U8(u8),
    U16(u16),
    U32(u32),
}

#[repr(C)]
pub struct Variant {
    pub d: Box<VariantImpl>,
}

impl Variant {
    pub fn from_bool(b: bool) -> Self {
        Self {
            d: Box::new(VariantImpl::Bool(b)),
        }
    }

    pub fn from_f32(f: f32) -> Self {
        Self {
            d: Box::new(VariantImpl::F32(f)),
        }
    }

    pub fn from_f64(f: f64) -> Self {
        Self {
            d: Box::new(VariantImpl::F64(f)),
        }
    }

    pub fn from_i8(i: i8) -> Self {
        Self {
            d: Box::new(VariantImpl::I8(i)),
        }
    }

    pub fn from_i16(i: i16) -> Self {
        Self {
            d: Box::new(VariantImpl::I16(i)),
        }
    }

    pub fn from_i32(i: i32) -> Self {
        Self {
            d: Box::new(VariantImpl::I32(i)),
        }
    }

    pub fn from_string(s: String) -> Self {
        Self {
            d: Box::new(VariantImpl::String(s)),
        }
    }

    pub fn from_u8(i: u8) -> Self {
        Self {
            d: Box::new(VariantImpl::U8(i)),
        }
    }

    pub fn from_u16(i: u16) -> Self {
        Self {
            d: Box::new(VariantImpl::U16(i)),
        }
    }

    pub fn from_u32(i: u32) -> Self {
        Self {
            d: Box::new(VariantImpl::U32(i)),
        }
    }

    pub fn unsupported() -> Self {
        Self {
            d: Box::new(VariantImpl::Unsupported),
        }
    }
}

impl Deref for Variant {
    type Target = VariantImpl;

    fn deref(&self) -> &Self::Target {
        &self.d
    }
}

impl DerefMut for Variant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.d
    }
}

const_assert_eq!(mem::size_of::<Variant>(), mem::size_of::<*mut c_void>());
const_assert_eq!(mem::align_of::<Variant>(), mem::align_of::<*mut c_void>());

// Safety
//
// By implementing Variant through the "pimpl" idiom we ensure
// that is equivalent to void* in C++ and thus 100% trivial.
// We also have static asserts to ensure that this remains true.
unsafe impl ExternType for Variant {
    type Id = type_id!("CxxQt::Variant");
    type Kind = cxx::kind::Trivial;
}

#[export_name = "cxxqt1$assign$variant$to$qvariant"]
pub unsafe extern "C" fn assign_variant_to_qvariant(rust: &Variant, cpp: &mut QVariant) {
    // TODO: this could probably be optimised by having dedicated functions to assign to cpp directly
    // rather than to first create a new QVariant. The best would be to change the init... functions
    // to assign... and then have a single init function to create an empty QVariant.
    let_qvariant!(q = rust);
    qvariant_assign_qvariant(&q, cpp);
}

#[export_name = "cxxqt1$drop$variant"]
pub unsafe extern "C" fn drop_variant(this: &mut ManuallyDrop<Variant>) {
    ManuallyDrop::drop(this);
}

impl From<&QVariant> for Variant {
    fn from(qvariant: &QVariant) -> Self {
        qvariant.to_rust()
    }
}
