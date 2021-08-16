// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt::make_qobject;

#[make_qobject]
pub mod sub_object {
    #[derive(Default)]
    pub struct Data {
        number: i32,
        string: String,
    }

    pub struct RustObj {
        number: i32,
        string: String,
    }

    impl RustObj {
        fn increment_number_self(&self, cpp: Pin<&mut CppObj>) {
            let value = cpp.number();
            cpp.set_number(value + 1);
        }

        fn increment_number(&self, number: i32) -> i32 {
            number + 1
        }

        fn say_hi(&self, string: &str, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }
    }
}
