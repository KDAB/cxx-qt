// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QRect"]
        fn qvariantCanConvertQRect(variant: &QVariant) -> bool;
        #[rust_name = "construct_QRect"]
        fn qvariantConstruct(value: &QRect) -> QVariant;
        #[rust_name = "value_or_default_QRect"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QRect;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QRect(variant)
}

pub(crate) fn construct(value: &ffi::QRect) -> ffi::QVariant {
    ffi::construct_QRect(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QRect {
    ffi::value_or_default_QRect(variant)
}
