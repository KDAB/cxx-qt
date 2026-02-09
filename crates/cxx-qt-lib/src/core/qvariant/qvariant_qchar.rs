// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qchar.h");
        type QChar = crate::QChar;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QChar"]
        fn qvariantCanConvertQChar(variant: &QVariant) -> bool;
        #[rust_name = "construct_QChar"]
        fn qvariantConstruct(value: &QChar) -> QVariant;
        #[rust_name = "value_or_default_QChar"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QChar;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QChar(variant)
}

pub(crate) fn construct(value: &ffi::QChar) -> ffi::QVariant {
    ffi::construct_QChar(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QChar {
    ffi::value_or_default_QChar(variant)
}
