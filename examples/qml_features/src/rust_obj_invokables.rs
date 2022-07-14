// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge]
pub mod rust_obj_invokables {
    #[derive(Default)]
    pub struct Data {
        number: i32,
    }

    struct RustObj {
        rust_only_field: i32,
    }

    impl Default for RustObj {
        fn default() -> Self {
            Self { rust_only_field: 1 }
        }
    }

    impl RustObj {
        // ANCHOR: book_cpp_obj
        #[invokable]
        fn invokable_mutate_cpp(&self, cpp: &mut CppObj) {
            cpp.set_number(cpp.number() * 2);
        }
        // ANCHOR_END: book_cpp_obj

        #[invokable]
        fn invokable_return(&self) -> i32 {
            self.rust_only_field
        }

        #[invokable]
        fn invokable_multiply(&mut self, factor: i32) -> i32 {
            self.rust_only_method(factor);
            self.rust_only_field
        }

        fn rust_only_method(&mut self, factor: i32) {
            self.rust_only_field *= factor;
        }
    }
}
// ANCHOR_END: book_macro_code
