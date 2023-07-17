// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge(cxx_file_stem = "my_types", namespace = "cxx_qt::my_types")]
mod ffi {
    extern "RustQt" {
        #[cxx_qt::qobject]
        #[qproperty(bool, boolean)]
        #[qproperty(f32, float_32)]
        #[qproperty(f64, float_64)]
        #[qproperty(i8, int_8)]
        #[qproperty(i16, int_16)]
        #[qproperty(i32, int_32)]
        #[qproperty(u8, uint_8)]
        #[qproperty(u16, uint_16)]
        #[qproperty(u32, uint_32)]
        type MyTypes = super::MyTypesRust;
    }
}

#[derive(Default)]
pub struct MyTypesRust {
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
