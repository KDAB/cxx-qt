// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qsize.h");
        type QSize = crate::QSize;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QSize"]
        fn qvariantCanConvertQSize(variant: &QVariant) -> bool;
        #[rust_name = "construct_QSize"]
        fn qvariantConstruct(value: &QSize) -> QVariant;
        #[rust_name = "value_or_default_QSize"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QSize;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QSize(variant)
}

pub(crate) fn construct(value: &ffi::QSize) -> ffi::QVariant {
    ffi::construct_QSize(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QSize {
    ffi::value_or_default_QSize(variant)
}
