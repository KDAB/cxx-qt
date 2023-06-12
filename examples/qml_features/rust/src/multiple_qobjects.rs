// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how multiple QObjects can be defined in one module

/// A CXX-Qt bridge which shows multiple QObjects can be defined in one module
#[cxx_qt::bridge(cxx_file_stem = "multiple_qobjects")]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        /// QColor from cxx_qt_lib
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qurl.h");
        /// QUrl from cxx_qt_lib
        type QUrl = cxx_qt_lib::QUrl;
    }

    /// The first QObject
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    #[qproperty(i32, counter)]
    #[qproperty(QColor, color)]
    pub struct FirstObject {
        counter: i32,
        color: QColor,
    }

    impl Default for FirstObject {
        fn default() -> Self {
            Self {
                counter: 10,
                color: QColor::from_rgb(0, 0, 255),
            }
        }
    }

    // Enabling threading on the qobject
    impl cxx_qt::Threading for qobject::FirstObject {}

    unsafe extern "RustQt" {
        /// Accepted Q_SIGNAL
        #[qsignal]
        fn accepted(self: Pin<&mut qobject::FirstObject>);

        /// Rejected Q_SIGNAL
        #[qsignal]
        fn rejected(self: Pin<&mut qobject::FirstObject>);
    }

    unsafe extern "RustQt" {
        /// A Q_INVOKABLE on the first QObject which increments a counter
        #[qinvokable]
        fn increment(self: Pin<&mut qobject::FirstObject>);
    }

    /// The second QObject
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    #[qproperty(i32, counter)]
    #[qproperty(QUrl, url)]
    pub struct SecondObject {
        counter: i32,
        url: QUrl,
    }

    impl Default for SecondObject {
        fn default() -> Self {
            Self {
                counter: 100,
                url: QUrl::from("https://github.com/kdab/cxx-qt"),
            }
        }
    }

    // Enabling threading on the qobject
    impl cxx_qt::Threading for qobject::SecondObject {}

    unsafe extern "RustQt" {
        /// Accepted Q_SIGNAL
        #[qsignal]
        fn accepted(self: Pin<&mut qobject::SecondObject>);

        /// Rejected Q_SIGNAL
        #[qsignal]
        fn rejected(self: Pin<&mut qobject::SecondObject>);
    }

    unsafe extern "RustQt" {
        /// A Q_INVOKABLE on the second QObject which increments a counter
        #[qinvokable]
        fn increment(self: Pin<&mut qobject::SecondObject>);
    }
}

use core::pin::Pin;
use cxx_qt_lib::{QColor, QUrl};

// TODO: this will change to qobject::FirstObject once
// https://github.com/KDAB/cxx-qt/issues/559 is done
impl ffi::FirstObjectQt {
    /// A Q_INVOKABLE on the first QObject which increments a counter
    fn increment(mut self: Pin<&mut Self>) {
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

// TODO: this will change to qobject::SecondObject once
// https://github.com/KDAB/cxx-qt/issues/559 is done
impl ffi::SecondObjectQt {
    /// A Q_INVOKABLE on the second QObject which increments a counter
    fn increment(mut self: Pin<&mut Self>) {
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
