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
        #[rust_name = "can_convert_i64"]
        fn qvariantCanConvertI64(variant: &QVariant) -> bool;
        #[rust_name = "construct_i64"]
        fn qvariantConstruct(value: &i64) -> QVariant;
        #[rust_name = "value_i64"]
        fn qvariantValue(variant: &QVariant) -> i64;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_i64(variant)
}

pub(crate) fn construct(value: &i64) -> ffi::QVariant {
    ffi::construct_i64(value)
}

pub(crate) fn value(variant: &ffi::QVariant) -> i64 {
    ffi::value_i64(variant)
}
