// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cxx_qt_module
// ANCHOR: book_make_qobject_macro
use cxx_qt::make_qobject;

#[make_qobject]
mod my_object {
    // ANCHOR_END: book_make_qobject_macro

    // ANCHOR: book_data_struct
    #[derive(Default)]
    pub struct Data {
        number: i32,
        string: String,
    }
    // ANCHOR_END: book_data_struct

    // ANCHOR: book_rustobj_struct
    #[derive(Default)]
    struct RustObj;
    // ANCHOR_END: book_rustobj_struct

    // ANCHOR: book_rustobj_impl
    impl RustObj {
        #[invokable]
        fn increment_number(&self, cpp: &mut CppObj) {
            cpp.set_number(cpp.number() + 1);
        }

        #[invokable]
        fn say_hi(&self, string: &str, number: i32) {
            println!(
                "Hi from Rust! String is '{}' and number is {}",
                string, number
            );
        }
    }
    // ANCHOR_END: book_rustobj_impl
}
// ANCHOR_END: book_cxx_qt_module
