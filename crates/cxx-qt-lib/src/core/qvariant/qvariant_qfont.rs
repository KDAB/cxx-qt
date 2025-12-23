// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qfont.h");
        type QFont = crate::QFont;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QFont"]
        fn qvariantCanConvertQFont(variant: &QVariant) -> bool;
        #[rust_name = "construct_QFont"]
        fn qvariantConstruct(value: &QFont) -> QVariant;
        #[rust_name = "value_or_default_QFont"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QFont;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QFont(variant)
}

pub(crate) fn construct(value: &ffi::QFont) -> ffi::QVariant {
    ffi::construct_QFont(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QFont {
    ffi::value_or_default_QFont(variant)
}
