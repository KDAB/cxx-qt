// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qlist/qlist_QObjectMutPtr.h");
        type QList_QObjectMutPtr = crate::QList<crate::QObjectMutPtr>;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QList_QObjectMutPtr"]
        fn qvariantCanConvertQList_QObjectMutPtr(variant: &QVariant) -> bool;
        #[rust_name = "construct_QList_QObjectMutPtr"]
        fn qvariantConstruct(value: &QList_QObjectMutPtr) -> QVariant;
        #[rust_name = "value_or_default_QList_QObjectMutPtr"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QList_QObjectMutPtr;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QList_QObjectMutPtr(variant)
}

pub(crate) fn construct(value: &ffi::QList_QObjectMutPtr) -> ffi::QVariant {
    ffi::construct_QList_QObjectMutPtr(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QList_QObjectMutPtr {
    ffi::value_or_default_QList_QObjectMutPtr(variant)
}
