// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QGuiApplication, QString};

#[cxx::bridge]
mod qguiapplication_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qguiapplication.h");
        type QGuiApplication = cxx_qt_lib::QGuiApplication;
    }

    extern "Rust" {
        fn construct_qguiapplication() -> UniquePtr<QGuiApplication>;
        fn read_qguiapplication(c: &QGuiApplication) -> bool;
    }
}

fn construct_qguiapplication() -> cxx::UniquePtr<QGuiApplication> {
    let mut app = QGuiApplication::new();
    if let Some(app) = app.as_mut() {
        app.set_application_name(&QString::from("kdab"));
    }
    app
}

fn read_qguiapplication(app: &QGuiApplication) -> bool {
    app.application_name().to_string() == "kdab"
}
