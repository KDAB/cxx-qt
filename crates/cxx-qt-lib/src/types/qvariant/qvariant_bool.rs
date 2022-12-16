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
        #[rust_name = "can_convert_bool"]
        fn qvariantCanConvertBool(variant: &QVariant) -> bool;
        #[rust_name = "construct_bool"]
        fn qvariantConstruct(value: &bool) -> QVariant;
        #[rust_name = "value_bool"]
        fn qvariantValue(variant: &QVariant) -> bool;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_bool(variant)
}

pub(crate) fn construct(value: &bool) -> ffi::QVariant {
    ffi::construct_bool(value)
}

pub(crate) fn value(variant: &ffi::QVariant) -> bool {
    ffi::value_bool(variant)
}
