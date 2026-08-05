// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qjsonarray.h");
        type QJsonArray = crate::QJsonArray;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QJsonArray"]
        fn qvariantCanConvertQJsonArray(variant: &QVariant) -> bool;
        #[rust_name = "construct_QJsonArray"]
        fn qvariantConstruct(value: &QJsonArray) -> QVariant;
        #[rust_name = "value_or_default_QJsonArray"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QJsonArray;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QJsonArray(variant)
}

pub(crate) fn construct(value: &ffi::QJsonArray) -> ffi::QVariant {
    ffi::construct_QJsonArray(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QJsonArray {
    ffi::value_or_default_QJsonArray(variant)
}
