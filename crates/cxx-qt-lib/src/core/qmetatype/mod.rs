// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(cxxqt_qt_version_major = "6")]
mod qmetatypetype_v6;
#[cfg(cxxqt_qt_version_major = "6")]
pub use qmetatypetype_v6::QMetaTypeType;

#[cfg(cxxqt_qt_version_major = "5")]
mod qmetatypetype_v5;
#[cfg(cxxqt_qt_version_major = "5")]
pub use qmetatypetype_v5::QMetaTypeType;

impl From<i32> for QMetaTypeType {
    fn from(value: i32) -> Self {
        Self { repr: value }
    }
}
impl From<QMetaTypeType> for i32 {
    fn from(value: QMetaTypeType) -> Self {
        value.repr
    }
}
