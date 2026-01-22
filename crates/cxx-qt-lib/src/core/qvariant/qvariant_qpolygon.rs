// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpolygon.h");
        type QPolygon = crate::QPolygon;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QPolygon"]
        fn qvariantCanConvertQPolygon(variant: &QVariant) -> bool;
        #[rust_name = "construct_QPolygon"]
        fn qvariantConstruct(value: &QPolygon) -> QVariant;
        #[rust_name = "value_or_default_QPolygon"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QPolygon;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QPolygon(variant)
}

pub(crate) fn construct(value: &ffi::QPolygon) -> ffi::QVariant {
    ffi::construct_QPolygon(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QPolygon {
    ffi::value_or_default_QPolygon(variant)
}
