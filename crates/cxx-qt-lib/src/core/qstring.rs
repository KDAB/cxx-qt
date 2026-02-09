// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::cmp::Ordering;
use std::fmt::{self, Write};
use std::mem::MaybeUninit;

use crate::{CaseSensitivity, QByteArray, QStringList, SplitBehaviorFlags};

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type CaseSensitivity = crate::CaseSensitivity;
        type SplitBehaviorFlags = crate::SplitBehaviorFlags;
    }

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = super::QString;

        /// Appends the string `str` onto the end of this string.
        fn append<'a>(&'a mut self, str: &QString) -> &'a mut QString;

        /// Clears the contents of the string and makes it null.
        fn clear(&mut self);

        // We wrap this method to provide an enum so hide it from docs
        #[doc(hidden)]
        #[rust_name = "compare_i32"]
        fn compare(&self, other: &QString, cs: CaseSensitivity) -> i32;

        /// Returns `true` if this string contains an occurrence of the string `str`; otherwise returns `false`.
        ///
        /// If `cs` is [`CaseSensitivity::CaseSensitive`], the search is case-sensitive; otherwise the search is case-insensitive.
        fn contains(&self, str: &QString, cs: CaseSensitivity) -> bool;

        /// Returns `true` if the string ends with `s`; otherwise returns `false`.
        ///
        /// If `cs` is [`CaseSensitivity::CaseSensitive`], the search is case-sensitive; otherwise the search is case-insensitive.
        #[rust_name = "ends_with"]
        fn endsWith(&self, s: &QString, cs: CaseSensitivity) -> bool;

        /// Returns `true` if the string has no characters; otherwise returns `false`.
        #[rust_name = "is_empty"]
        fn isEmpty(&self) -> bool;

        /// Returns `true` if the string is lowercase, that is, it's identical to its [`to_lower`](Self::to_lower) folding.
        #[rust_name = "is_lower"]
        fn isLower(&self) -> bool;

        /// Returns `true` if this string is null; otherwise returns `false`.
        #[rust_name = "is_null"]
        fn isNull(&self) -> bool;

        /// Returns `true` if the string is read right to left.
        #[rust_name = "is_right_to_left"]
        fn isRightToLeft(&self) -> bool;

        /// Returns `true` if the string is uppercase, that is, it's identical to its [`to_upper`](Self::to_upper) folding.
        #[rust_name = "is_upper"]
        fn isUpper(&self) -> bool;

        /// Returns `true` if the string contains valid UTF-16 encoded data, or `false` otherwise.
        #[rust_name = "is_valid_utf16"]
        fn isValidUtf16(&self) -> bool;

        /// Prepends the string `str` to the beginning of this string and returns a mutable reference to this string.
        fn prepend<'a>(&'a mut self, str: &QString) -> &'a mut QString;

        /// Removes every occurrence of the given `str` string in this string, and returns a mutable reference to this string.
        ///
        /// If `cs` is [`CaseSensitivity::CaseSensitive`], the search is case-sensitive; otherwise the search is case-insensitive.
        fn remove<'a>(&'a mut self, str: &QString, cs: CaseSensitivity) -> &'a mut QString;

        /// Removes the first character in this string. If the string is empty, this function does nothing.
        ///
        /// This function was introduced in Qt 6.5.
        #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_5))]
        #[rust_name = "remove_first"]
        fn removeFirst(&mut self) -> &mut QString;

        /// Removes the last character in this string. If the string is empty, this function does nothing.
        ///
        /// This function was introduced in Qt 6.5.
        #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_5))]
        #[rust_name = "remove_last"]
        fn removeLast(&mut self) -> &mut QString;

        /// Replaces every occurrence of the string `before` with the string `after` and returns a mutable reference to this string.
        ///
        /// If `cs` is [`CaseSensitivity::CaseSensitive`], the search is case-sensitive; otherwise the search is case-insensitive.
        fn replace<'a>(
            &'a mut self,
            before: &QString,
            after: &QString,
            cs: CaseSensitivity,
        ) -> &'a mut QString;

        /// Returns `true` if the string starts with `s`; otherwise returns `false`.
        #[rust_name = "starts_with"]
        fn startsWith(&self, s: &QString, cs: CaseSensitivity) -> bool;

        /// Converts a plain text string to an HTML string with HTML metacharacters `<`, `>`, `&`, and `"` replaced by HTML entities.
        #[rust_name = "to_html_escaped"]
        fn toHtmlEscaped(&self) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qstring_drop"]
        fn drop(string: &mut QString);

        #[doc(hidden)]
        #[rust_name = "qstring_init_default"]
        fn construct() -> QString;
        #[doc(hidden)]
        #[rust_name = "qstring_init_from_rust_string"]
        fn qstringInitFromRustString(string: &str) -> QString;
        #[doc(hidden)]
        #[rust_name = "qstring_init_from_qstring"]
        fn construct(string: &QString) -> QString;

        #[doc(hidden)]
        #[rust_name = "qstring_eq"]
        fn operatorEq(a: &QString, b: &QString) -> bool;
        #[doc(hidden)]
        #[rust_name = "qstring_cmp"]
        fn operatorCmp(a: &QString, b: &QString) -> i8;

        #[doc(hidden)]
        #[rust_name = "qstring_as_slice"]
        fn qstringAsSlice(string: &QString) -> &[u16];

        #[doc(hidden)]
        #[rust_name = "qstring_arg"]
        fn qstringArg(string: &QString, a: &QString) -> QString;
        #[doc(hidden)]
        #[rust_name = "qstring_index_of"]
        fn qstringIndexOf(
            string: &QString,
            str: &QString,
            from: isize,
            cs: CaseSensitivity,
        ) -> isize;
        #[doc(hidden)]
        #[rust_name = "qstring_insert"]
        fn qstringInsert<'a>(string: &'a mut QString, pos: isize, str: &QString)
            -> &'a mut QString;
        #[doc(hidden)]
        #[rust_name = "qstring_left"]
        fn qstringLeft(string: &QString, n: isize) -> QString;
        #[doc(hidden)]
        #[rust_name = "qstring_len"]
        fn qstringLen(string: &QString) -> isize;
        #[doc(hidden)]
        #[rust_name = "qstring_mid"]
        fn qstringMid(string: &QString, position: isize, n: isize) -> QString;
        #[doc(hidden)]
        #[rust_name = "qstring_right"]
        fn qstringRight(string: &QString, n: isize) -> QString;
        #[doc(hidden)]
        #[rust_name = "qstring_simplified"]
        fn qstringSimplified(string: &QString) -> QString;
        #[doc(hidden)]
        #[rust_name = "qstring_split"]
        fn qstringSplit(
            string: &QString,
            sep: &QString,
            behavior: SplitBehaviorFlags,
            cs: CaseSensitivity,
        ) -> QStringList;
        #[doc(hidden)]
        #[rust_name = "qstring_to_latin1"]
        fn qstringToLatin1(string: &QString) -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "qstring_to_local8bit"]
        fn qstringToLocal8Bit(string: &QString) -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "qstring_to_lower"]
        fn qstringToLower(string: &QString) -> QString;
        #[doc(hidden)]
        #[rust_name = "qstring_to_upper"]
        fn qstringToUpper(string: &QString) -> QString;
        #[doc(hidden)]
        #[rust_name = "qstring_to_utf8"]
        fn qstringToUtf8(string: &QString) -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "qstring_trimmed"]
        fn qstringTrimmed(string: &QString) -> QString;
    }
}

