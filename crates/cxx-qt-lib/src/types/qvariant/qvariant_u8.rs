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
        #[rust_name = "can_convert_u8"]
        fn qvariantCanConvertU8(variant: &QVariant) -> bool;
        #[rust_name = "construct_u8"]
        fn qvariantConstruct(value: &u8) -> QVariant;
        #[rust_name = "value_u8"]
        fn qvariantValue(variant: &QVariant) -> u8;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_u8(variant)
}

pub(crate) fn construct(value: &u8) -> ffi::QVariant {
    ffi::construct_u8(value)
}

pub(crate) fn value(variant: &ffi::QVariant) -> u8 {
    ffi::value_u8(variant)
}
