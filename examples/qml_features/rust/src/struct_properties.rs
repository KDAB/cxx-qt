// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "struct_properties")]
mod ffi {
    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct StructProperties {
        #[qproperty]
        number: i32,
    }
}
// ANCHOR_END: book_macro_code
