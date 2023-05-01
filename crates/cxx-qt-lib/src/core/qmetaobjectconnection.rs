// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmetaobjectconnection.h");

        #[doc(hidden)]
        type QMetaObjectConnection = crate::QMetaObjectConnection;

        #[doc(hidden)]
        #[rust_name = "qmetaobjectconnection_disconnect"]
        fn qmetaobjectconnectionDisconnect(conn: &QMetaObjectConnection);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qmetaobjectconnection_drop"]
        fn drop(conn: &mut QMetaObjectConnection);
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

impl Drop for QMetaObjectConnection {
    /// Disconnect and deconstruct the [QMetaObjectConnection]
    fn drop(&mut self) {
        ffi::qmetaobjectconnection_disconnect(self);
        ffi::qmetaobjectconnection_drop(self);
    }
}

impl QMetaObjectConnection {
    /// Release the [QMetaObjectConnection] without disconnecting
    pub fn release(mut self) {
        // Manually call drop in C++ and then forget
        //
        // As our Drop implementation disconnects automatically whereas we just want to release
        ffi::qmetaobjectconnection_drop(&mut self);
        std::mem::forget(self);
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QMetaObjectConnection {
    type Id = type_id!("rust::cxxqtlib1::QMetaObjectConnection");
    type Kind = cxx::kind::Trivial;
}
