// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = crate::QModelIndex;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QModelIndex"]
        fn qvariantCanConvertQModelIndex(variant: &QVariant) -> bool;
        #[rust_name = "construct_QModelIndex"]
        fn qvariantConstruct(value: &QModelIndex) -> QVariant;
        #[rust_name = "value_or_default_QModelIndex"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QModelIndex;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QModelIndex(variant)
}

pub(crate) fn construct(value: &ffi::QModelIndex) -> ffi::QVariant {
    ffi::construct_QModelIndex(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QModelIndex {
    ffi::value_or_default_QModelIndex(variant)
}
