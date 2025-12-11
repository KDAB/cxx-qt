// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_build_rs
use cxx_qt_build::{CxxQtBuilder, PluginType, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(
        // For this example, the main.qml will be part of the main binary, not the plugin.
        // The import in main.qml will then trigger loading the QML plugin
        QmlModule::new("com.kdab.cxx_qt.demo").plugin_type(PluginType::Dynamic),
    )
    .files(["src/cxxqt_object.rs"])
    .build();
}
// ANCHOR_END: book_build_rs
