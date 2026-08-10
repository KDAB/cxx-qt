// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qjsonobject.h");
        type QJsonObject = crate::QJsonObject;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QJsonObject"]
        fn qvariantCanConvertQJsonObject(variant: &QVariant) -> bool;
        #[rust_name = "construct_QJsonObject"]
        fn qvariantConstruct(value: &QJsonObject) -> QVariant;
        #[rust_name = "value_or_default_QJsonObject"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QJsonObject;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QJsonObject(variant)
}

pub(crate) fn construct(value: &ffi::QJsonObject) -> ffi::QVariant {
    ffi::construct_QJsonObject(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QJsonObject {
    ffi::value_or_default_QJsonObject(variant)
}
