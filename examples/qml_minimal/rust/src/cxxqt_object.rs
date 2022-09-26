// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cxx_qt_module
// ANCHOR: book_bridge_macro

#[cxx_qt::bridge]
mod ffi {
    // ANCHOR_END: book_bridge_macro

    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
    }

    // ANCHOR: book_rustobj_struct
    #[cxx_qt::qobject]
    pub struct MyObject {
        #[qproperty]
        number: i32,
        #[qproperty]
        string: QString,
    }

    impl Default for MyObject {
        fn default() -> Self {
            Self {
                number: 0,
                string: QString::from(""),
            }
        }
    }
    // ANCHOR_END: book_rustobj_struct

    // ANCHOR: book_rustobj_impl
    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn increment_number(self: Pin<&mut Self>) {
            let previous = *self.as_ref().get_number();
            self.set_number(previous + 1);
        }

        #[qinvokable]
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
