// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "my_data", namespace = "cxx_qt::my_data")]
mod ffi {
    use serde::{Deserialize, Serialize};

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[derive(Deserialize, Serialize)]
    #[cxx_qt::qobject]
    pub struct MyData {
        #[qproperty]
        pub number: i32,
        #[qproperty(cxx_type = "QString")]
        pub string: QString,
    }

    impl Default for MyData {
        fn default() -> Self {
            let string = r#"{"number": 4, "string": "Hello World!"}"#;
            serde_json::from_str(string).unwrap()
        }
    }

    impl qobject::MyData {
        #[qinvokable]
        pub fn as_json_str(&self) -> QString {
            let data_string = serde_json::to_string(&self.rust()).unwrap();
            QString::from(&data_string)
        }

        #[qinvokable]
        pub fn grab_values(mut self: Pin<&mut Self>) {
            let string = r#"{"number": 2, "string": "Goodbye!"}"#;
            let data_serde: MyData = serde_json::from_str(string).unwrap();
            self.as_mut().set_number(data_serde.number);
            self.as_mut().set_string(data_serde.string);
        }
    }
}
// ANCHOR_END: book_macro_code
