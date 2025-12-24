// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qlist/qlist_QVariant.h");
        type QList_QVariant = crate::QList<crate::QVariant>;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QVariantList"]
        fn qvariantCanConvertQVariantList(variant: &QVariant) -> bool;
        #[rust_name = "construct_QVariantList"]
        fn qvariantConstruct(value: &QList_QVariant) -> QVariant;
        #[rust_name = "value_or_default_QVariantList"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QList_QVariant;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QVariantList(variant)
}

pub(crate) fn construct(value: &ffi::QList_QVariant) -> ffi::QVariant {
    ffi::construct_QVariantList(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QList_QVariant {
    ffi::value_or_default_QVariantList(variant)
}
