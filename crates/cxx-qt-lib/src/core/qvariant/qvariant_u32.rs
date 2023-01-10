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
        #[rust_name = "can_convert_u32"]
        fn qvariantCanConvertU32(variant: &QVariant) -> bool;
        #[rust_name = "construct_u32"]
        fn qvariantConstruct(value: &u32) -> QVariant;
        #[rust_name = "value_or_default_u32"]
        fn qvariantValueOrDefault(variant: &QVariant) -> u32;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_u32(variant)
}

pub(crate) fn construct(value: &u32) -> ffi::QVariant {
    ffi::construct_u32(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> u32 {
    ffi::value_or_default_u32(variant)
}
