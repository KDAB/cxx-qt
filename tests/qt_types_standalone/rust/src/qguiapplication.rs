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
    let app = QGuiApplication::new();
    QGuiApplication::set_application_name(&QString::from("kdab"));
    app
}

fn read_qguiapplication(_app: &QGuiApplication) -> bool {
    QGuiApplication::application_name().to_string() == "kdab"
}
