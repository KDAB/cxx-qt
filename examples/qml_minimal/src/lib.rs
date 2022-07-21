// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cxx_qt_module
// ANCHOR: book_bridge_macro

#[cxx_qt::bridge]
mod my_object {
    // ANCHOR_END: book_bridge_macro

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
    }

    // ANCHOR: book_data_struct
    pub struct Data {
        number: i32,
        string: UniquePtr<QString>,
    }

    impl Default for Data {
        fn default() -> Self {
            Self {
                number: 0,
                string: QString::from_str(""),
            }
        }
    }

    // ANCHOR_END: book_data_struct

    // ANCHOR: book_rustobj_struct
    #[derive(Default)]
    pub struct RustObj;
    // ANCHOR_END: book_rustobj_struct

    // ANCHOR: book_rustobj_impl
    impl RustObj {
        #[invokable]
        pub fn increment_number(&self, cpp: &mut CppObj) {
            cpp.set_number(cpp.number() + 1);
        }

        #[invokable]
        pub fn say_hi(&self, string: &QString, number: i32) {
            println!(
                "Hi from Rust! String is '{}' and number is {}",
                string, number
            );
        }
    }
    // ANCHOR_END: book_rustobj_impl
}
// ANCHOR_END: book_cxx_qt_module
