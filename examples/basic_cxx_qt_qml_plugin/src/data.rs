// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt::make_qobject;

#[make_qobject]
mod data {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct MyData {
        number: i32,
        string: String,
    }

    impl Default for MyData {
        fn default() -> Self {
            let data = r#"{"number": 4, "string": "Hello World!"}"#;
            serde_json::from_str(data).unwrap()
        }
    }

    impl MyData {
        fn as_json_str(&self) -> String {
            serde_json::to_string(&self).unwrap()
        }
    }
}
