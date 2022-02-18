// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt::make_qobject;

// ANCHOR: book_macro_code
#[make_qobject]
mod handler_property_change {
    #[derive(Default)]
    pub struct Data {
        number: i32,
    }

    #[derive(Default)]
    struct RustObj {
        count: u32,
    }

    impl RustObj {
        #[invokable]
        fn get_count(&self) -> u32 {
            self.count
        }
    }

    impl PropertyChangeHandler<CppObj<'_>, Property> for RustObj {
        fn handle_property_change(&mut self, cpp: &mut CppObj, property: Property) {
            match property {
                Property::Number => {
                    println!("New Number: {}", cpp.number());
                    self.count += 1;
                }
                _others => {}
            }
        }
    }
}
// ANCHOR_END: book_macro_code
