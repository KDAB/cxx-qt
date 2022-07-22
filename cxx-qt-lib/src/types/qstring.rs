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
        #[rust_name = "qstring_to_rust_string"]
        fn qstringToRustString(string: &QString) -> String;

        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qstring_init_from_rust_string"]
        fn qstringInitFromRustString(string: &str) -> UniquePtr<QString>;
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qstring_init_from_qstring"]
        fn qstringInitFromQString(string: &QString) -> UniquePtr<QString>;
    }

    impl UniquePtr<QString> {}
}

/// The QString class provides a Unicode character string.
///
/// Note that this is the C++ representation and String or &str should be used in Rust.
pub type QString = ffi::QString;

impl QString {
    pub fn from_ref(string: &QString) -> cxx::UniquePtr<Self> {
        ffi::qstring_init_from_qstring(string)
    }

    pub fn from_str(str: &str) -> cxx::UniquePtr<Self> {
        ffi::qstring_init_from_rust_string(str)
    }
}

impl std::fmt::Display for QString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", ffi::qstring_to_rust_string(self))
    }
}

impl From<&QString> for cxx::UniquePtr<QString> {
    fn from(value: &QString) -> cxx::UniquePtr<QString> {
        QString::from_ref(value)
    }
}
