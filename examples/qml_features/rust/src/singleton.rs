// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "rust_singleton")]
pub mod ffi {
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0", qml_singleton)]
    #[derive(Default)]
    pub struct RustSingleton {
        #[qproperty]
        persistent_value: i32,
    }

    impl qobject::RustSingleton {
        #[qinvokable]
        pub fn increment(self: Pin<&mut Self>) {
            let new_value = self.persistent_value() + 1;
            self.set_persistent_value(new_value);
        }
    }
}
