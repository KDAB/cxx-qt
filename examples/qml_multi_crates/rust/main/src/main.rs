// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

extern crate qml_multi_crates;

use cxx_qt::Upcast;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    cxx_qt::init_crate!(qml_multi_crates);

    // Create the application and engine
    let app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/kdab/cxx_qt/demo/qml/main.qml"));
    }

    if let Some(engine) = engine.as_mut() {
        // Listen to a signal from the QML Engine
        engine
            .upcast_pin()
            .on_quit(|_| {
                println!("QML Quit!");
            })
            .release();
    }

    app.exec();
}

#[cfg(test)]
mod tests {
    // In the test cfg there needs to be at least one test that calls the crate initialization.
    // Otherwise linking will fail!
    #[test]
    fn init_dependencies() {
        cxx_qt::init_crate!(qml_multi_crates);
        cxx_qt::init_qml_module!("com.kdab.cxx_qt.demo");
    }
}
