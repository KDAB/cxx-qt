// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// Note: keep any changes here in sync with the main README.md

#[cxx_qt::bridge]
mod my_object {
    use serde::{Deserialize, Serialize};

    const DEFAULT_STR: &str = r#"{"number": 1, "string": "Hello World!"}"#;

    #[derive(Deserialize, Serialize)]
    pub struct Data {
        number: i32,
        string: String,
    }

    impl Default for Data {
        fn default() -> Self {
            serde_json::from_str(DEFAULT_STR).unwrap()
        }
    }

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        #[invokable]
        pub fn increment(&self, cpp: &mut CppObj) {
            cpp.set_number(cpp.number() + 1);
        }

        #[invokable]
        pub fn reset(&self, cpp: &mut CppObj) {
            let data: Data = serde_json::from_str(DEFAULT_STR).unwrap();
            cpp.grab_values_from_data(data);
        }

        #[invokable]
        pub fn serialize(&self, cpp: &mut CppObj) -> String {
            let data = Data::from(cpp);
            serde_json::to_string(&data).unwrap()
        }

        #[invokable]
        pub fn grab_values(&self, cpp: &mut CppObj) {
            let string = r#"{"number": 2, "string": "Goodbye!"}"#;
            let data: Data = serde_json::from_str(string).unwrap();
            cpp.grab_values_from_data(data);
        }
    }
}
