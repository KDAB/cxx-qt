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
        type QMetaObjectConnection;

        #[doc(hidden)]
        #[rust_name = "qmetaobjectconnection_disconnect"]
        fn qmetaobjectconnectionDisconnect(conn: &QMetaObjectConnection);
    }

    impl UniquePtr<QMetaObjectConnection> {}
}

pub use ffi::QMetaObjectConnection;

impl QMetaObjectConnection {
    /// Disconnect a connection.
    pub fn disconnect(&self) {
        ffi::qmetaobjectconnection_disconnect(self);
    }
}
