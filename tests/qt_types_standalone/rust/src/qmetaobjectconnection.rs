// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QMetaObjectConnection, QMetaObjectConnectionGuard};

#[cxx::bridge]
mod qmetaobjectconnection_cxx {
    #[namespace = "rust::cxxqt1"] // note that QMetaObjectConnection is reexported
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmetaobjectconnection.h");
        type QMetaObjectConnection = cxx_qt_lib::QMetaObjectConnection;
    }

    extern "Rust" {
        type QMetaObjectConnectionGuardWrapper;

        fn create_qmetaobjectconnectionguard(
            conn: QMetaObjectConnection,
        ) -> Box<QMetaObjectConnectionGuardWrapper>;

        fn qmetaobjectconnection_disconnect(conn: &QMetaObjectConnection) -> bool;
        fn qmetaobjectconnection_drop(wrapper: &mut QMetaObjectConnectionGuardWrapper);
        fn qmetaobjectconnection_release(
            wrapper: &mut QMetaObjectConnectionGuardWrapper,
        ) -> QMetaObjectConnection;
    }
}

// CXX doesn't support Rust alias so we need to have a new type
struct QMetaObjectConnectionGuardWrapper {
    guard: Option<QMetaObjectConnectionGuard>,
}

fn create_qmetaobjectconnectionguard(
    conn: QMetaObjectConnection,
) -> Box<QMetaObjectConnectionGuardWrapper> {
    Box::new(QMetaObjectConnectionGuardWrapper {
        guard: Some(QMetaObjectConnectionGuard::from(conn)),
    })
}

fn qmetaobjectconnection_disconnect(conn: &QMetaObjectConnection) -> bool {
    conn.disconnect()
}

fn qmetaobjectconnection_drop(wrapper: &mut QMetaObjectConnectionGuardWrapper) {
    if let Some(guard) = wrapper.guard.take() {
        drop(guard);
    }
}

fn qmetaobjectconnection_release(
    wrapper: &mut QMetaObjectConnectionGuardWrapper,
) -> QMetaObjectConnection {
    if let Some(guard) = wrapper.guard.take() {
        guard.release()
    } else {
        unreachable!();
    }
}
