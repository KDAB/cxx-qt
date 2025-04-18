// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how multiple QObjects can be defined in one module

/// A CXX-Qt bridge which shows multiple QObjects can be defined in one module
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        /// QColor from cxx_qt_lib
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qurl.h");
        /// QUrl from cxx_qt_lib
        type QUrl = cxx_qt_lib::QUrl;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, counter)]
        #[qproperty(QColor, color)]
        type FirstObject = super::FirstObjectRust;

        /// Accepted Q_SIGNAL
        #[qsignal]
        fn accepted(self: Pin<&mut Self>);

        /// Rejected Q_SIGNAL
        #[qsignal]
        fn rejected(self: Pin<&mut Self>);

        /// A Q_INVOKABLE on the first QObject which increments a counter
        #[qinvokable]
        fn increment(self: Pin<&mut Self>);
    }

    // Enabling threading on the qobject
    impl cxx_qt::Threading for FirstObject {}

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, counter)]
        #[qproperty(QUrl, url)]
        type SecondObject = super::SecondObjectRust;

        /// Accepted Q_SIGNAL
        #[qsignal]
        fn accepted(self: Pin<&mut Self>);

        /// Rejected Q_SIGNAL
        #[qsignal]
        fn rejected(self: Pin<&mut Self>);

        /// A Q_INVOKABLE on the second QObject which increments a counter
        #[qinvokable]
        fn increment(self: Pin<&mut Self>);
    }

    // Enabling threading on the qobject
    impl cxx_qt::Threading for SecondObject {}
}

use core::pin::Pin;
use cxx_qt_lib::{QColor, QUrl};

/// The first QObject
pub struct FirstObjectRust {
    counter: i32,
    color: QColor,
}

impl Default for FirstObjectRust {
    fn default() -> Self {
        Self {
            counter: 10,
            color: QColor::from_rgb(0, 0, 255),
        }
    }
}

impl qobject::FirstObject {
    /// A Q_INVOKABLE on the first QObject which increments a counter
    pub fn increment(mut self: Pin<&mut Self>) {
        let new_value = self.as_ref().counter() + 1;
        self.as_mut().set_counter(new_value);

        if new_value % 2 == 0 {
            self.as_mut().set_color(QColor::from_rgb(0, 0, 255));
            self.accepted();
        } else {
            self.as_mut().set_color(QColor::from_rgb(255, 0, 0));
            self.rejected();
        }
    }
}

/// The second QObject
pub struct SecondObjectRust {
    counter: i32,
    url: QUrl,
}

impl Default for SecondObjectRust {
    fn default() -> Self {
        Self {
            counter: 100,
            url: QUrl::from("https://github.com/kdab/cxx-qt"),
        }
    }
}

impl qobject::SecondObject {
    /// A Q_INVOKABLE on the second QObject which increments a counter
    pub fn increment(mut self: Pin<&mut Self>) {
        let new_value = self.as_ref().counter() + 1;
        self.as_mut().set_counter(new_value);

        if new_value % 5 == 0 {
            self.as_mut()
                .set_url(QUrl::from("https://github.com/kdab/cxx-qt"));
            self.accepted();
        } else {
            self.as_mut().set_url(QUrl::from("https://kdab.com"));
            self.rejected();
        }
    }
}
