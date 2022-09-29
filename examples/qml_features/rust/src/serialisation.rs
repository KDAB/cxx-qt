// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code

use serde::{Deserialize, Serialize};

// TODO: once Qt types support serde again, the Serialisation struct can be used
// https://github.com/KDAB/cxx-qt/issues/292
#[derive(Deserialize, Serialize)]
pub struct DataSerde {
    number: i32,
    string: String,
}

impl From<&Serialisation> for DataSerde {
    fn from(value: &Serialisation) -> DataSerde {
        DataSerde {
            number: value.number,
            string: value.string.to_string(),
        }
    }
}

#[cxx_qt::bridge(cxx_file_stem = "serialisation")]
mod ffi {
    use super::DataSerde;

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject]
    pub struct Serialisation {
        #[qproperty]
        pub number: i32,
        #[qproperty]
        pub string: QString,
    }

    #[cxx_qt::signals(Serialisation)]
    pub enum Connection {
        Error { message: QString },
    }

    impl Default for Serialisation {
        fn default() -> Self {
            let string = r#"{"number": 4, "string": "Hello World!"}"#;
            let data_serde: DataSerde = serde_json::from_str(string).unwrap();
            data_serde.into()
        }
    }

    impl From<DataSerde> for Serialisation {
        fn from(value: DataSerde) -> Serialisation {
            Serialisation {
                number: value.number,
                string: QString::from(&value.string),
            }
        }
    }

    impl qobject::Serialisation {
        #[qinvokable]
        pub fn as_json_str(self: Pin<&mut Self>) -> QString {
            let data_serde = DataSerde::from(self.rust());
            match serde_json::to_string(&data_serde) {
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
            match serde_json::from_str::<DataSerde>(&string.to_string()) {
                Ok(data_serde) => {
                    self.as_mut().set_number(data_serde.number);
                    self.as_mut().set_string(QString::from(&data_serde.string));
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
