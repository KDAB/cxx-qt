// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! QObject module

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!(<QtCore/QObject>);
        /// QObject type.
        ///
        /// Most methods available on this type are within the [cxx_qt_lib::core::QObjectExt] trait,
        /// which needs to be imported in order to access these.
        type QObject;
    }
}

pub use ffi::QObject;
