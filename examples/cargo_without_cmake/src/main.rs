// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example provides demostrations of building a Cargo only CXX-Qt application

// Use this crate to test that missing_docs works with our generated code for a Cargo only build
#![deny(missing_docs)]

/// A module for our Rust defined QObject
// ANCHOR: book_cargo_imports
pub mod cxxqt_object;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
// ANCHOR_END: book_cargo_imports

// ANCHOR: book_cargo_rust_main
fn main() {
    // Create the application and engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/kdab/cxx_qt/demo/qml/main.qml"));
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
// ANCHOR_END: book_cargo_rust_main
