// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QCoreApplication, QString};

#[cxx::bridge]
mod qcoreapplication_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcoreapplication.h");
        type QCoreApplication = cxx_qt_lib::QCoreApplication;
    }

    extern "Rust" {
        fn construct_qcoreapplication() -> UniquePtr<QCoreApplication>;
        fn read_qcoreapplication(c: &QCoreApplication) -> bool;
    }
}

fn construct_qcoreapplication() -> cxx::UniquePtr<QCoreApplication> {
    let mut app = QCoreApplication::new();
    if let Some(app) = app.as_mut() {
        app.set_application_name(&QString::from("kdab"));
    }
    app
}

fn read_qcoreapplication(app: &QCoreApplication) -> bool {
    app.application_name().to_string() == "kdab"
}
