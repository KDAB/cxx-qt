// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlinef.h");
        type QLineF = crate::QLineF;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QLineF"]
        fn qvariantCanConvertQLineF(variant: &QVariant) -> bool;
        #[rust_name = "construct_QLineF"]
        fn qvariantConstruct(value: &QLineF) -> QVariant;
        #[rust_name = "value_or_default_QLineF"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QLineF;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QLineF(variant)
}

pub(crate) fn construct(value: &ffi::QLineF) -> ffi::QVariant {
    ffi::construct_QLineF(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QLineF {
    ffi::value_or_default_QLineF(variant)
}