/// The `QString` class provides a Unicode character string.
///
/// Note that `QString` is encoded in UTF-16, whereas Rust's [`String`] is encoded in UTF-8.
///
/// Qt Documentation: [QString](https://doc.qt.io/qt/qstring.html#details)
#[repr(C)]
pub struct QString {
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 QString has one pointer as a member
    /// Qt6 QString has one member, which contains two pointers and a size_t
    #[cfg(cxxqt_qt_version_major = "5")]
    _space: MaybeUninit<usize>,
    #[cfg(cxxqt_qt_version_major = "6")]
    _space: MaybeUninit<[usize; 3]>,
}

impl Clone for QString {
    /// Constructs a copy of this string.
    ///
    /// This operation takes constant time, because `QString` is implicitly shared.
    /// This makes returning a `QString` from a function very fast.
    /// If a shared instance is modified, it will be copied (copy-on-write), and that takes linear time.
    fn clone(&self) -> Self {
        ffi::qstring_init_from_qstring(self)
    }
}

impl Default for QString {
    /// Constructs a null string. Null strings are also empty.
    fn default() -> Self {
        ffi::qstring_init_default()
    }
}

impl PartialEq for QString {
    fn eq(&self, other: &Self) -> bool {
        ffi::qstring_eq(self, other)
    }
}

impl Eq for QString {}

impl PartialOrd for QString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QString {
    fn cmp(&self, other: &Self) -> Ordering {
        ffi::qstring_cmp(self, other).cmp(&0)
    }
}

