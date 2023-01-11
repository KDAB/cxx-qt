// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qurl.h");
        type QUrl = crate::QUrl;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QUrl"]
        fn qvariantCanConvertQUrl(variant: &QVariant) -> bool;
        #[rust_name = "construct_QUrl"]
        fn qvariantConstruct(value: &QUrl) -> QVariant;
        #[rust_name = "value_or_default_QUrl"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QUrl;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QUrl(variant)
}

pub(crate) fn construct(value: &ffi::QUrl) -> ffi::QVariant {
    ffi::construct_QUrl(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QUrl {
    ffi::value_or_default_QUrl(variant)
}
