// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge]
mod data_struct_properties {
    #[derive(Default)]
    pub struct Data {
        number: i32,
    }

    #[derive(Default)]
    struct RustObj;
}
// ANCHOR_END: book_macro_code
