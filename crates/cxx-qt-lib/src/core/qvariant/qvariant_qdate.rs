// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qdate.h");
        type QDate = crate::QDate;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QDate"]
        fn qvariantCanConvertQDate(variant: &QVariant) -> bool;
        #[rust_name = "construct_QDate"]
        fn qvariantConstruct(value: &QDate) -> QVariant;
        #[rust_name = "value_or_default_QDate"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QDate;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QDate(variant)
}

pub(crate) fn construct(value: &ffi::QDate) -> ffi::QVariant {
    ffi::construct_QDate(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QDate {
    ffi::value_or_default_QDate(variant)
}
