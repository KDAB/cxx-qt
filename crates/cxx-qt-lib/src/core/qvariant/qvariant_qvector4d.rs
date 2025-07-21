// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvector4d.h");
        type QVector4D = crate::QVector4D;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QVector4D"]
        fn qvariantCanConvertQVector4D(variant: &QVariant) -> bool;
        #[rust_name = "construct_QVector4D"]
        fn qvariantConstruct(value: &QVector4D) -> QVariant;
        #[rust_name = "value_or_default_QVector4D"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QVector4D;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QVector4D(variant)
}

pub(crate) fn construct(value: &ffi::QVector4D) -> ffi::QVariant {
    ffi::construct_QVector4D(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QVector4D {
    ffi::value_or_default_QVector4D(variant)
}
