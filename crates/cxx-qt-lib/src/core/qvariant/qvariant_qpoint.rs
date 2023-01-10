// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QPoint"]
        fn qvariantCanConvertQPoint(variant: &QVariant) -> bool;
        #[rust_name = "construct_QPoint"]
        fn qvariantConstruct(value: &QPoint) -> QVariant;
        #[rust_name = "value_or_default_QPoint"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QPoint;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QPoint(variant)
}

pub(crate) fn construct(value: &ffi::QPoint) -> ffi::QVariant {
    ffi::construct_QPoint(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QPoint {
    ffi::value_or_default_QPoint(variant)
}
