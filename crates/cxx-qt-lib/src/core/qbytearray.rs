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

        /// Clears the contents of the byte array and makes it null.
        fn clear(self: &mut QByteArray);
        /// Returns true if the byte array has size 0; otherwise returns false.
        #[rust_name = "is_empty"]
        fn isEmpty(self: &QByteArray) -> bool;
        /// Returns true if this byte array is lowercase, that is, if it's identical to its toLower() folding.
        #[rust_name = "is_lower"]
        fn isLower(self: &QByteArray) -> bool;
        /// Returns true if this byte array is null; otherwise returns false.
        #[rust_name = "is_null"]
        fn isNull(self: &QByteArray) -> bool;
        /// Returns true if this byte array is uppercase, that is, if it's identical to its toUpper() folding.
        #[rust_name = "is_upper"]
        fn isUpper(self: &QByteArray) -> bool;
        /// Releases any memory not required to store the array's data.
        fn squeeze(self: &mut QByteArray);
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
        #[rust_name = "qbytearray_eq"]
        fn operatorEq(a: &QByteArray, b: &QByteArray) -> bool;

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
        #[rust_name = "qbytearray_as_mut_slice"]
        fn qbytearrayAsMutSlice(bytearray: &mut QByteArray) -> &mut [u8];
        #[doc(hidden)]
        #[rust_name = "qbytearray_as_slice"]
        fn qbytearrayAsSlice(bytearray: &QByteArray) -> &[u8];

        #[doc(hidden)]
        #[rust_name = "qbytearray_append"]
        fn qbytearrayAppend(bytearray: &mut QByteArray, ch: u8);
        #[doc(hidden)]
        #[rust_name = "qbytearray_fill"]
        fn qbytearrayFill(bytearray: &mut QByteArray, ch: u8, size: isize);
        #[doc(hidden)]
        #[rust_name = "qbytearray_insert"]
        fn qbytearrayInsert(bytearray: &mut QByteArray, pos: isize, ch: u8);
        #[doc(hidden)]
        #[rust_name = "qbytearray_len"]
        fn qbytearrayLen(bytearray: &QByteArray) -> isize;
        #[doc(hidden)]
        #[rust_name = "qbytearray_prepend"]
        fn qbytearrayPrepend(bytearray: &mut QByteArray, ch: u8);
        #[doc(hidden)]
        #[rust_name = "qbytearray_remove"]
        fn qbytearrayRemove(bytearray: &mut QByteArray, pos: isize, len: isize);
        #[doc(hidden)]
        #[rust_name = "qbytearray_reserve"]
        fn qbytearrayReserve(bytearray: &mut QByteArray, size: isize);
        #[doc(hidden)]
        #[rust_name = "qbytearray_resize"]
        fn qbytearrayResize(bytearray: &mut QByteArray, size: isize);
        #[doc(hidden)]
        #[rust_name = "qbytearray_simplified"]
        fn qbytearraySimplified(bytearray: &QByteArray) -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "qbytearray_to_lower"]
        fn qbytearrayToLower(bytearray: &QByteArray) -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "qbytearray_to_upper"]
        fn qbytearrayToUpper(bytearray: &QByteArray) -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "qbytearray_trimmed"]
        fn qbytearrayTrimmed(bytearray: &QByteArray) -> QByteArray;
    }
}

/// The QByteArray class provides an array of bytes.
#[repr(C)]
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

impl std::cmp::PartialEq for QByteArray {
    fn eq(&self, other: &Self) -> bool {
        ffi::qbytearray_eq(self, other)
    }
}

impl std::cmp::Eq for QByteArray {}

impl std::fmt::Display for QByteArray {
    /// Convert the QByteArray to a Rust string
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Ok(string) = String::from_utf8(self.into()) {
            write!(f, "{string}")
        } else {
            write!(f, "{:?}", self.as_slice())
        }
    }
}

impl std::fmt::Debug for QByteArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
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

#[cfg(feature = "bytes")]
impl From<&bytes::Bytes> for QByteArray {
    /// Convert `bytes::Bytes` to a QByteArray. This makes a deep copy of the data.
    fn from(value: &bytes::Bytes) -> Self {
        Self::from(value.as_ref())
    }
}

