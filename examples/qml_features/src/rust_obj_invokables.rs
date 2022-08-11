// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge]
pub mod ffi {
    #[derive(Default)]
    pub struct Data {
        number: i32,
    }

    #[cxx_qt::qobject]
    pub struct RustObjInvokables {
        rust_only_field: i32,
    }

    impl Default for RustObjInvokables {
        fn default() -> Self {
            Self { rust_only_field: 1 }
        }
    }

    impl cxx_qt::QObject<RustObjInvokables> {
        // ANCHOR: book_cpp_obj
        #[qinvokable]
        pub fn invokable_mutate_cpp(&self, cpp: &mut CppObj) {
            cpp.set_number(cpp.number() * 2);
        }
        // ANCHOR_END: book_cpp_obj

        #[qinvokable]
        pub fn invokable_return(&self) -> i32 {
            self.rust_only_field
        }

        #[qinvokable]
        pub fn invokable_multiply(&mut self, factor: i32) -> i32 {
            self.rust_only_method(factor);
            self.rust_only_field
        }
    }

    impl RustObjInvokables {
        fn rust_only_method(&mut self, factor: i32) {
            self.rust_only_field *= factor;
        }
    }
}
// ANCHOR_END: book_macro_code
