// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fmt::Display;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QString;

        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qstring_init_from_rust_string"]
        fn qstringInitFromRustString(string: &str) -> UniquePtr<QString>;
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qstring_to_rust_string"]
        fn qstringToRustString(string: &QString) -> String;
    }

    impl UniquePtr<QString> {}
}

/// The QStringCpp class provides a Unicode character string.
///
/// Note that this is the C++ representation and String or &str should be used in Rust.
pub type QStringCpp = ffi::QString;

impl QStringCpp {
    /// Create a new Rust String from this QStringCpp. This operation
    /// needs to convert the UTF-16 data in the QString to UTF-8
    /// data and thus likely needs to an allocate. This is essentially
    /// a copy and so any changes will not propagate to the QStringCpp.
    pub fn to_rust(&self) -> String {
        ffi::qstring_to_rust_string(self)
    }
}

impl crate::ToUniquePtr for &String {
    type CppType = QStringCpp;

    /// Retrieve the UniquePtr to the Qt QStringCpp of this Rust String
    /// so that this object can be passed back to C++.
    fn to_unique_ptr(self) -> cxx::UniquePtr<QStringCpp> {
        ffi::qstring_init_from_rust_string(self.as_ref())
    }
}

impl crate::ToUniquePtr for &str {
    type CppType = QStringCpp;

    /// Retrieve the UniquePtr to the Qt QStringCpp of this Rust &str
    /// so that this object can be passed back to C++.
    fn to_unique_ptr(self) -> cxx::UniquePtr<QStringCpp> {
        ffi::qstring_init_from_rust_string(self)
    }
}

impl From<&QStringCpp> for String {
    fn from(qstring: &QStringCpp) -> Self {
        qstring.to_rust()
    }
}

impl Display for QStringCpp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.into();
        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for QStringCpp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
