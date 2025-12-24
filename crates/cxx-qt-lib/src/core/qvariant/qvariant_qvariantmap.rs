// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qmap/qmap_QString_QVariant.h");
        type QMap_QString_QVariant = crate::QMap<crate::QMapPair_QString_QVariant>;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QVariantMap"]
        fn qvariantCanConvertQVariantMap(variant: &QVariant) -> bool;
        #[rust_name = "construct_QVariantMap"]
        fn qvariantConstruct(value: &QMap_QString_QVariant) -> QVariant;
        #[rust_name = "value_or_default_QVariantMap"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QMap_QString_QVariant;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QVariantMap(variant)
}

pub(crate) fn construct(value: &ffi::QMap_QString_QVariant) -> ffi::QVariant {
    ffi::construct_QVariantMap(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QMap_QString_QVariant {
    ffi::value_or_default_QVariantMap(variant)
}
