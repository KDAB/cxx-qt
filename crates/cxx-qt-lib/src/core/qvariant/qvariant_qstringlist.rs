// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QStringList"]
        fn qvariantCanConvertQStringList(variant: &QVariant) -> bool;
        #[rust_name = "construct_QStringList"]
        fn qvariantConstruct(value: &QStringList) -> QVariant;
        #[rust_name = "value_or_default_QStringList"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QStringList;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QStringList(variant)
}

pub(crate) fn construct(value: &ffi::QStringList) -> ffi::QVariant {
    ffi::construct_QStringList(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QStringList {
    ffi::value_or_default_QStringList(variant)
}
