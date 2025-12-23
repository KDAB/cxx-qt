// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qimage.h");
        type QImage = crate::QImage;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QImage"]
        fn qvariantCanConvertQImage(variant: &QVariant) -> bool;
        #[rust_name = "construct_QImage"]
        fn qvariantConstruct(value: &QImage) -> QVariant;
        #[rust_name = "value_or_default_QImage"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QImage;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QImage(variant)
}

pub(crate) fn construct(value: &ffi::QImage) -> ffi::QVariant {
    ffi::construct_QImage(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QImage {
    ffi::value_or_default_QImage(variant)
}
