// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a QML_SINGLETON can be used

/// A CXX-Qt bridge which shows how a QML_SINGLETON can be used
// ANCHOR: book_macro_code
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[qproperty(i32, persistent_value)]
        type RustSingleton = super::RustSingletonRust;

        /// Increment the persistent value Q_PROPERTY of the QML_SINGLETON
        #[qinvokable]
        fn increment(self: Pin<&mut RustSingleton>);
    }
}

use core::pin::Pin;

/// A QObject which is a QML_SINGLETON
#[derive(Default)]
pub struct RustSingletonRust {
    /// A Q_PROPERTY with a persistent value
    persistent_value: i32,
}

impl qobject::RustSingleton {
    /// Increment the persistent value Q_PROPERTY of the QML_SINGLETON
    pub fn increment(self: Pin<&mut Self>) {
        let new_value = self.persistent_value() + 1;
        self.set_persistent_value(new_value);
    }
}
