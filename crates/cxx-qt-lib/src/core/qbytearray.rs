// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::fmt;
use std::mem::MaybeUninit;
use std::str;

use crate::{unsafe_impl_qflag, QFlags};

#[cxx::bridge]
mod ffi {
    /// This enum contains the options available for encoding and decoding Base64. Base64 is defined by [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648).
    ///
    /// An empty `QFlags<QByteArrayBase64Option>` will use the regular Base64 alphabet, called simply "base64".
    #[namespace = "rust::cxxqtlib1"]
    #[repr(u32)]
    enum QByteArrayBase64Option {
        /// An alternate alphabet, called "base64url", which replaces two characters in the alphabet to be more friendly to URLs.
        Base64UrlEncoding = 1,
        /// Omits adding the padding equal signs at the end of the encoded data.
        OmitTrailingEquals = 2,
    }

    #[namespace = "rust::cxxqtlib1"]
    #[repr(i32)]
    enum QByteArrayBase64DecodingStatus {
        Ok,
        IllegalInputLength,
        IllegalCharacter,
        IllegalPadding,
    }

    #[namespace = "rust::cxxqtlib1"]
    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");

        type QByteArrayBase64DecodingStatus;
        type QByteArrayFromBase64Result = super::QByteArrayFromBase64Result;
        type QByteArrayBase64Option;
        type QByteArrayBase64Options = super::QByteArrayBase64Options;
    }

    unsafe extern "C++" {
        type QByteArray = super::QByteArray;

        /// Clears the contents of the byte array and makes it null.
        fn clear(&mut self);

        /// Returns `true` if the byte array has size 0; otherwise returns `false`.
        #[rust_name = "is_empty"]
        fn isEmpty(&self) -> bool;
        /// Returns `true` if this byte array is lowercase, that is, if it's identical to its [`to_lower`](Self::to_lower) folding.
        #[rust_name = "is_lower"]
        fn isLower(&self) -> bool;
        /// Returns `true` if this byte array is null; otherwise returns `false`.
        #[rust_name = "is_null"]
        fn isNull(&self) -> bool;
        /// Returns `true` if this byte array is uppercase, that is, if it's identical to its [`to_upper`](Self::to_upper) folding.
        #[rust_name = "is_upper"]
        fn isUpper(&self) -> bool;
        /// Releases any memory not required to store the array's data.
        fn squeeze(&mut self);
        /// Returns a copy of the byte array, encoded using the options `options`.
        #[rust_name = "to_base64"]
        fn toBase64(&self, options: QByteArrayBase64Options) -> QByteArray;
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
        #[rust_name = "qbytearrray_from_base64_encoding"]
        fn qbytearrayFromBase64Encoding(
            base64: &QByteArray,
            options: QByteArrayBase64Options,
        ) -> QByteArrayFromBase64Result;
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

use ffi::QByteArrayBase64DecodingStatus;
pub use ffi::QByteArrayBase64Option;

/// [`QFlags`] of [`QByteArrayBase64Option`].
pub type QByteArrayBase64Options = QFlags<QByteArrayBase64Option>;
unsafe_impl_qflag!(
    QByteArrayBase64Option,
    "rust::cxxqtlib1::QByteArrayBase64Options",
    u32
);

/// The `QByteArray` class provides an array of bytes.
///
/// Qt Documentation: [QByteArray](https://doc.qt.io/qt/qbytearray.html#details)
#[repr(C)]
pub struct QByteArray {
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 QByteArray has one pointer as a member
    /// Qt6 QByteArray has one member, which contains two pointers and a size_t
    #[cfg(cxxqt_qt_version_major = "5")]
    _space: MaybeUninit<usize>,
    #[cfg(cxxqt_qt_version_major = "6")]
    _space: MaybeUninit<[usize; 3]>,
}

impl AsRef<[u8]> for QByteArray {
    /// Construct a slice of `u8` from a `QByteArray`.
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Clone for QByteArray {
    /// Constructs a copy of `self`.
    ///
    /// This operation takes constant time, because `QByteArray` is implicitly shared similar to a [`Cow`](std::borrow::Cow).
    /// This makes returning a `QByteArray` from a function very fast.
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

impl fmt::Display for QByteArray {
    /// Convert the `QByteArray` to a Rust string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let slice = self.as_slice();
        if let Ok(string) = str::from_utf8(slice) {
            string.fmt(f)
        } else {
            fmt::Debug::fmt(slice, f)
        }
    }
}

impl fmt::Debug for QByteArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Drop for QByteArray {
    /// Destroys the byte array.
    fn drop(&mut self) {
        ffi::qbytearray_drop(self);
    }
}

impl From<&str> for QByteArray {
    /// Constructs a `QByteArray` from a Rust string slice. This makes a deep copy of the data.
    fn from(str: &str) -> Self {
        ffi::qbytearray_from_slice_u8(str.as_bytes())
    }
}

impl From<&String> for QByteArray {
    /// Constructs a `QByteArray` from a Rust string. This makes a deep copy of the data.
    fn from(str: &String) -> Self {
        ffi::qbytearray_from_slice_u8(str.as_bytes())
    }
}

impl From<&[u8]> for QByteArray {
    /// Constructs a `QByteArray` from a `&[u8]`. This makes a deep copy of the data.
    fn from(bytes: &[u8]) -> Self {
        ffi::qbytearray_from_slice_u8(bytes)
    }
}

impl From<&QByteArray> for Vec<u8> {
    /// Convert the `QByteArray` to a `Vec<u8>`. This makes a deep copy of the data.
    fn from(bytearray: &QByteArray) -> Self {
        ffi::qbytearray_to_vec_u8(bytearray)
    }
}

impl<const N: usize> From<&[u8; N]> for QByteArray {
    #[inline]
    fn from(bytes: &[u8; N]) -> Self {
        ffi::qbytearray_from_slice_u8(bytes)
    }
}

#[cfg(feature = "bytes")]
impl From<&bytes::Bytes> for QByteArray {
    /// Convert `bytes::Bytes` to a `QByteArray`. This makes a deep copy of the data.
    fn from(value: &bytes::Bytes) -> Self {
        Self::from(value.as_ref())
    }
}

#[cfg(feature = "bytes")]
impl From<&QByteArray> for bytes::Bytes {
    /// Convert `QByteArray` to a `bytes::Bytes`. This makes a deep copy of the data.
    fn from(value: &QByteArray) -> Self {
        Self::copy_from_slice(value.as_ref())
    }
}

impl QByteArray {
    /// Inserts `value` at the end of the list.
    pub fn append(&mut self, ch: u8) {
        ffi::qbytearray_append(self, ch);
    }

    /// Extracts a mutable slice of the entire vector.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        ffi::qbytearray_as_mut_slice(self)
    }

    /// Extracts a slice containing the entire byte array.
    pub fn as_slice(&self) -> &[u8] {
        ffi::qbytearray_as_slice(self)
    }

    /// Sets every byte in the byte array to `ch`.
    /// If `size` is different from -1,
    /// the byte array is resized to size `size` beforehand.
    pub fn fill(&mut self, ch: u8, size: isize) {
        ffi::qbytearray_fill(self, ch, size);
    }

    /// Decodes the Base64 array `base64`, using the options defined by `options`.
    pub fn from_base64_encoding(
        base64: &Self,
        options: QByteArrayBase64Options,
    ) -> Result<Self, QByteArrayFromBase64Error> {
        let result = ffi::qbytearrray_from_base64_encoding(base64, options);
        match result.decoding_status {
            QByteArrayBase64DecodingStatus::IllegalInputLength => {
                Err(QByteArrayFromBase64Error::IllegalInputLength)
            }
            QByteArrayBase64DecodingStatus::IllegalCharacter => {
                Err(QByteArrayFromBase64Error::IllegalCharacter)
            }
            QByteArrayBase64DecodingStatus::IllegalPadding => {
                Err(QByteArrayFromBase64Error::IllegalPadding)
            }
            _ => Ok(result.decoded),
        }
    }

    /// Construct a `QByteArray` from a `bytes::Bytes` without a deep copy
    ///
    /// # Safety
    ///
    /// The caller must ensure that the original `bytes::Bytes` outlives the `QByteArray`
    /// and that the `QByteArray` is not modified
    #[cfg(feature = "bytes")]
    pub unsafe fn from_raw_bytes(bytes: &bytes::Bytes) -> Self {
        Self::from_raw_data(bytes.as_ref())
    }

    /// Construct a `QByteArray` from a `&[u8]` without a deep copy
    ///
    /// # Safety
    ///
    /// The caller must ensure that the original slice outlives the `QByteArray`
    /// and that the `QByteArray` is not modified
    pub unsafe fn from_raw_data(bytes: &[u8]) -> Self {
        ffi::qbytearray_from_raw_data(bytes)
    }

    /// Inserts byte `ch` at index position `pos` in the byte array.
    ///
    /// This array grows to accommodate the insertion. If `pos` is beyond the end of the array, the array is first extended with space characters to reach this `pos`.
    pub fn insert(&mut self, pos: isize, ch: u8) {
        ffi::qbytearray_insert(self, pos, ch);
    }

    /// Returns the number of bytes in this byte array.
    pub fn len(&self) -> isize {
        ffi::qbytearray_len(self)
    }

    /// Prepends the byte `ch` to this byte array.
    pub fn prepend(&mut self, ch: u8) {
        ffi::qbytearray_prepend(self, ch);
    }

    /// Removes `len` bytes from the array, starting at index position `pos`.
    ///
    /// If `pos` is out of range, nothing happens. If `pos` is valid, but `pos + len` is larger than the size of the array, the array is truncated at position `pos`.
    pub fn remove(&mut self, pos: isize, len: isize) {
        ffi::qbytearray_remove(self, pos, len);
    }

    /// Attempts to allocate memory for at least `size` bytes.
    ///
    /// If you know in advance how large the byte array will be, you can call this function, and if you call [`resize`](Self::resize) often you are likely to get better performance.
    ///
    /// If in doubt about how much space shall be needed, it is usually better to use an upper bound as `size`, or a high estimate of the most likely size, if a strict upper bound would be much bigger than this. If `size` is an underestimate, the array will grow as needed once the reserved size is exceeded, which may lead to a larger allocation than your best overestimate would have and will slow the operation that triggers it.
    ///
    /// The sole purpose of this function is to provide a means of fine tuning `QByteArray`'s memory usage. In general, you will rarely ever need to call this function.
    pub fn reserve(&mut self, size: isize) {
        ffi::qbytearray_reserve(self, size);
    }

    /// Sets the size of the byte array to `size` bytes.
    ///
    /// If `size` is greater than the current size, the byte array is extended to make it `size` bytes with the extra bytes added to the end. **The new bytes are uninitialized.**
    ///
    /// If `size` is less than the current size, bytes beyond position `size` are excluded from the byte array.
    ///
    /// **Note:** While `resize` will grow the capacity if needed, it never shrinks capacity.
    pub fn resize(&mut self, size: isize) {
        ffi::qbytearray_resize(self, size);
    }

    /// Returns a copy of this byte array that has spacing characters removed from the start and end,
    /// and in which each sequence of internal spacing characters is replaced with a single space.
    ///
    /// The spacing characters are the ASCII characters tabulation `'\t'`, line feed `'\n'`, carriage return `'\r'`, vertical tabulation `'\x08'` (`'\v'` in C), form feed `'\x0C'` (`'\f'` in C), and space `' '`.
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
    ///
    /// The spacing characters are the ASCII characters tabulation `'\t'`, line feed `'\n'`, carriage return `'\r'`, vertical tabulation `'\x08'` (`'\v'` in C), form feed `'\x0C'` (`'\f'` in C), and space `' '`.
    pub fn trimmed(&self) -> Self {
        ffi::qbytearray_trimmed(self)
    }
}

#[repr(C)]
struct QByteArrayFromBase64Result {
    decoded: QByteArray,
    decoding_status: QByteArrayBase64DecodingStatus,
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QByteArrayFromBase64Result {
    type Id = type_id!("rust::cxxqtlib1::QByteArrayFromBase64Result");
    type Kind = cxx::kind::Trivial;
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum QByteArrayFromBase64Error {
    IllegalInputLength = 1,
    IllegalCharacter,
    IllegalPadding,
}

impl fmt::Display for QByteArrayFromBase64Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::IllegalInputLength => "illegal input length",
            Self::IllegalCharacter => "illegal character",
            Self::IllegalPadding => "illegal padding",
        })
    }
}

