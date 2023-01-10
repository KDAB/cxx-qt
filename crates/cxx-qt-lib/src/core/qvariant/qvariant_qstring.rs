// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QString"]
        fn qvariantCanConvertQString(variant: &QVariant) -> bool;
        #[rust_name = "construct_QString"]
        fn qvariantConstruct(value: &QString) -> QVariant;
        #[rust_name = "value_or_default_QString"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QString;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QString(variant)
}

pub(crate) fn construct(value: &ffi::QString) -> ffi::QVariant {
    ffi::construct_QString(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QString {
    ffi::value_or_default_QString(variant)
}
