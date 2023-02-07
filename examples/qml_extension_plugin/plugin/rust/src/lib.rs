// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

const DEFAULT_STR: &str = r#"{"number": 1, "string": "Hello World!"}"#;

#[cxx_qt::bridge(cxx_file_stem = "my_object", namespace = "core")]
mod ffi {
    use super::DEFAULT_STR;
    use serde::{Deserialize, Serialize};

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[derive(Deserialize, Serialize)]
    #[cxx_qt::qobject]
    pub struct MyObject {
        #[qproperty]
        pub number: i32,
        #[qproperty]
        pub string: QString,
    }

    impl Default for MyObject {
        fn default() -> Self {
            serde_json::from_str(DEFAULT_STR).unwrap()
        }
    }

    impl qobject::MyObject {
        #[qinvokable]
        pub fn increment(self: Pin<&mut Self>) {
            let new_number = self.number() + 1;
            self.set_number(new_number);
        }

        #[qinvokable]
        pub fn reset(mut self: Pin<&mut Self>) {
            let data: MyObject = serde_json::from_str(DEFAULT_STR).unwrap();
            self.as_mut().set_number(data.number);
            self.as_mut().set_string(data.string);
        }

        #[qinvokable]
        pub fn serialize(&self) -> QString {
            let data_string = serde_json::to_string(&self.rust()).unwrap();
            QString::from(&data_string)
        }

        #[qinvokable]
        pub fn grab_values(mut self: Pin<&mut Self>) {
            let string = r#"{"number": 2, "string": "Goodbye!"}"#;
            let data: MyObject = serde_json::from_str(string).unwrap();
            self.as_mut().set_number(data.number);
            self.as_mut().set_string(data.string);
        }
    }
}
