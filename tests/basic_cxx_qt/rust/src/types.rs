// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge(namespace = "cxx_qt::my_types")]
mod ffi {
    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyTypes {
        #[qproperty]
        boolean: bool,
        #[qproperty]
        float_32: f32,
        #[qproperty]
        float_64: f64,
        #[qproperty]
        int_8: i8,
        #[qproperty]
        int_16: i16,
        #[qproperty]
        int_32: i32,
        #[qproperty]
        uint_8: u8,
        #[qproperty]
        uint_16: u16,
        #[qproperty]
        uint_32: u32,
    }
}
