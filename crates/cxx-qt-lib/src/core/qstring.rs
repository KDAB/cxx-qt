// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::cmp::Ordering;
use std::fmt;
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type CaseSensitivity = crate::CaseSensitivity;
        type SplitBehaviorFlags = crate::SplitBehaviorFlags;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = super::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;

        /// Appends the string str onto the end of this string.
        fn append<'a>(self: &'a mut QString, str: &QString) -> &'a mut QString;

        /// Clears the contents of the string and makes it null.
        fn clear(self: &mut QString);

        // We wrap this method to provide an enum so hide it from docs
        #[doc(hidden)]
        #[rust_name = "compare_i32"]
        fn compare(self: &QString, other: &QString, cs: CaseSensitivity) -> i32;

        /// Returns true if this string contains an occurrence of the string str; otherwise returns false.
        fn contains(self: &QString, str: &QString, cs: CaseSensitivity) -> bool;

        /// Returns true if the string ends with s; otherwise returns false.
        #[rust_name = "ends_with"]
        fn endsWith(self: &QString, s: &QString, cs: CaseSensitivity) -> bool;

        /// Returns true if the string has no characters; otherwise returns false.
        #[rust_name = "is_empty"]
        fn isEmpty(self: &QString) -> bool;

        /// Returns true if the string is lowercase, that is, it's identical to its toLower() folding.
        #[rust_name = "is_lower"]
        fn isLower(self: &QString) -> bool;

        /// Returns true if this string is null; otherwise returns false.
        #[rust_name = "is_null"]
        fn isNull(self: &QString) -> bool;

        /// Returns true if the string is read right to left.
        #[rust_name = "is_right_to_left"]
        fn isRightToLeft(self: &QString) -> bool;

        /// Returns true if the string is uppercase, that is, it's identical to its toUpper() folding.
        #[rust_name = "is_upper"]
        fn isUpper(self: &QString) -> bool;

        /// Returns true if the string contains valid UTF-16 encoded data, or false otherwise.
        #[rust_name = "is_valid_utf16"]
        fn isValidUtf16(self: &QString) -> bool;

        /// Prepends the string str to the beginning of this string and returns a reference to this string.
        fn prepend<'a>(self: &'a mut QString, str: &QString) -> &'a mut QString;

        /// Removes every occurrence of the given str string in this string, and returns a reference to this string.
        fn remove<'a>(self: &'a mut QString, str: &QString, cs: CaseSensitivity)
            -> &'a mut QString;

        /// Replaces every occurrence of the string before with the string after and returns a reference to this string.
        fn replace<'a>(
            self: &'a mut QString,
            before: &QString,
            after: &QString,
            cs: CaseSensitivity,
        ) -> &'a mut QString;

        /// Returns true if the string starts with s; otherwise returns false.
        #[rust_name = "starts_with"]
        fn startsWith(self: &QString, s: &QString, cs: CaseSensitivity) -> bool;

        /// Converts a plain text string to an HTML string with HTML metacharacters <, >, &, and " replaced by HTML entities.
        #[rust_name = "to_html_escaped"]
        fn toHtmlEscaped(self: &QString) -> QString;
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
        #[rust_name = "qstring_to_rust_string"]
        fn qstringToRustString(string: &QString) -> String;

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

/// The QString class provides a Unicode character string.
///
/// Note that QString is a UTF-16 whereas Rust strings are a UTF-8
#[repr(C)]
pub struct QString {
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 QString has one pointer as a member
    /// Qt6 QString has one member, which contains two pointers and a size_t
    #[cfg(qt_version_major = "5")]
    _space: MaybeUninit<usize>,
    #[cfg(qt_version_major = "6")]
    _space: MaybeUninit<[usize; 3]>,
}

impl Clone for QString {
    /// Constructs a copy of other.
    ///
    /// This operation takes constant time, because QString is implicitly shared.
    /// This makes returning a QString from a function very fast.
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
    /// Convert the QString to a Rust string
    ///
    /// Note that this converts from UTF-16 to UTF-8
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", <&QString as Into<String>>::into(self))
    }
}

