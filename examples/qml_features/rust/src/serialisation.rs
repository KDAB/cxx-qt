// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a serialisation can be used

// ANCHOR: book_macro_code

use serde::{Deserialize, Serialize};

/// A struct representating our serialised form
#[derive(Deserialize, Serialize)]
pub struct DataSerde {
    number: i32,
    string: String,
}

impl From<&SerialisationRust> for DataSerde {
    fn from(value: &SerialisationRust) -> DataSerde {
        DataSerde {
            number: value.number,
            string: value.string.to_string(),
        }
    }
}

/// A CXX-Qt bridge which shows how use serde for (de)serialization of the data in a QObjects' QPROPERTY's
#[cxx_qt::bridge(cxx_file_stem = "serialisation")]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// QString from cxx_qt_lib
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[cxx_qt::qobject(qml_element)]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        type Serialisation = super::SerialisationRust;

        /// An error signal
        #[qsignal]
        fn error(self: Pin<&mut Serialisation>, message: QString);

        /// Retrieve the JSON form of this QObject
        #[qinvokable]
        fn as_json_str(self: Pin<&mut Serialisation>) -> QString;

        /// From a given JSON string try to load values for the Q_PROPERTYs
        // ANCHOR: book_grab_values
        #[qinvokable]
        fn from_json_str(self: Pin<&mut Serialisation>, string: &QString);
        // ANCHOR_END: book_grab_values
    }
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;

/// A QObject which can be serialised
pub struct SerialisationRust {
    /// The number Q_PROPERTY
    pub number: i32,
    /// The string Q_PROPERTY
    pub string: QString,
}

impl Default for SerialisationRust {
    fn default() -> Self {
        let string = r#"{"number": 4, "string": "Hello World!"}"#;
        let data_serde: DataSerde = serde_json::from_str(string).unwrap();
        data_serde.into()
    }
}

impl From<DataSerde> for SerialisationRust {
    fn from(value: DataSerde) -> Self {
        Self {
            number: value.number,
            string: QString::from(&value.string),
        }
    }
}

impl qobject::Serialisation {
    /// Retrieve the JSON form of this QObject
    pub fn as_json_str(self: Pin<&mut Self>) -> QString {
        let data_serde = DataSerde::from(self.rust());
        match serde_json::to_string(&data_serde) {
            Ok(data_string) => QString::from(&data_string),
            Err(err) => {
                self.error(QString::from(&err.to_string()));
                QString::default()
            }
        }
    }

    /// From a given JSON string try to load values for the Q_PROPERTYs
    // ANCHOR: book_grab_values
    pub fn from_json_str(mut self: Pin<&mut Self>, string: &QString) {
        match serde_json::from_str::<DataSerde>(&string.to_string()) {
            Ok(data_serde) => {
                self.as_mut().set_number(data_serde.number);
                self.as_mut().set_string(QString::from(&data_serde.string));
            }
            Err(err) => {
                self.error(QString::from(&err.to_string()));
            }
        }
    }
    // ANCHOR_END: book_grab_values
}
// ANCHOR_END: book_macro_code
