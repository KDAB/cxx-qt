// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qlist/qlist_u32.h");
        type QList_u32 = crate::QList<u32>;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QList_u32"]
        fn qvariantCanConvertQList_u32(variant: &QVariant) -> bool;
        #[rust_name = "construct_QList_u32"]
        fn qvariantConstruct(value: &QList_u32) -> QVariant;
        #[rust_name = "value_or_default_QList_u32"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QList_u32;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QList_u32(variant)
}

pub(crate) fn construct(value: &ffi::QList_u32) -> ffi::QVariant {
    ffi::construct_QList_u32(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QList_u32 {
    ffi::value_or_default_QList_u32(variant)
}
