// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = crate::QDateTime;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QDateTime"]
        fn qvariantCanConvertQDateTime(variant: &QVariant) -> bool;
        #[rust_name = "construct_QDateTime"]
        fn qvariantConstruct(value: &QDateTime) -> QVariant;
        #[rust_name = "value_QDateTime"]
        fn qvariantValue(variant: &QVariant) -> QDateTime;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QDateTime(variant)
}

pub(crate) fn construct(value: &ffi::QDateTime) -> ffi::QVariant {
    ffi::construct_QDateTime(value)
}

pub(crate) fn value(variant: &ffi::QVariant) -> ffi::QDateTime {
    ffi::value_QDateTime(variant)
}
