// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = crate::QSizeF;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QSizeF"]
        fn qvariantCanConvertQSizeF(variant: &QVariant) -> bool;
        #[rust_name = "construct_QSizeF"]
        fn qvariantConstruct(value: &QSizeF) -> QVariant;
        #[rust_name = "value_or_default_QSizeF"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QSizeF;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QSizeF(variant)
}

pub(crate) fn construct(value: &ffi::QSizeF) -> ffi::QVariant {
    ffi::construct_QSizeF(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QSizeF {
    ffi::value_or_default_QSizeF(variant)
}
