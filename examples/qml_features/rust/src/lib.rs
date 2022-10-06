// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod custom_base;
mod empty;
mod mock_qt_types;
mod rust_obj_invokables;
mod serialisation;
mod signals;
mod struct_properties;
mod threading;
mod types;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

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

    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn increment_number_self(mut self: Pin<&mut Self>) {
            let value = self.number() + 1;
            self.as_mut().set_number(value);
        }

        #[qinvokable]
        pub fn increment_number(&self, number: i32) -> i32 {
            number + 1
        }

        #[qinvokable]
        pub fn say_hi(&self, string: &QString, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }
    }
}
