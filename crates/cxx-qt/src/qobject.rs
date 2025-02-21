// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! QObject module

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        // TODO! Implement QObject wrapper properly
        include!(<QtCore/QObject>);
        /// QObject type
        type QObject;

        #[cxx_name = "dumpObjectInfo"]
        /// Dump Object info method, added so that upcast methods can be tested.
        fn dump_object_info(&self);
    }
}

pub use ffi::QObject;
