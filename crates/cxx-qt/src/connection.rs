// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    #[namespace = "rust::cxxqt1"]
    unsafe extern "C++" {
        include!("cxx-qt/connection.h");

        #[doc(hidden)]
        type QMetaObjectConnection = crate::QMetaObjectConnection;

        #[doc(hidden)]
        #[rust_name = "qmetaobjectconnection_default"]
        fn qmetaobjectconnectionDefault() -> QMetaObjectConnection;

        #[doc(hidden)]
        #[rust_name = "qmetaobjectconnection_disconnect"]
        fn qmetaobjectconnectionDisconnect(connection: &QMetaObjectConnection) -> bool;

        #[doc(hidden)]
        #[rust_name = "qmetaobjectconnection_drop"]
        fn qmetaobjectconnectionDrop(connection: &mut QMetaObjectConnection);
    }

    /// This enum describes the types of connection that can be used with signals.
    ///
    /// Note that UniqueConnection is not supported.
    #[namespace = "Qt"]
    #[repr(i32)]
    enum ConnectionType {
        /// If the receiver lives in the thread that emits the signal, Qt::DirectConnection is used.
        /// Otherwise, Qt::QueuedConnection is used. The connection type is determined when the signal is emitted.
        AutoConnection,
        /// The slot is invoked immediately when the signal is emitted.
        /// The slot is executed in the signalling thread.
        DirectConnection,
        /// The slot is invoked when control returns to the event loop of the receiver's thread.
        /// The slot is executed in the receiver's thread.
        QueuedConnection,
        /// Same as Qt::QueuedConnection, except that the signalling thread blocks until the slot returns.
        /// This connection must not be used if the receiver lives in the signalling thread, or else the application will deadlock.
        BlockingQueuedConnection,
    }

    // We need to tell CXX that the type already exists, otherwise the following error ocucrs
    // "scoped/unscoped mismatch in enum"
    #[namespace = "Qt"]
    unsafe extern "C++" {
        type ConnectionType;
    }
}

/// Represents a handle to a signal-slot (or signal-functor) connection.
///
/// This struct is returned when a connection is made using `on_SIGNAL_NAME`.
///
/// Note that when this struct is dropped the connection is disconnected.
/// So so keep a connection active either hold onto the struct for the duration
/// that the connection should be active or call `release`.
#[repr(C)]
pub struct QMetaObjectConnection {
    _space: MaybeUninit<usize>,
}

impl Default for QMetaObjectConnection {
    fn default() -> Self {
        ffi::qmetaobjectconnection_default()
    }
}

impl Drop for QMetaObjectConnection {
    fn drop(&mut self) {
        ffi::qmetaobjectconnection_drop(self);
    }
}

impl QMetaObjectConnection {
    /// Disconnect the signal
    pub fn disconnect(&self) -> bool {
        ffi::qmetaobjectconnection_disconnect(self)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QMetaObjectConnection {
    type Id = type_id!("rust::cxxqt1::QMetaObjectConnection");
    type Kind = cxx::kind::Trivial;
}

pub use ffi::ConnectionType;
