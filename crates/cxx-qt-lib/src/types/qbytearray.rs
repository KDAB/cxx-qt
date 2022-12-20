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
        fn drop(bytearray: &mut QByteArray);

        #[doc(hidden)]
        #[rust_name = "qbytearray_default"]
        fn construct() -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "qbytearray_clone"]
        fn construct(bytearray: &QByteArray) -> QByteArray;

        #[doc(hidden)]
        #[rust_name = "qbytearray_from_slice_u8"]
        fn qbytearrayFromSliceU8(slice: &[u8]) -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "qbytearray_to_vec_u8"]
        fn qbytearrayToVecU8(string: &QByteArray) -> Vec<u8>;

        #[doc(hidden)]
        #[rust_name = "qbytearray_from_raw_data"]
        fn qbytearrayFromRawData(slice: &[u8]) -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "qbytearray_as_slice"]
        fn qbytearrayAsSlice(bytearray: &QByteArray) -> &[u8];
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

impl AsRef<[u8]> for QByteArray {
    /// Construct a slice of u8 from a QByteArray
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Clone for QByteArray {
    /// Constructs a copy of other.
    ///
    /// This operation takes constant time, because QByteArray is implicitly shared similar to a [std::borrow::Cow].
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
        if let Ok(string) = String::from_utf8(self.into()) {
            write!(f, "{}", string)
        } else {
            write!(f, "{:?}", self.as_slice())
        }
    }
}

impl Drop for QByteArray {
    /// Destroys the byte array.
    fn drop(&mut self) {
        ffi::qbytearray_drop(self)
    }
}

impl From<&str> for QByteArray {
    /// Constructs a QByteArray from a Rust string slice. This makes a deep copy of the data.
    fn from(str: &str) -> Self {
        ffi::qbytearray_from_slice_u8(str.as_bytes())
    }
}

impl From<&String> for QByteArray {
    /// Constructs a QByteArray from a Rust string. This makes a deep copy of the data.
    fn from(str: &String) -> Self {
        ffi::qbytearray_from_slice_u8(str.as_bytes())
    }
}

impl From<&[u8]> for QByteArray {
    /// Constructs a QByteArray from a `&[u8]`. This makes a deep copy of the data.
    fn from(bytes: &[u8]) -> Self {
        ffi::qbytearray_from_slice_u8(bytes)
    }
}

impl From<&QByteArray> for Vec<u8> {
    /// Convert the QByteArray to a `Vec<u8>`. This makes a deep copy of the data.
    fn from(bytearray: &QByteArray) -> Self {
        ffi::qbytearray_to_vec_u8(bytearray)
    }
}

impl QByteArray {
    /// Construct a slice of u8 from a QByteArray
    pub fn as_slice(&self) -> &[u8] {
        ffi::qbytearray_as_slice(self)
    }

    /// Construct a QByteArray from a `&[u8]` without a deep copy
    ///
    /// # Safety
    ///
    /// The caller must ensure that the original slice outlives the QByteArray
    pub unsafe fn from_raw_data(bytes: &[u8]) -> QByteArray {
        ffi::qbytearray_from_raw_data(bytes)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QByteArray {
    type Id = type_id!("QByteArray");
    type Kind = cxx::kind::Trivial;
}
