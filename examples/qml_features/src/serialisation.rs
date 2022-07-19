// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge]
mod serialisation {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct Data {
        number: i32,
        string: String,
    }

    impl Default for Data {
        fn default() -> Self {
            let string = r#"{"number": 4, "string": "Hello World!"}"#;
            serde_json::from_str(string).unwrap()
        }
    }

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        #[invokable]
        pub fn as_json_str(&self, cpp: &mut CppObj) -> String {
            let data = Data::from(cpp);
            serde_json::to_string(&data).unwrap()
        }

        // ANCHOR: book_grab_values
        #[invokable]
        pub fn grab_values(&self, cpp: &mut CppObj) {
            let string = r#"{"number": 2, "string": "Goodbye!"}"#;
            let data: Data = serde_json::from_str(string).unwrap();
            cpp.grab_values_from_data(data);
        }
        // ANCHOR_END: book_grab_values
    }
}
// ANCHOR_END: book_macro_code
