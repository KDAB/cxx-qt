// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qregion.h");
        type QRegion = crate::QRegion;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QRegion"]
        fn qvariantCanConvertQRegion(variant: &QVariant) -> bool;
        #[rust_name = "construct_QRegion"]
        fn qvariantConstruct(value: &QRegion) -> QVariant;
        #[rust_name = "value_or_default_QRegion"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QRegion;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QRegion(variant)
}

pub(crate) fn construct(value: &ffi::QRegion) -> ffi::QVariant {
    ffi::construct_QRegion(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QRegion {
    ffi::value_or_default_QRegion(variant)
}
