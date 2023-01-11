// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_f32"]
        fn qvariantCanConvertF32(variant: &QVariant) -> bool;
        #[rust_name = "construct_f32"]
        fn qvariantConstruct(value: &f32) -> QVariant;
        #[rust_name = "value_or_default_f32"]
        fn qvariantValueOrDefault(variant: &QVariant) -> f32;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_f32(variant)
}

pub(crate) fn construct(value: &f32) -> ffi::QVariant {
    ffi::construct_f32(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> f32 {
    ffi::value_or_default_f32(variant)
}
