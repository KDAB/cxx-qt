// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = crate::QPointF;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QPointF"]
        fn qvariantCanConvertQPointF(variant: &QVariant) -> bool;
        #[rust_name = "construct_QPointF"]
        fn qvariantConstruct(value: &QPointF) -> QVariant;
        #[rust_name = "value_or_default_QPointF"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QPointF;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QPointF(variant)
}

pub(crate) fn construct(value: &ffi::QPointF) -> ffi::QVariant {
    ffi::construct_QPointF(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QPointF {
    ffi::value_or_default_QPointF(variant)
}
