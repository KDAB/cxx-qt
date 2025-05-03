// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::marker::PhantomData;

use crate::QMetaObjectConnection;

/// Represents a guard to a signal-slot (or signal-functor) connection.
///
/// This struct can be created from a [QMetaObjectConnection].
///
/// Note that when this struct is dropped the connection is disconnected.
/// So to keep a connection active either hold onto the struct for the duration
/// that the connection should be active or call `release`, hence the `#[must_use]`.
#[must_use]
pub struct QMetaObjectConnectionGuard<'a> {
    connection: QMetaObjectConnection,
    _phantom: PhantomData<&'a ()>,
}

impl From<QMetaObjectConnection> for QMetaObjectConnectionGuard<'static> {
    fn from(connection: QMetaObjectConnection) -> Self {
        Self {
            connection,
            _phantom: PhantomData,
        }
    }
}

impl Drop for QMetaObjectConnectionGuard<'_> {
    /// Disconnect and deconstruct the connection
    fn drop(&mut self) {
        self.connection.disconnect();
    }
}

impl QMetaObjectConnectionGuard<'static> {
    /// Release the connection without disconnecting
    pub fn release(mut self) -> QMetaObjectConnection {
        // Take the connection as our Drop implementation disconnects automatically
        // whereas we just want to release
        core::mem::take(&mut self.connection)
    }
}
