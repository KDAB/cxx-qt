// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qhash/qhash_QString_QVariant.h");
        type QHash_QString_QVariant = crate::QHash<crate::QHashPair_QString_QVariant>;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QVariantHash"]
        fn qvariantCanConvertQVariantHash(variant: &QVariant) -> bool;
        #[rust_name = "construct_QVariantHash"]
        fn qvariantConstruct(value: &QHash_QString_QVariant) -> QVariant;
        #[rust_name = "value_or_default_QVariantHash"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QHash_QString_QVariant;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QVariantHash(variant)
}

pub(crate) fn construct(value: &ffi::QHash_QString_QVariant) -> ffi::QVariant {
    ffi::construct_QVariantHash(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QHash_QString_QVariant {
    ffi::value_or_default_QVariantHash(variant)
}
