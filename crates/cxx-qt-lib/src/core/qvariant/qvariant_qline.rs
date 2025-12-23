// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qline.h");
        type QLine = crate::QLine;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QLine"]
        fn qvariantCanConvertQLine(variant: &QVariant) -> bool;
        #[rust_name = "construct_QLine"]
        fn qvariantConstruct(value: &QLine) -> QVariant;
        #[rust_name = "value_or_default_QLine"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QLine;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QLine(variant)
}

pub(crate) fn construct(value: &ffi::QLine) -> ffi::QVariant {
    ffi::construct_QLine(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QLine {
    ffi::value_or_default_QLine(variant)
}
