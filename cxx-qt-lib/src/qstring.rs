// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

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

pub type QString = ffi::QString;

impl QString {
    /// Create a new Rust string from this QString. This operation
    /// needs to convert the UTF-16 data in the QString to UTF-8
    /// data and thus likely needs to an allocate. This is essentially
    /// a copy and so any changes will not propagate to the QString.
    pub fn to_rust(&self) -> String {
        ffi::qstring_to_rust_string(self)
    }
}

impl crate::ToUniquePtr for &String {
    type CppType = QString;

    /// Retrieve the UniquePtr to the Qt QString of this Rust String
    /// so that this object can be passed back to C++.
    fn to_unique_ptr(self) -> cxx::UniquePtr<QString> {
        ffi::qstring_init_from_rust_string(self.as_ref())
    }
}

impl crate::ToUniquePtr for &str {
    type CppType = QString;

    /// Retrieve the UniquePtr to the Qt QString of this Rust &str
    /// so that this object can be passed back to C++.
    fn to_unique_ptr(self) -> cxx::UniquePtr<QString> {
        ffi::qstring_init_from_rust_string(self)
    }
}

impl From<&QString> for String {
    fn from(qstring: &QString) -> Self {
        qstring.to_rust()
    }
}