#[cfg(feature = "bytes")]
impl From<&QByteArray> for bytes::Bytes {
    /// Convert QByteArray to a `bytes::Bytes`. This makes a deep copy of the data.
    fn from(value: &QByteArray) -> Self {
        Self::copy_from_slice(value.as_ref())
    }
}

impl QByteArray {
    /// Inserts value at the end of the list.
    pub fn append(&mut self, ch: u8) {
        ffi::qbytearray_append(self, ch);
    }

    /// Construct a mutable slice of u8 from a QByteArray
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        ffi::qbytearray_as_mut_slice(self)
    }

    /// Construct a slice of u8 from a QByteArray
    pub fn as_slice(&self) -> &[u8] {
        ffi::qbytearray_as_slice(self)
    }

    /// Sets every byte in the byte array to ch.
    /// If size is different from -1 (the default),
    /// the byte array is resized to size size beforehand.
    pub fn fill(&mut self, ch: u8, size: isize) {
        ffi::qbytearray_fill(self, ch, size)
    }

    /// Construct a QByteArray from a `bytes::Bytes` without a deep copy
    ///
    /// # Safety
    ///
    /// The caller must ensure that the original `bytes::Bytes` outlives the QByteArray
    /// and that the QByteArray is not modified
    #[cfg(feature = "bytes")]
    pub unsafe fn from_raw_bytes(bytes: &bytes::Bytes) -> Self {
        Self::from_raw_data(bytes.as_ref())
    }

    /// Construct a QByteArray from a `&[u8]` without a deep copy
    ///
    /// # Safety
    ///
    /// The caller must ensure that the original slice outlives the QByteArray
    /// and that the QByteArray is not modified
    pub unsafe fn from_raw_data(bytes: &[u8]) -> Self {
        ffi::qbytearray_from_raw_data(bytes)
    }

    /// Inserts item value into the list at the given position.
    pub fn insert(&mut self, pos: isize, ch: u8) {
        ffi::qbytearray_insert(self, pos, ch);
    }

    /// Returns the number of items in the QByteArray.
    pub fn len(&self) -> isize {
        ffi::qbytearray_len(self)
    }

    /// Inserts value at the start of the list.
    pub fn prepend(&mut self, ch: u8) {
        ffi::qbytearray_prepend(self, ch);
    }

    /// Removes len bytes from the array, starting at index position pos.
    pub fn remove(&mut self, pos: isize, len: isize) {
        ffi::qbytearray_remove(self, pos, len);
    }

    /// Reserve the specified capacity to prevent repeated allocations
    /// when the maximum size is known.
    pub fn reserve(&mut self, size: isize) {
        ffi::qbytearray_reserve(self, size);
    }

    /// Sets the size of the byte array to size bytes.
    ///
    /// If size is greater than the current size, the byte array is extended to make it size bytes with the extra bytes added to the end. The new bytes are uninitialized.
    ///
    /// If size is less than the current size, bytes beyond position size are excluded from the byte array.
    pub fn resize(&mut self, size: isize) {
        ffi::qbytearray_resize(self, size);
    }

    /// Returns a copy of this byte array that has spacing characters removed from the start and end,
    /// and in which each sequence of internal spacing characters is replaced with a single space.
    pub fn simplified(&self) -> Self {
        ffi::qbytearray_simplified(self)
    }

    /// Returns a copy of the byte array in which each ASCII uppercase letter converted to lowercase.
    pub fn to_lower(&self) -> Self {
        ffi::qbytearray_to_lower(self)
    }

    /// Returns a copy of the byte array in which each ASCII lowercase letter converted to uppercase.
    pub fn to_upper(&self) -> Self {
        ffi::qbytearray_to_upper(self)
    }

    /// Returns a copy of this byte array with spacing characters removed from the start and end.
    pub fn trimmed(&self) -> Self {
        ffi::qbytearray_trimmed(self)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QByteArray {
    type Id = type_id!("QByteArray");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "bytes")]
    use super::*;

    #[cfg(feature = "bytes")]
    #[test]
    fn test_bytes() {
        let bytes = bytes::Bytes::from("KDAB");
        let qbytearray = QByteArray::from(&bytes);
        assert_eq!(bytes.as_ref(), qbytearray.as_ref());

        let bytes_bytes = bytes::Bytes::from(&qbytearray);
        assert_eq!(bytes, bytes_bytes)
    }
}