impl std::error::Error for QByteArrayFromBase64Error {}

#[cfg(feature = "serde")]
impl serde::Serialize for QByteArray {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(self.as_slice())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QByteArray {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{Error as DeError, SeqAccess, Visitor};

        struct BytesVisitor;

        impl<'de> Visitor<'de> for BytesVisitor {
            type Value = QByteArray;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an array of bytes")
            }

            fn visit_bytes<E: DeError>(self, v: &[u8]) -> Result<Self::Value, E> {
                Ok(Self::Value::from(v))
            }

            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Self::Value::from(v))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut values = Self::Value::default();
                if let Some(size_hint) = seq.size_hint() {
                    if size_hint != 0 && size_hint <= isize::MAX as usize {
                        values.reserve(size_hint as isize);
                    }
                }
                while let Some(value) = seq.next_element()? {
                    values.append(value);
                }
                Ok(values)
            }
        }

        let visitor = BytesVisitor;
        deserializer.deserialize_byte_buf(visitor)
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
    use super::*;

    #[cfg(feature = "serde")]
    #[test]
    fn qbytearray_serde() {
        let qbytearray = QByteArray::from("KDAB");
        assert_eq!(crate::serde_impl::roundtrip(&qbytearray), qbytearray);
    }

    #[test]
    fn qbytearray_base64() {
        let qbytearray = QByteArray::from("KDAB");
        let options = QByteArrayBase64Options::default();
        let encoded = qbytearray.to_base64(options);
        let decoded = QByteArray::from_base64_encoding(&encoded, options);
        assert_eq!(decoded, Ok(qbytearray));
    }

    #[test]
    fn qbytearray_base64_url() {
        let qbytearray = QByteArray::from("KDAB");
        let options = QByteArrayBase64Option::Base64UrlEncoding.into();
        let encoded = qbytearray.to_base64(options);
        let decoded = QByteArray::from_base64_encoding(&encoded, options);
        assert_eq!(decoded, Ok(qbytearray));
    }

    #[cfg(feature = "bytes")]
    #[test]
    fn test_bytes() {
        let bytes = bytes::Bytes::from("KDAB");
        let qbytearray = QByteArray::from(&bytes);
        assert_eq!(bytes.as_ref(), qbytearray.as_ref());

        let bytes_bytes = bytes::Bytes::from(&qbytearray);
        assert_eq!(bytes, bytes_bytes);
    }

    #[test]
    fn test_display_fmt() {
        let qbytearray = QByteArray::from("KDAB");
        assert_eq!(format!("{qbytearray:-<8}"), "KDAB----");
    }
}
