// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qlist/qlist_i64.h");
        type QList_i64 = crate::QList<i64>;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QList_i64"]
        fn qvariantCanConvertQList_i64(variant: &QVariant) -> bool;
        #[rust_name = "construct_QList_i64"]
        fn qvariantConstruct(value: &QList_i64) -> QVariant;
        #[rust_name = "value_or_default_QList_i64"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QList_i64;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QList_i64(variant)
}

pub(crate) fn construct(value: &ffi::QList_i64) -> ffi::QVariant {
    ffi::construct_QList_i64(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QList_i64 {
    ffi::value_or_default_QList_i64(variant)
}
