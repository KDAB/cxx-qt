// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qquaternion.h");
        type QQuaternion = crate::QQuaternion;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QQuaternion"]
        fn qvariantCanConvertQQuaternion(variant: &QVariant) -> bool;
        #[rust_name = "construct_QQuaternion"]
        fn qvariantConstruct(value: &QQuaternion) -> QVariant;
        #[rust_name = "value_or_default_QQuaternion"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QQuaternion;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QQuaternion(variant)
}

pub(crate) fn construct(value: &ffi::QQuaternion) -> ffi::QVariant {
    ffi::construct_QQuaternion(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QQuaternion {
    ffi::value_or_default_QQuaternion(variant)
}
