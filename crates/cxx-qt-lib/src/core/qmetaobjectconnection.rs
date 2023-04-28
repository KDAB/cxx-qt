// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
mod ffi {
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmetaobjectconnection.h");

        /// Represents a handle to a signal-slot (or signal-functor) connection.
        ///
        /// Acts as a guard so when deconstructed the connection is disconnected
        type QMetaObjectConnectionGuard;

        /// Manually disconnect the signal
        fn disconnect(self: &QMetaObjectConnectionGuard);
        /// Release the connection so that it is not disconnected when deconstructed
        fn release(self: Pin<&mut QMetaObjectConnectionGuard>);
    }

    impl UniquePtr<QMetaObjectConnectionGuard> {}
}

pub use ffi::QMetaObjectConnectionGuard;
