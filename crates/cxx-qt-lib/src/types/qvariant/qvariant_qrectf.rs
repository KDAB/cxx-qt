// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qrectf.h");
        type QRectF = crate::QRectF;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QRectF"]
        fn qvariantCanConvertQRectF(variant: &QVariant) -> bool;
        #[rust_name = "construct_QRectF"]
        fn qvariantConstruct(value: &QRectF) -> QVariant;
        #[rust_name = "value_QRectF"]
        fn qvariantValue(variant: &QVariant) -> QRectF;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QRectF(variant)
}

pub(crate) fn construct(value: &ffi::QRectF) -> ffi::QVariant {
    ffi::construct_QRectF(value)
}

pub(crate) fn value(variant: &ffi::QVariant) -> ffi::QRectF {
    ffi::value_QRectF(variant)
}
