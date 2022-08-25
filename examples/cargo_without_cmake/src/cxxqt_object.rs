// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
    }

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

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject;
    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn increment_number(self: Pin<&mut Self>) {
            let previous = self.as_ref().number();
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
}
