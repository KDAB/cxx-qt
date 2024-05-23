// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// Find CXX-Qt initialise method when we link at the end
extern "C" {
    fn __cxx_qt_init();
}

/// Initialise any static initialisers needed by CXX-Qt
/// - `Q_IMPORT_PLUGIN` QML plugins defined in the build.rs
/// - `Q_INIT_RESOURCE` qrc paths defined in the build.rs
/// - Register types with Qt that are required
pub fn init() {
    unsafe {
        __cxx_qt_init();
    }
}
