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

impl From<&MyObjectRust> for DataSerde {
    fn from(value: &MyObjectRust) -> DataSerde {
        DataSerde {
            number: value.number,
            string: value.string.to_string(),
        }
    }
}

const DEFAULT_STR: &str = r#"{"number": 1, "string": "Hello World!"}"#;

#[cxx_qt::bridge(cxx_file_stem = "my_object", namespace = "core")]
pub mod qobject {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        type MyObject = super::MyObjectRust;

        #[qinvokable]
        pub fn increment(self: Pin<&mut MyObject>);

        #[qinvokable]
        pub fn reset(self: Pin<&mut MyObject>);

        #[qinvokable]
        pub fn serialize(self: &MyObject) -> QString;

        #[qinvokable]
        pub fn grab_values(self: Pin<&mut MyObject>);
    }
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;

pub struct MyObjectRust {
    pub number: i32,
    pub string: QString,
}

impl Default for MyObjectRust {
    fn default() -> Self {
        let data_serde: DataSerde = serde_json::from_str(DEFAULT_STR).unwrap();
        data_serde.into()
    }
}

impl From<DataSerde> for MyObjectRust {
    fn from(value: DataSerde) -> Self {
        Self {
            number: value.number,
            string: QString::from(&value.string),
        }
    }
}

impl qobject::MyObject {
    pub fn increment(self: Pin<&mut Self>) {
        let new_number = self.number() + 1;
        self.set_number(new_number);
    }

    pub fn reset(mut self: Pin<&mut Self>) {
        let data: DataSerde = serde_json::from_str(DEFAULT_STR).unwrap();
        self.as_mut().set_number(data.number);
        self.as_mut().set_string(QString::from(&data.string));
    }

    pub fn serialize(&self) -> QString {
        let data_serde = DataSerde::from(self.rust());
        let data_string = serde_json::to_string(&data_serde).unwrap();
        QString::from(&data_string)
    }

    pub fn grab_values(mut self: Pin<&mut Self>) {
        let string = r#"{"number": 2, "string": "Goodbye!"}"#;
        let data: DataSerde = serde_json::from_str(string).unwrap();
        self.as_mut().set_number(data.number);
        self.as_mut().set_string(QString::from(&data.string));
    }
}
