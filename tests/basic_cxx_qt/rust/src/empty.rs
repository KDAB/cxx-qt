// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge]
mod ffi {
    extern "RustQt" {
        #[qobject]
        type Empty = super::EmptyRust;
    }
}

#[derive(Default)]
pub struct EmptyRust;
