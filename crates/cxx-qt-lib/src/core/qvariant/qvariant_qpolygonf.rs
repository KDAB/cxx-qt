// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpolygonf.h");
        type QPolygonF = crate::QPolygonF;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QPolygonF"]
        fn qvariantCanConvertQPolygonF(variant: &QVariant) -> bool;
        #[rust_name = "construct_QPolygonF"]
        fn qvariantConstruct(value: &QPolygonF) -> QVariant;
        #[rust_name = "value_or_default_QPolygonF"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QPolygonF;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QPolygonF(variant)
}

pub(crate) fn construct(value: &ffi::QPolygonF) -> ffi::QVariant {
    ffi::construct_QPolygonF(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QPolygonF {
    ffi::value_or_default_QPolygonF(variant)
}
