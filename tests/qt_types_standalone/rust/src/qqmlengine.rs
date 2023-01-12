// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QQmlEngine, QUrl};

#[cxx::bridge]
mod qqmlengine_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qqmlengine.h");
        type QQmlEngine = cxx_qt_lib::QQmlEngine;
    }

    extern "Rust" {
        fn construct_qqmlengine() -> UniquePtr<QQmlEngine>;
        fn read_qqmlengine(c: &QQmlEngine) -> bool;
    }
}

fn construct_qqmlengine() -> cxx::UniquePtr<QQmlEngine> {
    let mut engine = QQmlEngine::new();
    if let Some(engine) = engine.as_mut() {
        engine.set_base_url(&QUrl::from("qrc:/kdab.qml"));
    }
    engine
}

fn read_qqmlengine(engine: &QQmlEngine) -> bool {
    engine.base_url().to_string() == "qrc:/kdab.qml"
}
