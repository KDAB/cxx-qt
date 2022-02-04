// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// We are only using references to QColor so it is actually ffi safe as far as we are concerned
#![allow(improper_ctypes)]

use cxx::{memory::UniquePtrTarget, type_id, ExternType};
use std::{
    ffi::c_void,
    marker::{PhantomData, PhantomPinned},
    mem::MaybeUninit,
    pin::Pin,
};

extern "C" {
    #[link_name = "cxxqt1$qcolor$init$from$qcolor"]
    fn qcolor_init_from_qcolor(ptr: &mut MaybeUninit<cxx::UniquePtr<QColor>>, qcolor: &QColor);
    #[link_name = "cxxqt1$qcolor$init$from$rgba"]
    fn qcolor_init_from_rgba(
        ptr: &mut MaybeUninit<cxx::UniquePtr<QColor>>,
        red: i32,
        green: i32,
        blue: i32,
        alpha: i32,
    );
    #[link_name = "cxxqt1$qcolor$get$alpha"]
    fn qcolor_get_alpha(this: &QColor) -> i32;
    #[link_name = "cxxqt1$qcolor$get$red"]
    fn qcolor_get_red(this: &QColor) -> i32;
    #[link_name = "cxxqt1$qcolor$get$green"]
    fn qcolor_get_green(this: &QColor) -> i32;
    #[link_name = "cxxqt1$qcolor$get$blue"]
    fn qcolor_get_blue(this: &QColor) -> i32;
    #[link_name = "cxxqt1$qcolor$set$alpha"]
    fn qcolor_set_alpha(this: Pin<&mut QColor>, alpha: i32);
    #[link_name = "cxxqt1$qcolor$set$red"]
    fn qcolor_set_red(this: Pin<&mut QColor>, red: i32);
    #[link_name = "cxxqt1$qcolor$set$green"]
    fn qcolor_set_green(this: Pin<&mut QColor>, green: i32);
    #[link_name = "cxxqt1$qcolor$set$blue"]
    fn qcolor_set_blue(this: Pin<&mut QColor>, blue: i32);
}

/// Binding to Qt `QColor`.
///
/// # Invariants
///
/// As an invariant of this API and the static analysis of the cxx::bridge
/// macro, in Rust code we can never obtain a `QColor` by value. Qt's QColor
/// requires a move constructor and may hold internal pointers, which is not
/// compatible with Rust's move behavior. Instead in Rust code we will only ever
/// look at a QColor through a reference or smart pointer, as in `&QColor`
/// or `UniquePtr<QColor>`.
#[repr(C)]
pub struct QColor {
    _pinned: PhantomData<PhantomPinned>,
}

impl QColor {
    /// Create a new Rust Color from this QColor.
    /// This is a copy operation so any changes will not propagate to the original QColor.
    pub fn to_rust(&self) -> Color {
        Color::from_qcolor(self)
    }
}

// Safety:
//
// The code in this file ensures that QColor can only ever be allocated
// on the stack in pinned form which avoids the pitfalls of trying to
// move this type that has a non-trivial move constructor.
unsafe impl ExternType for QColor {
    type Id = type_id!("QColor");
    type Kind = cxx::kind::Opaque;
}

extern "C" {
    #[link_name = "cxxqt1$unique_ptr$qcolor$null"]
    fn unique_ptr_qcolor_null(this: *mut MaybeUninit<*mut c_void>);
    #[link_name = "cxxqt1$unique_ptr$qcolor$raw"]
    fn unique_ptr_qcolor_raw(this: *mut MaybeUninit<*mut c_void>, raw: *mut QColor);
    #[link_name = "cxxqt1$unique_ptr$qcolor$get"]
    fn unique_ptr_qcolor_get(this: *const MaybeUninit<*mut c_void>) -> *const QColor;
    #[link_name = "cxxqt1$unique_ptr$qcolor$release"]
    fn unique_ptr_qcolor_release(this: *mut MaybeUninit<*mut c_void>) -> *mut QColor;
    #[link_name = "cxxqt1$unique_ptr$qcolor$drop"]
    fn unique_ptr_qcolor_drop(this: *mut MaybeUninit<*mut c_void>);
}

