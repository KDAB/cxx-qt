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
    struct RustObj {
        update_call_count: i32,
    }

    impl RustObj {
        fn double_number_self(&self, cpp: Pin<&mut CppObj>) {
            let value = cpp.number() * 2;
            cpp.set_number(value);
        }

        fn double_number_sub(&self, sub: Pin<&mut crate::sub::sub_object::SubObject>) {
            let value = sub.number() * 2;
            sub.set_number(value);
        }

        fn double_number(&self, number: i32) -> i32 {
            number * 2
        }

        fn say_hi(&self, string: &QString, number: i32) {
            let s: String = string.into();
            println!("Hi from Rust! String is {} and number is {}", s, number);
        }

        fn request_update(&self, cpp: Pin<&mut CppObj>) {
            let wrapper = CppObjWrapper::new(cpp);
            let update_requester = wrapper.update_requester();
            update_requester.request_update();
        }

        fn update_call_count(&self) -> i32 {
            self.update_call_count
        }
    }

    impl UpdateRequestHandler<CppObj> for RustObj {
        fn handle_update_request(&mut self, _cpp: Pin<&mut CppObj>) {
            self.update_call_count += 1;
        }
    }
}
