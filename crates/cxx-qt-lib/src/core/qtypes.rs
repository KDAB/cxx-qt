// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::ops::{Deref, DerefMut};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qtypes.h");
    }
}

/// Typedef for long long int. This type is guaranteed to be 64-bit on all platforms supported by Qt.
#[repr(transparent)]
#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct QInt64(i64);

impl Deref for QInt64 {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for QInt64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<i64> for QInt64 {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<QInt64> for i64 {
    fn from(value: QInt64) -> Self {
        value.0
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QInt64 {
    type Id = type_id!("qint64");
    type Kind = cxx::kind::Trivial;
}

/// Integral type for representing pointers in a signed integer (useful for hashing, etc.).
#[repr(transparent)]
#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct QIntPtr(isize);

impl Deref for QIntPtr {
    type Target = isize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for QIntPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<isize> for QIntPtr {
    fn from(value: isize) -> Self {
        QIntPtr(value)
    }
}

impl From<QIntPtr> for isize {
    fn from(value: QIntPtr) -> Self {
        value.0
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QIntPtr {
    type Id = type_id!("qintptr");
    type Kind = cxx::kind::Trivial;
}

/// Typedef for double
///
/// Note that configuring Qt with -qreal float is not supported
#[repr(transparent)]
#[derive(Default, Debug, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct qreal(f64);

impl Deref for qreal {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for qreal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<f64> for qreal {
    fn from(value: f64) -> Self {
        qreal(value)
    }
}

impl From<qreal> for f64 {
    fn from(value: qreal) -> Self {
        value.0
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for qreal {
    type Id = type_id!("qreal");
    type Kind = cxx::kind::Trivial;
}

/// Typedef for unsigned long long int. This type is guaranteed to be 64-bit on all platforms supported by Qt.
#[repr(transparent)]
#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct QUInt64(u64);

impl Deref for QUInt64 {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for QUInt64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u64> for QUInt64 {
    fn from(value: u64) -> Self {
        QUInt64(value)
    }
}

impl From<QUInt64> for u64 {
    fn from(value: QUInt64) -> Self {
        value.0
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QUInt64 {
    type Id = type_id!("quint64");
    type Kind = cxx::kind::Trivial;
}

/// Integral type for representing pointers in an unsigned integer (useful for hashing, etc.).
#[repr(transparent)]
#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct QUIntPtr(usize);

impl Deref for QUIntPtr {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for QUIntPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<usize> for QUIntPtr {
    fn from(value: usize) -> Self {
        QUIntPtr(value)
    }
}

impl From<QUIntPtr> for usize {
    fn from(value: QUIntPtr) -> Self {
        value.0
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QUIntPtr {
    type Id = type_id!("quintptr");
    type Kind = cxx::kind::Trivial;
}

/// Integral type providing Posix' ssize_t for all platforms.
///
/// This type is guaranteed to be the same size as a size_t on all platforms supported by Qt.
#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct QSizeType(isize);

impl Deref for QSizeType {
    type Target = isize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for QSizeType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<isize> for QSizeType {
    fn from(value: isize) -> Self {
        QSizeType(value)
    }
}

impl From<QSizeType> for isize {
    fn from(value: QSizeType) -> Self {
        value.0
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QSizeType {
    type Id = type_id!("qsizetype");
    type Kind = cxx::kind::Trivial;
}
