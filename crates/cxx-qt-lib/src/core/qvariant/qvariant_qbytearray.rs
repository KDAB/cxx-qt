// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QByteArray"]
        fn qvariantCanConvertQByteArray(variant: &QVariant) -> bool;
        #[rust_name = "construct_QByteArray"]
        fn qvariantConstruct(value: &QByteArray) -> QVariant;
        #[rust_name = "value_or_default_QByteArray"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QByteArray;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QByteArray(variant)
}

pub(crate) fn construct(value: &ffi::QByteArray) -> ffi::QVariant {
    ffi::construct_QByteArray(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QByteArray {
    ffi::value_or_default_QByteArray(variant)
}