impl fmt::Debug for QString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
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
    /// Constructs a QString from a Rust string
    ///
    /// Note that this converts from UTF-8 to UTF-16
    fn from(str: &str) -> Self {
        ffi::qstring_init_from_rust_string(str)
    }
}

impl From<&String> for QString {
    /// Constructs a QString from a Rust string
    ///
    /// Note that this converts from UTF-8 to UTF-16
    fn from(str: &String) -> Self {
        ffi::qstring_init_from_rust_string(str)
    }
}

impl From<&QString> for String {
    /// Convert the QString to a Rust string
    ///
    /// Note that this converts from UTF-16 to UTF-8
    fn from(qstring: &QString) -> Self {
        ffi::qstring_to_rust_string(qstring)
    }
}

impl QString {
    /// Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.
    pub fn arg(&self, a: &QString) -> Self {
        ffi::qstring_arg(self, a)
    }

    /// Lexically compares this string with the other string and
    /// returns if this string is less than, equal to, or greater than the other string.
    pub fn compare(&self, other: &QString, cs: ffi::CaseSensitivity) -> Ordering {
        self.compare_i32(other, cs).cmp(&0)
    }

    /// Returns the index position of the first occurrence of the string str in this string,
    /// searching forward from index position from. Returns -1 if str is not found.
    pub fn index_of(&self, str: &QString, from: isize, cs: ffi::CaseSensitivity) -> isize {
        ffi::qstring_index_of(self, str, from, cs)
    }

    /// Inserts the string str at the given index position and returns a mutable reference to this string.
    pub fn insert<'a>(&'a mut self, pos: isize, str: &Self) -> &'a mut Self {
        ffi::qstring_insert(self, pos, str)
    }

    /// Returns a substring that contains the n leftmost characters of the string.
    pub fn left(&self, n: isize) -> Self {
        ffi::qstring_left(self, n)
    }

    /// Returns the number of characters in this string.
    pub fn len(self: &QString) -> isize {
        ffi::qstring_len(self)
    }

    /// Returns a string that contains n characters of this string, starting at the specified position index.
    pub fn mid(&self, position: isize, n: isize) -> Self {
        ffi::qstring_mid(self, position, n)
    }

    /// Returns a substring that contains the n rightmost characters of the string.
    pub fn right(&self, n: isize) -> Self {
        ffi::qstring_right(self, n)
    }

    /// Returns a string that has whitespace removed from the start and the end,
    /// and that has each sequence of internal whitespace replaced with a single space.
    pub fn simplified(&self) -> Self {
        ffi::qstring_simplified(self)
    }

    /// Splits the string into substrings wherever sep occurs, and returns the list of those strings.
    /// If sep does not match anywhere in the string, split() returns a single-element list containing this string.
    pub fn split(
        &self,
        sep: &QString,
        behavior: ffi::SplitBehaviorFlags,
        cs: ffi::CaseSensitivity,
    ) -> ffi::QStringList {
        ffi::qstring_split(self, sep, behavior, cs)
    }

    /// Returns a Latin-1 representation of the string as a QByteArray.
    pub fn to_latin1(&self) -> ffi::QByteArray {
        ffi::qstring_to_latin1(self)
    }

    /// Returns the local 8-bit representation of the string as a QByteArray.
    /// The returned byte array is undefined if the string contains characters not supported by the local 8-bit encoding.
    pub fn to_local8bit(&self) -> ffi::QByteArray {
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

    /// Returns a UTF-8 representation of the string as a QByteArray.
    pub fn to_utf8(&self) -> ffi::QByteArray {
        ffi::qstring_to_utf8(self)
    }

    /// Returns a string that has whitespace removed from the start and the end.
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

#[cfg(test)]
mod test {
    use super::*;

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
