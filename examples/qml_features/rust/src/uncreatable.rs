// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a QML_UNCREATABLE QObject can be used

/// A CXX-Qt bridge which shows how a QML_UNCREATABLE QObject can be used
// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "rust_uncreatable")]
pub mod ffi {
    /// A QObject which is a QML_UNCREATABLE
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0", qml_uncreatable)]
    #[derive(Default)]
    #[qproperty(i32, value)]
    pub struct RustUncreatable {
        /// A value Q_PROPERTY
        value: i32,
    }
}
