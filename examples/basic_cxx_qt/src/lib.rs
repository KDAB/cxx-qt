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
    struct MyObject {
        number: i32,
        string: String,
        sub: crate::sub::sub_object::SubObject,
    }

    impl MyObject {
        fn double_number(&self, number: i32) -> i32 {
            number * 2
        }

        fn say_hi(&self, string: &str, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }
    }
}
