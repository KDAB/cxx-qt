// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DataSerde {
    number: i32,
    string: String,
}

impl From<&MyData> for DataSerde {
    fn from(value: &MyData) -> DataSerde {
        DataSerde {
            number: value.number,
            string: value.string.to_string(),
        }
    }
}

#[cxx_qt::bridge(cxx_file_stem = "my_data", namespace = "cxx_qt::my_data")]
mod ffi {
    use super::DataSerde;

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject]
    #[qproperty(i32, number)]
    #[qproperty(QString, string)]
    pub struct MyData {
        pub number: i32,
        pub string: QString,
    }

    impl Default for MyData {
        fn default() -> Self {
            let string = r#"{"number": 4, "string": "Hello World!"}"#;
            let data_serde: DataSerde = serde_json::from_str(string).unwrap();
            data_serde.into()
        }
    }

    impl From<DataSerde> for MyData {
        fn from(value: DataSerde) -> MyData {
            MyData {
                number: value.number,
                string: QString::from(&value.string),
            }
        }
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn as_json_str(self: &qobject::MyData) -> QString;

        #[qinvokable]
        fn grab_values(self: Pin<&mut qobject::MyData>);
    }
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;

// TODO: this will change to qobject::MyData once
// https://github.com/KDAB/cxx-qt/issues/559 is done
impl ffi::MyDataQt {
    pub fn as_json_str(&self) -> QString {
        let data_serde = DataSerde::from(self.rust());
        let data_string = serde_json::to_string(&data_serde).unwrap();
        QString::from(&data_string)
    }

    pub fn grab_values(mut self: Pin<&mut Self>) {
        let string = r#"{"number": 2, "string": "Goodbye!"}"#;
        let data_serde: DataSerde = serde_json::from_str(string).unwrap();
        self.as_mut().set_number(data_serde.number);
        self.as_mut().set_string(QString::from(&data_serde.string));
    }
}
// ANCHOR_END: book_macro_code
