// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpen.h");
        type QPen = crate::QPen;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QPen"]
        fn qvariantCanConvertQPen(variant: &QVariant) -> bool;
        #[rust_name = "construct_QPen"]
        fn qvariantConstruct(value: &QPen) -> QVariant;
        #[rust_name = "value_or_default_QPen"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QPen;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QPen(variant)
}

pub(crate) fn construct(value: &ffi::QPen) -> ffi::QVariant {
    ffi::construct_QPen(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QPen {
    ffi::value_or_default_QPen(variant)
}
