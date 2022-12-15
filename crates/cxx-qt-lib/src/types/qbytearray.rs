// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");

        type QByteArray = super::QByteArray;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qbytearray_drop"]
        fn drop(string: &mut QByteArray);

        #[doc(hidden)]
        #[rust_name = "qbytearray_default"]
        fn construct() -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "qbytearray_from_rust_string"]
        fn qbytearrayFromRustString(string: &str) -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "qbytearray_clone"]
        fn construct(string: &QByteArray) -> QByteArray;

        #[doc(hidden)]
        #[rust_name = "qbytearray_to_rust_string"]
        fn qbytearrayToRustString(string: &QByteArray) -> String;
    }
}

/// The QByteArray class provides an array of bytes.
pub struct QByteArray {
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 QByteArray has one pointer as a member
    /// Qt6 QByteArray has one member, which contains two pointers and a size_t
    #[cfg(qt_version_major = "5")]
    _space: MaybeUninit<usize>,
    #[cfg(qt_version_major = "6")]
    _space: MaybeUninit<[usize; 3]>,
}

impl Clone for QByteArray {
    /// Constructs a copy of other.
    ///
    /// This operation takes constant time, because QByteArray is implicitly shared.
    /// This makes returning a QByteArray from a function very fast.
    /// If a shared instance is modified, it will be copied (copy-on-write), and that takes linear time.
    fn clone(&self) -> Self {
        ffi::qbytearray_clone(self)
    }
}

impl Default for QByteArray {
    /// Constructs an empty byte array.
    fn default() -> Self {
        ffi::qbytearray_default()
    }
}

impl std::fmt::Display for QByteArray {
    /// Convert the QByteArray to a Rust string
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", <&QByteArray as Into<String>>::into(self))
    }
}

impl Drop for QByteArray {
    /// Destroys the byte array.
    fn drop(&mut self) {
        ffi::qbytearray_drop(self)
    }
}

impl From<&str> for QByteArray {
    /// Constructs a QByteArray from a Rust string
    fn from(str: &str) -> Self {
        ffi::qbytearray_from_rust_string(str)
    }
}

impl From<&String> for QByteArray {
    /// Constructs a QByteArray from a Rust string
    fn from(str: &String) -> Self {
        ffi::qbytearray_from_rust_string(str)
    }
}

impl From<&QByteArray> for String {
    /// Convert the QByteArray to a Rust string
    fn from(qbytearray: &QByteArray) -> Self {
        ffi::qbytearray_to_rust_string(qbytearray)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QByteArray {
    type Id = type_id!("QByteArray");
    type Kind = cxx::kind::Trivial;
}
