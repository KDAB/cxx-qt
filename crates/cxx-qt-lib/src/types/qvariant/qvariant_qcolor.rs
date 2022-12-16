// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QColor"]
        fn qvariantCanConvertQColor(variant: &QVariant) -> bool;
        #[rust_name = "construct_QColor"]
        fn qvariantConstruct(value: &QColor) -> QVariant;
        #[rust_name = "value_QColor"]
        fn qvariantValue(variant: &QVariant) -> QColor;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QColor(variant)
}

pub(crate) fn construct(value: &ffi::QColor) -> ffi::QVariant {
    ffi::construct_QColor(value)
}

pub(crate) fn value(variant: &ffi::QVariant) -> ffi::QColor {
    ffi::value_QColor(variant)
}