impl fmt::Display for QString {
    /// Format the `QString` as a Rust string.
    ///
    /// Note that this converts from UTF-16 to UTF-8.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.width().is_some() || f.precision().is_some() {
            return f.pad(&String::from(self));
        }
        for c in char::decode_utf16(self.as_slice().iter().copied()) {
            f.write_char(c.unwrap_or(char::REPLACEMENT_CHARACTER))?;
        }
        Ok(())
    }
}

impl fmt::Debug for QString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        String::from(self).fmt(f)
    }
}

impl std::ops::Add for QString {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut res = ffi::qstring_init_from_qstring(&self);
        res.append(&other);
        res
    }
}

impl Drop for QString {
    /// Destroys the string.
    fn drop(&mut self) {
        ffi::qstring_drop(self)
    }
}

impl From<&str> for QString {
    /// Constructs a `QString` from a string slice.
    ///
    /// Note that this converts from UTF-8 to UTF-16.
    fn from(str: &str) -> Self {
        ffi::qstring_init_from_rust_string(str)
    }
}

impl From<&String> for QString {
    /// Constructs a `QString` from a Rust `String` reference.
    ///
    /// Note that this converts from UTF-8 to UTF-16.
    fn from(str: &String) -> Self {
        ffi::qstring_init_from_rust_string(str)
    }
}

impl From<String> for QString {
    /// Constructs a `QString` from a Rust `String`.
    ///
    /// Note that this converts from UTF-8 to UTF-16.
    fn from(str: String) -> Self {
        ffi::qstring_init_from_rust_string(&str)
    }
}

impl From<&QString> for String {
    /// Constructs a Rust `String` from a `QString` reference.
    ///
    /// Note that this converts from UTF-16 to UTF-8.
    fn from(qstring: &QString) -> Self {
        String::from_utf16_lossy(qstring.as_slice())
    }
}

impl From<QString> for String {
    /// Constructs a Rust `String` from a `QString`.
    ///
    /// Note that this converts from UTF-16 to UTF-8.
    fn from(qstring: QString) -> Self {
        String::from_utf16_lossy(qstring.as_slice())
    }
}

impl QString {
    /// Returns a copy of this string with the lowest numbered place marker replaced by string `a`, i.e., %1, %2, ..., %99.
    ///
    /// If there is no unreplaced place-marker remaining, a warning message is printed and the result is undefined. Place-marker numbers must be in the range 1 to 99.
    pub fn arg(&self, a: &QString) -> Self {
        ffi::qstring_arg(self, a)
    }

    /// Extracts a slice containing the entire UTF-16 array.
    pub fn as_slice(&self) -> &[u16] {
        ffi::qstring_as_slice(self)
    }

    /// Lexically compares this string with the `other` string.
    ///
    /// If `cs` is [`CaseSensitivity::CaseSensitive`], the comparison is case-sensitive; otherwise the comparison is case-insensitive.
    ///
    /// Case sensitive comparison is based exclusively on the numeric Unicode values of the characters and is very fast, but is not what a human would expect.
    pub fn compare(&self, other: &QString, cs: CaseSensitivity) -> Ordering {
        self.compare_i32(other, cs).cmp(&0)
    }

    /// Returns the index position of the first occurrence of the string `str` in this string,
    /// searching forward from index position `from`. Returns -1 if `str` is not found.
    ///
    /// If `cs` is [`CaseSensitivity::CaseSensitive`], the search is case-sensitive; otherwise the comparison is case-insensitive.
    pub fn index_of(&self, str: &QString, from: isize, cs: CaseSensitivity) -> isize {
        ffi::qstring_index_of(self, str, from, cs)
    }