// Safety: TODO
unsafe impl UniquePtrTarget for QColor {
    #[doc(hidden)]
    fn __typename(f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("QColor")
    }

    #[doc(hidden)]
    fn __null() -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unsafe {
            unique_ptr_qcolor_null(&mut repr);
        }
        repr
    }

    #[doc(hidden)]
    unsafe fn __raw(raw: *mut Self) -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unique_ptr_qcolor_raw(&mut repr, raw);
        repr
    }

    #[doc(hidden)]
    unsafe fn __get(repr: MaybeUninit<*mut c_void>) -> *const Self {
        unique_ptr_qcolor_get(&repr)
    }

    #[doc(hidden)]
    unsafe fn __release(mut repr: MaybeUninit<*mut c_void>) -> *mut Self {
        unique_ptr_qcolor_release(&mut repr)
    }

    #[doc(hidden)]
    unsafe fn __drop(mut repr: MaybeUninit<*mut c_void>) {
        unique_ptr_qcolor_drop(&mut repr)
    }
}

/// The Rust representation of Qt's QColor
///
/// Internally this holds a UniquePtr to a QColor which has been constructed on the C++ side.
pub struct Color {
    // Note that once map_qt_value is removed later, this can become private again
    #[doc(hidden)]
    pub(crate) inner: cxx::UniquePtr<QColor>,
}

impl Color {
    /// Construct a Rust Color from an existing UniquePtr<QColor> this is a move operation
    ///
    /// This is used in QVariant::value so that we don't need to perform another copy
    pub(crate) fn from_unique_ptr(ptr: cxx::UniquePtr<QColor>) -> Self {
        Self { inner: ptr }
    }

    /// Construct a Rust Color from an existing QColor, this is a copy operation.
    pub fn from_qcolor(qcolor: &QColor) -> Self {
        Self {
            // Safety: TODO
            inner: unsafe {
                let mut ptr = MaybeUninit::<cxx::UniquePtr<QColor>>::zeroed();
                qcolor_init_from_qcolor(&mut ptr, qcolor);
                ptr.assume_init()
            },
        }
    }

    /// Construct a Rust Color from a given set of RGBA values
    pub fn from_rgba(red: i32, green: i32, blue: i32, alpha: i32) -> Self {
        Self {
            // Safety: TODO
            inner: unsafe {
                let mut ptr = MaybeUninit::<cxx::UniquePtr<QColor>>::zeroed();
                qcolor_init_from_rgba(&mut ptr, red, green, blue, alpha);
                ptr.assume_init()
            },
        }
    }

    pub fn alpha(&self) -> i32 {
        // Safety: TODO
        unsafe { qcolor_get_alpha(&self.inner) }
    }

    pub fn blue(&self) -> i32 {
        // Safety: TODO
        unsafe { qcolor_get_blue(&self.inner) }
    }

    pub fn green(&self) -> i32 {
        // Safety: TODO
        unsafe { qcolor_get_green(&self.inner) }
    }

    pub fn red(&self) -> i32 {
        // Safety: TODO
        unsafe { qcolor_get_red(&self.inner) }
    }

    pub fn set_alpha(&mut self, alpha: i32) {
        // Safety: TODO
        unsafe {
            qcolor_set_alpha(self.inner.pin_mut(), alpha);
        }
    }

    pub fn set_blue(&mut self, blue: i32) {
        // Safety: TODO
        unsafe {
            qcolor_set_blue(self.inner.pin_mut(), blue);
        }
    }

    pub fn set_green(&mut self, green: i32) {
        // Safety: TODO
        unsafe {
            qcolor_set_green(self.inner.pin_mut(), green);
        }
    }

    pub fn set_red(&mut self, red: i32) {
        // Safety: TODO
        unsafe {
            qcolor_set_red(self.inner.pin_mut(), red);
        }
    }
}

impl crate::ToUniquePtr for Color {
    type CppType = QColor;

    /// Retrieve the UniquePtr to the Qt QColor of this Rust Color
    /// so that this object can be passed back to C++.
    fn to_unique_ptr(self) -> cxx::UniquePtr<QColor> {
        self.inner
    }
}

impl From<&QColor> for Color {
    fn from(qcolor: &QColor) -> Self {
        qcolor.to_rust()
    }
}
