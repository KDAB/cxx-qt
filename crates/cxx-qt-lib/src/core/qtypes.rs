// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
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

        #[cxx_name = "qintptr"]
        type QIntPtr = super::QIntPtr;

        #[cxx_name = "quint64"]
        type QUInt64 = super::QUInt64;

        #[cxx_name = "quintptr"]
        type QUIntPtr = super::QUIntPtr;

        #[cxx_name = "qsizetype"]
        type QSizeType = super::QSizeType;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[rust_name = "qint64_from_i64"]
        fn qint64FromI64(value: i64) -> QInt64;
        #[rust_name = "qint64_into_i64"]
        fn qint64IntoI64(value: QInt64) -> i64;

        #[rust_name = "qintptr_from_isize"]
        fn qintptrFromIsize(value: isize) -> QIntPtr;
        #[rust_name = "qintptr_into_isize"]
        fn qintptrIntoIsize(value: QIntPtr) -> isize;

        #[rust_name = "quint64_from_u64"]
        fn quint64FromU64(value: u64) -> QUInt64;
        #[rust_name = "quint64_into_u64"]
        fn quint64IntoU64(value: QUInt64) -> u64;

        #[rust_name = "quintptr_from_usize"]
        fn quintptrFromUsize(value: usize) -> QUIntPtr;
        #[rust_name = "quintptr_into_usize"]
        fn quintptrIntoUsize(value: QUIntPtr) -> usize;

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
pub struct QIntPtr {
    _space: MaybeUninit<isize>,
}

impl From<isize> for QIntPtr {
    fn from(value: isize) -> Self {
        ffi::qintptr_from_isize(value)
    }
}

impl From<QIntPtr> for isize {
    fn from(value: QIntPtr) -> Self {
        ffi::qintptr_into_isize(value)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QIntPtr {
    type Id = type_id!("qintptr");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
pub struct QUInt64 {
    _space: MaybeUninit<u64>,
}

impl From<u64> for QUInt64 {
    fn from(value: u64) -> Self {
        ffi::quint64_from_u64(value)
    }
}

impl From<QUInt64> for u64 {
    fn from(value: QUInt64) -> Self {
        ffi::quint64_into_u64(value)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QUInt64 {
    type Id = type_id!("quint64");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
pub struct QUIntPtr {
    _space: MaybeUninit<usize>,
}

impl From<usize> for QUIntPtr {
    fn from(value: usize) -> Self {
        ffi::quintptr_from_usize(value)
    }
}

impl From<QUIntPtr> for usize {
    fn from(value: QUIntPtr) -> Self {
        ffi::quintptr_into_usize(value)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QUIntPtr {
    type Id = type_id!("quintptr");
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
