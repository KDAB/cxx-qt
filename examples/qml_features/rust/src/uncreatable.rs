// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a QML_UNCREATABLE QObject can be used

/// A CXX-Qt bridge which shows how a QML_UNCREATABLE QObject can be used
// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "rust_uncreatable")]
pub mod ffi {
    extern "RustQt" {
        #[cxx_qt::qobject(qml_element, qml_uncreatable)]
        #[qproperty(i32, value)]
        type RustUncreatable = super::RustUncreatableRust;
    }
}

/// A QObject which is a QML_UNCREATABLE
#[derive(Default)]
pub struct RustUncreatableRust {
    /// A value Q_PROPERTY
    value: i32,
}
