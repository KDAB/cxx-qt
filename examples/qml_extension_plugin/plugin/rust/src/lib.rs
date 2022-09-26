// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// Note: keep any changes here in sync with the main README.md

use serde::{Deserialize, Serialize};

// Represent the data within the QObject below with serde friendly types, so we can (de)serialize it
#[derive(Deserialize, Serialize)]
pub struct DataSerde {
    number: i32,
    string: String,
}

impl From<&MyObject> for DataSerde {
    fn from(value: &MyObject) -> DataSerde {
        DataSerde {
            number: value.number,
            string: value.string.to_string(),
        }
    }
}

const DEFAULT_STR: &str = r#"{"number": 1, "string": "Hello World!"}"#;

#[cxx_qt::bridge(namespace = "core")]
mod ffi {
    use super::{DataSerde, DEFAULT_STR};

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject]
    pub struct MyObject {
        #[qproperty]
        pub number: i32,
        #[qproperty]
        pub string: QString,
    }

    impl Default for MyObject {
        fn default() -> Self {
            let data_serde: DataSerde = serde_json::from_str(DEFAULT_STR).unwrap();
            data_serde.into()
        }
    }

    impl From<DataSerde> for MyObject {
        fn from(value: DataSerde) -> MyObject {
            MyObject {
                number: value.number,
                string: QString::from(&value.string),
            }
        }
    }

    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn increment(self: Pin<&mut Self>) {
            let new_number = self.get_number() + 1;
            self.set_number(new_number);
        }

        #[qinvokable]
        pub fn reset(mut self: Pin<&mut Self>) {
            let data: DataSerde = serde_json::from_str(DEFAULT_STR).unwrap();
            self.as_mut().set_number(data.number);
            self.as_mut().set_string(QString::from(&data.string));
        }

        #[qinvokable]
        pub fn serialize(&self) -> QString {
            let data_serde = DataSerde::from(self.rust());
            let data_string = serde_json::to_string(&data_serde).unwrap();
            QString::from(&data_string)
        }

        #[qinvokable]
        pub fn grab_values(mut self: Pin<&mut Self>) {
            let string = r#"{"number": 2, "string": "Goodbye!"}"#;
            let data: DataSerde = serde_json::from_str(string).unwrap();
            self.as_mut().set_number(data.number);
            self.as_mut().set_string(QString::from(&data.string));
        }
    }
}
