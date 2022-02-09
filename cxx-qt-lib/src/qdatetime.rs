// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// We are only using references to QDateTime so it is actually ffi safe as far as we are concerned
#![allow(improper_ctypes)]

use crate::{QDate, QTime};
use cxx::{memory::UniquePtrTarget, type_id, ExternType};
use std::{
    ffi::c_void,
    marker::{PhantomData, PhantomPinned},
    mem::MaybeUninit,
    pin::Pin,
};

extern "C" {
    #[link_name = "cxxqt1$qdatetime$init$from$qdatetime"]
    fn qdatetime_init_from_qdatetime(
        ptr: &mut MaybeUninit<cxx::UniquePtr<QDateTime>>,
        qdatetime: &QDateTime,
    );
    #[link_name = "cxxqt1$qdatetime$init$from$date$and$time"]
    fn qdatetime_init_from_date_and_time(
        ptr: &mut MaybeUninit<cxx::UniquePtr<QDateTime>>,
        date: &QDate,
        time: &QTime,
    );
    #[link_name = "cxxqt1$qdatetime$get$date"]
    fn qdatetime_get_date(this: &QDateTime) -> QDate;
    #[link_name = "cxxqt1$qdatetime$get$time"]
    fn qdatetime_get_time(this: &QDateTime) -> QTime;
    #[link_name = "cxxqt1$qdatetime$set$date"]
    fn qdatetime_set_date(this: Pin<&mut QDateTime>, date: &QDate);
    #[link_name = "cxxqt1$qdatetime$set$time"]
    fn qdatetime_set_time(this: Pin<&mut QDateTime>, time: &QTime);
}

/// Binding to Qt `QDateTime`.
///
/// # Invariants
///
/// As an invariant of this API and the static analysis of the cxx::bridge
/// macro, in Rust code we can never obtain a `QDateTime` by value. Qt's QDateTime
/// requires a move constructor and may hold internal pointers, which is not
/// compatible with Rust's move behavior. Instead in Rust code we will only ever
/// look at a QDateTime through a reference or smart pointer, as in `&QDateTime`
/// or `UniquePtr<QDateTime>`.
#[repr(C)]
pub struct QDateTime {
    _pinned: PhantomData<PhantomPinned>,
}

impl QDateTime {
    /// Create a new Rust DateTime from this QDateTime.
    /// This is a copy operation so any changes will not propagate to the original QDateTime.
    pub fn to_rust(&self) -> DateTime {
        DateTime::from_qdatetime(self)
    }
}

// Safety:
//
// The code in this file ensures that QDateTime can only ever be allocated
// on the stack in pinned form which avoids the pitfalls of trying to
// move this type that has a non-trivial move constructor.
unsafe impl ExternType for QDateTime {
    type Id = type_id!("QDateTime");
    type Kind = cxx::kind::Opaque;
}

extern "C" {
    #[link_name = "cxxqt1$unique_ptr$qdatetime$null"]
    fn unique_ptr_qdatetime_null(this: *mut MaybeUninit<*mut c_void>);
    #[link_name = "cxxqt1$unique_ptr$qdatetime$raw"]
    fn unique_ptr_qdatetime_raw(this: *mut MaybeUninit<*mut c_void>, raw: *mut QDateTime);
    #[link_name = "cxxqt1$unique_ptr$qdatetime$get"]
    fn unique_ptr_qdatetime_get(this: *const MaybeUninit<*mut c_void>) -> *const QDateTime;
    #[link_name = "cxxqt1$unique_ptr$qdatetime$release"]
    fn unique_ptr_qdatetime_release(this: *mut MaybeUninit<*mut c_void>) -> *mut QDateTime;
    #[link_name = "cxxqt1$unique_ptr$qdatetime$drop"]
    fn unique_ptr_qdatetime_drop(this: *mut MaybeUninit<*mut c_void>);
}

// Safety: TODO
unsafe impl UniquePtrTarget for QDateTime {
    #[doc(hidden)]
    fn __typename(f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("QDateTime")
    }

    #[doc(hidden)]
    fn __null() -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unsafe {
            unique_ptr_qdatetime_null(&mut repr);
        }
        repr
    }

    #[doc(hidden)]
    unsafe fn __raw(raw: *mut Self) -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unique_ptr_qdatetime_raw(&mut repr, raw);
        repr
    }

    #[doc(hidden)]
    unsafe fn __get(repr: MaybeUninit<*mut c_void>) -> *const Self {
        unique_ptr_qdatetime_get(&repr)
    }

    #[doc(hidden)]
    unsafe fn __release(mut repr: MaybeUninit<*mut c_void>) -> *mut Self {
        unique_ptr_qdatetime_release(&mut repr)
    }

    #[doc(hidden)]
    unsafe fn __drop(mut repr: MaybeUninit<*mut c_void>) {
        unique_ptr_qdatetime_drop(&mut repr)
    }
}

/// The Rust representation of Qt's QDateTime
///
/// Internally this holds a UniquePtr to a QDateTime which has been constructed on the C++ side.
pub struct DateTime {
    // Note that once map_qt_value is removed later, this can become private again
    #[doc(hidden)]
    pub(crate) inner: cxx::UniquePtr<QDateTime>,
}

impl DateTime {
    /// Construct a Rust DateTime from an existing UniquePtr<QDateTime> this is a move operation
    ///
    /// This is used in QVariant::value so that we don't need to perform another copy
    pub(crate) fn from_unique_ptr(ptr: cxx::UniquePtr<QDateTime>) -> Self {
        Self { inner: ptr }
    }

    /// Construct a Rust DateTime from an existing QDateTime, this is a copy operation.
    pub fn from_qdatetime(qdatetime: &QDateTime) -> Self {
        Self {
            // Safety: TODO
            inner: unsafe {
                let mut ptr = MaybeUninit::<cxx::UniquePtr<QDateTime>>::zeroed();
                qdatetime_init_from_qdatetime(&mut ptr, qdatetime);
                ptr.assume_init()
            },
        }
    }

    /// Construct a Rust DateTime from a given QDate and QTime
    pub fn from_date_and_time(date: &QDate, time: &QTime) -> Self {
        Self {
            // Safety: TODO
            inner: unsafe {
                let mut ptr = MaybeUninit::<cxx::UniquePtr<QDateTime>>::zeroed();
                qdatetime_init_from_date_and_time(&mut ptr, date, time);
                ptr.assume_init()
            },
        }
    }

    pub fn date(&self) -> QDate {
        // Safety: TODO
        unsafe { qdatetime_get_date(&self.inner) }
    }

    pub fn time(&self) -> QTime {
        // Safety: TODO
        unsafe { qdatetime_get_time(&self.inner) }
    }

    pub fn set_date(&mut self, date: &QDate) {
        // Safety: TODO
        unsafe {
            qdatetime_set_date(self.inner.pin_mut(), date);
        }
    }

    pub fn set_time(&mut self, time: &QTime) {
        // Safety: TODO
        unsafe {
            qdatetime_set_time(self.inner.pin_mut(), time);
        }
    }
}

impl crate::ToUniquePtr for DateTime {
    type CppType = QDateTime;

    /// Retrieve the UniquePtr to the Qt QDateTime of this Rust DateTime
    /// so that this object can be passed back to C++.
    fn to_unique_ptr(self) -> cxx::UniquePtr<QDateTime> {
        self.inner
    }
}

impl From<&QDateTime> for DateTime {
    fn from(qdatetime: &QDateTime) -> Self {
        qdatetime.to_rust()
    }
}
