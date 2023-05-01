// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QMetaObjectConnection;

#[cxx::bridge]
mod qmetaobjectconnection_cxx {
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmetaobjectconnection.h");
        type QMetaObjectConnection = cxx_qt_lib::QMetaObjectConnection;
    }

    extern "Rust" {
        fn qmetaobjectconnection_drop(conn: QMetaObjectConnection);
        fn qmetaobjectconnection_release(conn: QMetaObjectConnection);
    }
}

fn qmetaobjectconnection_drop(conn: QMetaObjectConnection) {
    drop(conn);
}

fn qmetaobjectconnection_release(conn: QMetaObjectConnection) {
    conn.release();
}
