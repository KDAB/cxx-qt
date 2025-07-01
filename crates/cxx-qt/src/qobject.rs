// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! QObject module

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!(<QtCore/QObject>);
        /// The `QObject` class is the base class of all Qt objects.
        ///
        /// Most methods available on this type are within the [`cxx_qt_lib::QObjectExt`] trait,
        /// which needs to be imported in order to access these.
        ///
        /// Qt Documentation: [QObject](https://doc.qt.io/qt/qobject.html#details)
        type QObject;

        #[cxx_name = "dumpObjectInfo"]
        /// Dumps information about signal connections, etc. for this object to the debug output.
        fn dump_object_info(&self);
    }
}

pub use ffi::QObject;
