// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Goins <josh@redstrate.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::QString;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qanystringview.h");
        type QAnyStringView<'a> = super::QAnyStringView<'a>;

        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Returns true if the string has no characters; otherwise returns false.
        #[rust_name = "is_empty"]
        fn isEmpty(self: &QAnyStringView) -> bool;

        /// Returns true if this string is null; otherwise returns false.
        #[rust_name = "is_null"]
        fn isNull(self: &QAnyStringView) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "QAnyStringView_init_default"]
        fn construct() -> QAnyStringView<'static>;
        #[doc(hidden)]
        #[rust_name = "QAnyStringView_init_from_rust_string"]
        fn qanystringviewInitFromRustString<'a>(string: &str) -> QAnyStringView<'a>;
        #[doc(hidden)]
        #[rust_name = "QAnyStringView_init_from_qstring"]
        fn qanystringviewInitFromQString<'a>(string: &QString) -> QAnyStringView<'a>;
        #[doc(hidden)]
        #[rust_name = "QAnyStringView_init_from_QAnyStringView"]
        fn construct<'a>(string: &QAnyStringView) -> QAnyStringView<'a>;

        #[doc(hidden)]
        #[rust_name = "QAnyStringView_eq"]
        fn operatorEq(a: &QAnyStringView, b: &QAnyStringView) -> bool;

        #[doc(hidden)]
        #[rust_name = "QAnyStringView_len"]
        fn qanystringviewLen(string: &QAnyStringView) -> isize;
    }
}

/// The QAnyStringView class provides a unified view of a Latin-1, UTF-8, or UTF-16 string.
#[repr(C)]
pub struct QAnyStringView<'a> {
    /// QAnyStringView has two members, a pointer and a size_t
    _space: MaybeUninit<[usize; 1]>,
    _space2: MaybeUninit<[c_void; 1]>,

    /// Needed to keep the lifetime in check
    _phantom: PhantomData<&'a usize>,
}

impl<'a> Clone for QAnyStringView<'a> {
    /// Constructs a copy of other.
    ///
    /// This operation takes constant time, because QAnyStringView is a view-only string.
    fn clone(&self) -> QAnyStringView<'a> {
        ffi::QAnyStringView_init_from_QAnyStringView(self)
    }
}

impl Default for QAnyStringView<'_> {
    /// Constructs a null string. Null strings are also empty.
    fn default() -> Self {
        ffi::QAnyStringView_init_default()
    }
}

impl PartialEq for QAnyStringView<'_> {
    fn eq(&self, other: &Self) -> bool {
        ffi::QAnyStringView_eq(self, other)
    }
}

impl Eq for QAnyStringView<'_> {}

impl<'a> From<&'a str> for QAnyStringView<'a> {
    /// Constructs a QAnyStringView from a Rust string
    fn from(str: &str) -> Self {
        ffi::QAnyStringView_init_from_rust_string(str)
    }
}

impl From<&QString> for QAnyStringView<'_> {
    /// Constructs a QAnyStringView from a QString
    fn from(string: &QString) -> Self {
        ffi::QAnyStringView_init_from_qstring(string)
    }
}

impl QAnyStringView<'_> {
    /// Returns the number of characters in this string.
    pub fn len(&self) -> isize {
        ffi::QAnyStringView_len(self)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QAnyStringView<'_> {
    type Id = type_id!("QAnyStringView");
    type Kind = cxx::kind::Trivial;
}
