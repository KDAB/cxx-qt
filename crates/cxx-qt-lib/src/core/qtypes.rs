// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qtypes.h");

        #[cxx_name = "qint64"]
        type QInt64 = super::QInt64;

        #[cxx_name = "qsizetype"]
        type QSizeType = super::QSizeType;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[rust_name = "qint64_from_i64"]
        fn qint64FromI64(value: i64) -> QInt64;
        #[rust_name = "qint64_into_i64"]
        fn qint64IntoI64(value: QInt64) -> i64;

        #[rust_name = "qsizetype_from_isize"]
        fn qsizetypeFromIsize(value: isize) -> QSizeType;
        #[rust_name = "qsizetype_into_isize"]
        fn qsizetypeIntoIsize(value: QSizeType) -> isize;
    }
}

#[repr(C)]
pub struct QInt64 {
    _space: MaybeUninit<i64>,
}

impl From<i64> for QInt64 {
    fn from(value: i64) -> Self {
        ffi::qint64_from_i64(value)
    }
}

impl From<QInt64> for i64 {
    fn from(value: QInt64) -> Self {
        ffi::qint64_into_i64(value)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QInt64 {
    type Id = type_id!("qint64");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
pub struct QSizeType {
    _space: MaybeUninit<isize>,
}

impl From<isize> for QSizeType {
    fn from(value: isize) -> Self {
        ffi::qsizetype_from_isize(value)
    }
}

impl From<QSizeType> for isize {
    fn from(value: QSizeType) -> Self {
        ffi::qsizetype_into_isize(value)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QSizeType {
    type Id = type_id!("qsizetype");
    type Kind = cxx::kind::Trivial;
}
