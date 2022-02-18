// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt::make_qobject;

// ANCHOR: book_macro_code
#[make_qobject]
mod data_struct_properties {
    #[derive(Default)]
    pub struct Data {
        number: i32,
    }

    #[derive(Default)]
    struct RustObj;
}
// ANCHOR_END: book_macro_code
