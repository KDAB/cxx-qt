// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qtime.h");
        type QTime = crate::QTime;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QTime"]
        fn qvariantCanConvertQTime(variant: &QVariant) -> bool;
        #[rust_name = "construct_QTime"]
        fn qvariantConstruct(value: &QTime) -> QVariant;
        #[rust_name = "value_or_default_QTime"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QTime;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QTime(variant)
}

pub(crate) fn construct(value: &ffi::QTime) -> ffi::QVariant {
    ffi::construct_QTime(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QTime {
    ffi::value_or_default_QTime(variant)
}
