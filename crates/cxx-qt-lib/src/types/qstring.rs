// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QString = super::QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qstring_drop"]
        fn qstringDrop(string: &mut QString);

        #[doc(hidden)]
        #[rust_name = "qstring_init_default"]
        fn qstringInitDefault() -> QString;
        #[doc(hidden)]
        #[rust_name = "qstring_init_from_rust_string"]
        fn qstringInitFromRustString(string: &str) -> QString;
        #[doc(hidden)]
        #[rust_name = "qstring_init_from_qstring"]
        fn qstringInitFromQString(string: &QString) -> QString;

        #[doc(hidden)]
        #[rust_name = "qstring_to_rust_string"]
        fn qstringToRustString(string: &QString) -> String;
    }
}

/// The QString class provides a Unicode character string.
///
/// Note that QString is a UTF-16 whereas Rust strings are a UTF-8
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

impl std::fmt::Display for QString {
    /// Convert the QString to a Rust string
    ///
    /// Note that this converts from UTF-16 to UTF-8
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", <&QString as Into<String>>::into(self))
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

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QString {
    type Id = type_id!("QString");
    type Kind = cxx::kind::Trivial;
}
