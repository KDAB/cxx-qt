// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge]
mod my_types {
    extern "Qt" {
        #[derive(Default)]
        pub struct Data {
            boolean: bool,
            float_32: f32,
            float_64: f64,
            int_8: i8,
            int_16: i16,
            int_32: i32,
            uint_8: u8,
            uint_16: u16,
            uint_32: u32,
        }

        #[derive(Default)]
        struct RustObj;
    }
}
