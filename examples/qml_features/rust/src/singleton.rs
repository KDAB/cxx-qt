// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a QML_SINGLETON can be used

/// A CXX-Qt bridge which shows how a QML_SINGLETON can be used
// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "rust_singleton")]
pub mod ffi {
    /// A QObject which is a QML_SINGLETON
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0", qml_singleton)]
    #[derive(Default)]
    pub struct RustSingleton {
        /// A Q_PROPERTY with a persistent value
        #[qproperty]
        persistent_value: i32,
    }

    unsafe extern "RustQt" {
        /// Increment the persistent value Q_PROPERTY of the QML_SINGLETON
        #[qinvokable]
        fn increment(self: Pin<&mut qobject::RustSingleton>);
    }
}

use core::pin::Pin;

// TODO: this will change to qobject::RustSingleton once
// https://github.com/KDAB/cxx-qt/issues/559 is done
impl ffi::RustSingletonQt {
    /// Increment the persistent value Q_PROPERTY of the QML_SINGLETON
    fn increment(self: Pin<&mut Self>) {
        let new_value = self.persistent_value() + 1;
        self.set_persistent_value(new_value);
    }
}
