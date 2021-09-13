// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt::make_qobject;

mod data;
pub mod sub;

#[make_qobject]
mod my_object {
    #[derive(Default)]
    pub struct Data {
        number: i32,
        string: String,
        sub: crate::sub::sub_object::SubObject,
    }

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn increment_number_self(&self, cpp: Pin<&mut CppObj>) {
            let value = cpp.number() + 1;
            cpp.set_number(value);
        }

        #[invokable]
        fn increment_number_sub(&self, sub: Pin<&mut crate::sub::sub_object::SubObject>) {
            let value = sub.number() + 1;
            sub.set_number(value);
        }

        #[invokable]
        fn increment_number(&self, number: i32) -> i32 {
            number + 1
        }

        #[invokable]
        fn say_hi(&self, string: &QString, number: i32) {
            let s: String = string.into();
            println!("Hi from Rust! String is {} and number is {}", s, number);
        }
    }
}
