// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt::make_qobject;

#[make_qobject]
mod my_data {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct Data {
        number: i32,
        string: String,
    }

    impl Default for Data {
        fn default() -> Self {
            let string = r#"{"number": 4, "string": "Hello World!"}"#;
            serde_json::from_str(string).unwrap()
        }
    }

    struct RustObj {
        number: i32,
        string: String,
    }

    impl RustObj {
        fn as_json_str(&self) -> String {
            let data = Data::from(self);
            serde_json::to_string(&data).unwrap()
        }
    }
}
