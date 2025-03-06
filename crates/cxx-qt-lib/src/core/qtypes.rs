// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qtypes.h");
    }
}

/// Typedef for long long int. This type is guaranteed to be 64-bit on all platforms supported by Qt.
#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub struct qint64(i64);

impl From<i64> for qint64 {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<qint64> for i64 {
    fn from(value: qint64) -> Self {
        value.0
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for qint64 {
    type Id = type_id!("qint64");
    type Kind = cxx::kind::Trivial;
}

/// Integral type for representing pointers in a signed integer (useful for hashing, etc.).
#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub struct qintptr(isize);

impl From<isize> for qintptr {
    fn from(value: isize) -> Self {
        qintptr(value)
    }
}

impl From<qintptr> for isize {
    fn from(value: qintptr) -> Self {
        value.0
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for qintptr {
    type Id = type_id!("qintptr");
    type Kind = cxx::kind::Trivial;
}

/// Typedef for double
///
/// Note that configuring Qt with -qreal float is not supported
#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct qreal(f64);

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
#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub struct quint64(u64);

impl From<u64> for quint64 {
    fn from(value: u64) -> Self {
        quint64(value)
    }
}

impl From<quint64> for u64 {
    fn from(value: quint64) -> Self {
        value.0
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for quint64 {
    type Id = type_id!("quint64");
    type Kind = cxx::kind::Trivial;
}

/// Integral type for representing pointers in an unsigned integer (useful for hashing, etc.).
#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub struct quintptr(usize);

impl From<usize> for quintptr {
    fn from(value: usize) -> Self {
        quintptr(value)
    }
}

impl From<quintptr> for usize {
    fn from(value: quintptr) -> Self {
        value.0
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for quintptr {
    type Id = type_id!("quintptr");
    type Kind = cxx::kind::Trivial;
}

/// Integral type providing Posix' ssize_t for all platforms.
///
/// This type is guaranteed to be the same size as a size_t on all platforms supported by Qt.
#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct qsizetype(isize);

impl From<isize> for qsizetype {
    fn from(value: isize) -> Self {
        qsizetype(value)
    }
}

impl From<qsizetype> for isize {
    fn from(value: qsizetype) -> Self {
        value.0
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for qsizetype {
    type Id = type_id!("qsizetype");
    type Kind = cxx::kind::Trivial;
}