    /// Inserts the string `str` at the given index `position` and returns a mutable reference to this string.
    pub fn insert<'a>(&'a mut self, position: isize, str: &Self) -> &'a mut Self {
        ffi::qstring_insert(self, position, str)
    }

    /// Returns a substring that contains the `n` leftmost characters of the string.
    pub fn left(&self, n: isize) -> Self {
        ffi::qstring_left(self, n)
    }

    /// Returns the number of characters in this string.
    pub fn len(&self) -> isize {
        ffi::qstring_len(self)
    }

    /// Returns a string that contains `n` characters of this string, starting at the specified `position` index.
    pub fn mid(&self, position: isize, n: isize) -> Self {
        ffi::qstring_mid(self, position, n)
    }

    /// Returns a substring that contains the `n` rightmost characters of the string.
    pub fn right(&self, n: isize) -> Self {
        ffi::qstring_right(self, n)
    }

    /// Returns a string that has whitespace removed from the start and the end,
    /// and that has each sequence of internal whitespace replaced with a single space.
    ///
    /// Whitespace characters are the ASCII characters tabulation `'\t'`, line feed `'\n'`, carriage return `'\r'`, vertical tabulation `'\x08'` (`'\v'` in C), form feed `'\x0C'` (`'\f'` in C), and space `' '`.
    pub fn simplified(&self) -> Self {
        ffi::qstring_simplified(self)
    }

    /// Splits the string into substrings wherever `sep` occurs, and returns the list of those strings.
    /// If `sep` does not match anywhere in the string, this function returns a single-element list containing this string.
    ///
    /// `cs` specifies whether `sep` should be matched case sensitively or case insensitively.
    ///
    /// If `behavior` is [`SplitBehaviorFlags::SkipEmptyParts`], empty entries don't appear in the result.
    pub fn split(
        &self,
        sep: &QString,
        behavior: SplitBehaviorFlags,
        cs: CaseSensitivity,
    ) -> QStringList {
        ffi::qstring_split(self, sep, behavior, cs)
    }

    /// Returns a Latin-1 representation of the string as a `QByteArray`.
    ///
    /// The returned byte array is undefined if the string contains non-Latin1 characters. Those characters may be suppressed or replaced with a question mark.
    pub fn to_latin1(&self) -> QByteArray {
        ffi::qstring_to_latin1(self)
    }

    /// Returns the local 8-bit representation of the string as a `QByteArray`.
    ///
    /// If this string contains any characters that cannot be encoded in the local 8-bit encoding, the returned byte array is undefined. Those characters may be suppressed or replaced by another.
    pub fn to_local8bit(&self) -> QByteArray {
        ffi::qstring_to_local8bit(self)
    }

    /// Returns a lowercase copy of the string.
    pub fn to_lower(&self) -> Self {
        ffi::qstring_to_lower(self)
    }

    /// Returns an uppercase copy of the string.
    pub fn to_upper(&self) -> Self {
        ffi::qstring_to_upper(self)
    }

    /// Returns a UTF-8 representation of the string as a `QByteArray`.
    pub fn to_utf8(&self) -> QByteArray {
        ffi::qstring_to_utf8(self)
    }

    /// Returns a string that has whitespace removed from the start and the end.
    ///
    /// Whitespace characters are the ASCII characters tabulation `'\t'`, line feed `'\n'`, carriage return `'\r'`, vertical tabulation `'\x08'` (`'\v'` in C), form feed `'\x0C'` (`'\f'` in C), and space `' '`.
    pub fn trimmed(&self) -> Self {
        ffi::qstring_trimmed(self)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QString {
    type Id = type_id!("QString");
    type Kind = cxx::kind::Trivial;
}

#[cfg(feature = "serde")]
impl serde::Serialize for QString {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&String::from(self))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QString {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{Error as DeError, Unexpected, Visitor};

        struct StringVisitor;

        impl Visitor<'_> for StringVisitor {
            type Value = QString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Self::Value::from(v))
            }

            fn visit_bytes<E: DeError>(self, v: &[u8]) -> Result<Self::Value, E> {
                match std::str::from_utf8(v) {
                    Ok(s) => Ok(Self::Value::from(s)),
                    Err(_) => Err(E::invalid_value(Unexpected::Bytes(v), &self)),
                }
            }
        }

        let visitor = StringVisitor;
        deserializer.deserialize_string(visitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "serde")]
    #[test]
    fn qstring_serde() {
        let qstring = QString::from("KDAB");
        assert_eq!(crate::serde_impl::roundtrip(&qstring), qstring);
    }

    #[test]
    fn test_ordering() {
        let qstring_a = QString::from("a");
        let qstring_b = QString::from("b");

        assert!(qstring_a < qstring_b);
        assert_eq!(qstring_a.cmp(&qstring_b), Ordering::Less);
        assert_eq!(qstring_b.cmp(&qstring_a), Ordering::Greater);
        assert_eq!(qstring_a.cmp(&qstring_a), Ordering::Equal);

        assert_eq!(
            qstring_a.compare(&qstring_b, crate::CaseSensitivity::CaseInsensitive),
            Ordering::Less
        );
        assert_eq!(
            qstring_b.compare(&qstring_a, crate::CaseSensitivity::CaseInsensitive),
            Ordering::Greater
        );
        assert_eq!(
            qstring_a.compare(&qstring_a, crate::CaseSensitivity::CaseInsensitive),
            Ordering::Equal
        );
    }
}
