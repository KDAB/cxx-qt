// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cargo_imports
mod cxxqt_object;
mod qml;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
// ANCHOR_END: book_cargo_imports

// ANCHOR: book_cargo_rust_main
fn main() {
    // Normally in a C++ program, global variables for the Qt Resource System are
    // initialized before the C++ main function runs. However, when building a
    // Rust executable with Cargo, the Qt Resource System needs to be initialized
    // manually.
    // https://doc.qt.io/qt-6/resources.html#explicit-loading-and-unloading-of-embedded-resources
    qml::ffi::q_init_resources();

    // Create the application and engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Call the C++ method to register QML types
    qml::ffi::register_types();

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/main.qml"));
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
// ANCHOR_END: book_cargo_rust_main
