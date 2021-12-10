// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt::make_qobject;

#[make_qobject]
mod my_object {
    #[derive(Default)]
    pub struct Data {
        number: i32,
        string: String,
    }

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn increment_number(&self, cpp: Pin<&mut CppObj>) {
            let mut wrapper = CppObjWrapper::new(cpp);
            wrapper.set_number(wrapper.number() + 1);
        }

        #[invokable]
        fn say_hi(&self, string: &QString, number: i32) {
            let s: String = string.into();
            println!("Hi from Rust! String is '{}' and number is {}", s, number);
        }
    }
}
