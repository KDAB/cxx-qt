// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::marker::PhantomData;

use crate::QMetaObjectConnection;

/// Represents a scoped guard to a signal-slot (or signal-functor) connection.
///
/// This struct can be created from a [`QMetaObjectConnection`].
///
/// Note that when this struct is dropped the connection is disconnected.
/// So to keep a connection active hold onto the struct for the duration
/// that the connection should be active, hence the `#[must_use]`.
#[must_use = "If unused the connection will immediately be dropped"]
pub struct QScopedMetaObjectConnectionGuard<'a> {
    connection: QMetaObjectConnection,
    _phantom: PhantomData<&'a ()>,
}

/// Represents a static guard to a signal-slot (or signal-functor) connection.
///
/// This struct can be created from a [`QMetaObjectConnection`].
///
/// Note that when this struct is dropped the connection is disconnected.
/// So to keep a connection active either hold onto the struct for the duration
/// that the connection should be active or call [`release`](Self::release), hence the `#[must_use]`.
pub type QMetaObjectConnectionGuard = QScopedMetaObjectConnectionGuard<'static>;

impl From<QMetaObjectConnection> for QScopedMetaObjectConnectionGuard<'static> {
    fn from(connection: QMetaObjectConnection) -> Self {
        Self {
            connection,
            _phantom: PhantomData,
        }
    }
}

impl Drop for QScopedMetaObjectConnectionGuard<'_> {
    /// Disconnect and deconstruct the connection.
    fn drop(&mut self) {
        self.connection.disconnect();
    }
}

impl QScopedMetaObjectConnectionGuard<'static> {
    /// Release the connection without disconnecting.
    pub fn release(mut self) -> QMetaObjectConnection {
        // Take the connection as our Drop implementation disconnects automatically
        // whereas we just want to release
        core::mem::take(&mut self.connection)
    }
}
