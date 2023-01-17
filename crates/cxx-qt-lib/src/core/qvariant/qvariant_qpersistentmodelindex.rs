// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpersistentmodelindex.h");
        type QPersistentModelIndex = crate::QPersistentModelIndex;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QPersistentModelIndex"]
        fn qvariantCanConvertQPersistentModelIndex(variant: &QVariant) -> bool;
        #[rust_name = "construct_QPersistentModelIndex"]
        fn qvariantConstruct(value: &QPersistentModelIndex) -> QVariant;
        #[rust_name = "value_or_default_QPersistentModelIndex"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QPersistentModelIndex;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QPersistentModelIndex(variant)
}

pub(crate) fn construct(value: &ffi::QPersistentModelIndex) -> ffi::QVariant {
    ffi::construct_QPersistentModelIndex(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QPersistentModelIndex {
    ffi::value_or_default_QPersistentModelIndex(variant)
}
