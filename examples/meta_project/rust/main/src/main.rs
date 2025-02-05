// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

extern crate qml_meta_project;
// mod main_object;
// extern crate sub1;
// extern crate sub2;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    // extern "C" {
    //     fn cxx_qt_init_crate_qml_meta_project() -> bool;
    // }

    // extern "C" {
    //     fn cxx_qt_init_qml_module_com_kdab_cxx_qt_demo() -> bool;
    // }

    // unsafe {
    //     cxx_qt_init_crate_qml_meta_project();
    //     cxx_qt_init_qml_module_com_kdab_cxx_qt_demo();
    // }

    // Create the application and engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/kdab/cxx_qt/demo/qml/main.qml"));
    }

    if let Some(engine) = engine.as_mut() {
        // Listen to a signal from the QML Engine
        engine
            .as_qqmlengine()
            .on_quit(|_| {
                println!("QML Quit!");
            })
            .release();
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
