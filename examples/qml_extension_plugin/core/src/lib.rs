// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// Note: keep any changes here in sync with the main README.md

use serde::{Deserialize, Serialize};

// Represent the Data struct below with serde friendly types, so we can (de)serialize it
#[derive(Deserialize, Serialize)]
pub struct DataSerde {
    number: i32,
    string: String,
}

impl From<Data> for DataSerde {
    fn from(value: Data) -> DataSerde {
        DataSerde {
            number: value.number,
            string: value.string.to_string(),
        }
    }
}

const DEFAULT_STR: &str = r#"{"number": 1, "string": "Hello World!"}"#;

#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    use super::{DataSerde, DEFAULT_STR};

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
    }

    pub struct Data {
        pub number: i32,
        pub string: UniquePtr<QString>,
    }

    impl Default for Data {
        fn default() -> Self {
            let data_serde: DataSerde = serde_json::from_str(DEFAULT_STR).unwrap();
            data_serde.into()
        }
    }

    impl From<DataSerde> for Data {
        fn from(value: DataSerde) -> Data {
            Data {
                number: value.number,
                string: QString::from_str(&value.string),
            }
        }
    }

    #[derive(Default)]
    pub struct RustObj;

    impl cxx_qt::QObject<RustObj> {
        #[invokable]
        pub fn increment(&self, cpp: &mut CppObj) {
            cpp.set_number(cpp.number() + 1);
        }

        #[invokable]
        pub fn reset(&self, cpp: &mut CppObj) {
            let data: DataSerde = serde_json::from_str(DEFAULT_STR).unwrap();
            cpp.grab_values_from_data(data.into());
        }

        #[invokable]
        pub fn serialize(&self, cpp: &mut CppObj) -> UniquePtr<QString> {
            let data = Data::from(cpp);
            let data_serde = DataSerde::from(data);
            let data_string = serde_json::to_string(&data_serde).unwrap();
            QString::from_str(&data_string)
        }

        #[invokable]
        pub fn grab_values(&self, cpp: &mut CppObj) {
            let string = r#"{"number": 2, "string": "Goodbye!"}"#;
            let data: DataSerde = serde_json::from_str(string).unwrap();
            cpp.grab_values_from_data(data.into());
        }
    }
}
