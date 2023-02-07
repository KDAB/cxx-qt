// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code

#[cxx_qt::bridge(cxx_file_stem = "serialisation")]
mod ffi {
    use serde::{Deserialize, Serialize};

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[derive(Deserialize, Serialize)]
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    pub struct Serialisation {
        #[qproperty]
        pub number: i32,
        #[qproperty]
        pub string: QString,
    }

    #[cxx_qt::qsignals(Serialisation)]
    pub enum Connection {
        Error { message: QString },
    }

    impl Default for Serialisation {
        fn default() -> Self {
            let string = r#"{"number": 4, "string": "Hello World!"}"#;
            serde_json::from_str(string).unwrap()
        }
    }

    impl qobject::Serialisation {
        #[qinvokable]
        pub fn as_json_str(self: Pin<&mut Self>) -> QString {
            match serde_json::to_string(&self.rust()) {
                Ok(data_string) => QString::from(&data_string),
                Err(err) => {
                    self.emit(Connection::Error {
                        message: QString::from(&err.to_string()),
                    });
                    QString::default()
                }
            }
        }

        // ANCHOR: book_grab_values
        #[qinvokable]
        pub fn from_json_str(mut self: Pin<&mut Self>, string: &QString) {
            match serde_json::from_str::<Serialisation>(&string.to_string()) {
                Ok(data_serde) => {
                    self.as_mut().set_number(data_serde.number);
                    self.as_mut().set_string(data_serde.string);
                }
                Err(err) => {
                    self.emit(Connection::Error {
                        message: QString::from(&err.to_string()),
                    });
                }
            }
        }
        // ANCHOR_END: book_grab_values
    }
}
// ANCHOR_END: book_macro_code
