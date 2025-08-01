// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a serialisation can be used

// ANCHOR: book_macro_code

use serde::{Deserialize, Serialize};

/// A CXX-Qt bridge which shows how use serde for (de)serialization of the data in a QObjects' QPROPERTY's
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// QString from cxx_qt_lib
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        type Serialisation = super::SerialisationRust;

        /// An error signal
        #[qsignal]
        fn error(self: Pin<&mut Self>, message: QString);

        /// Retrieve the JSON form of this QObject
        #[qinvokable]
        #[cxx_name = "asJsonStr"]
        fn as_json_str(self: Pin<&mut Self>) -> QString;

        /// From a given JSON string try to load values for the Q_PROPERTYs
        // ANCHOR: book_grab_values
        #[qinvokable]
        #[cxx_name = "fromJsonStr"]
        fn from_json_str(self: Pin<&mut Self>, string: &QString);
        // ANCHOR_END: book_grab_values
    }
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;

/// A QObject which can be serialised
#[derive(Serialize, Deserialize)]
pub struct SerialisationRust {
    /// The number Q_PROPERTY
    pub number: i32,
    /// The string Q_PROPERTY
    pub string: QString,
}

impl Default for SerialisationRust {
    fn default() -> Self {
        let string = r#"{"number": 4, "string": "Hello World!"}"#;
        serde_json::from_str(string).unwrap()
    }
}

impl qobject::Serialisation {
    /// Retrieve the JSON form of this QObject
    pub fn as_json_str(self: Pin<&mut Self>) -> QString {
        match serde_json::to_string(&self.rust()) {
            Ok(data_string) => QString::from(&data_string),
            Err(err) => {
                self.error(QString::from(&err.to_string()));
                QString::default()
            }
        }
    }

    /// From a given JSON string try to load values for the Q_PROPERTYs
    // ANCHOR: book_grab_values
    pub fn from_json_str(mut self: Pin<&mut Self>, qstring: &QString) {
        match serde_json::from_str::<SerialisationRust>(&String::from(qstring)) {
            Ok(data_serde) => {
                self.as_mut().set_number(data_serde.number);
                self.as_mut().set_string(data_serde.string);
            }
            Err(err) => {
                self.error(QString::from(&err.to_string()));
            }
        }
    }
    // ANCHOR_END: book_grab_values
}
// ANCHOR_END: book_macro_code
