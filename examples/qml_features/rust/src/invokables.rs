// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a Q_INVOKABLE can be used

/// A CXX-Qt bridge which shows how a Q_INVOKABLE can be used
// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "rust_invokables")]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        /// QColor from cxx_qt_lib
        type QColor = cxx_qt_lib::QColor;
    }

    /// A QObject which has Q_INVOKABLEs
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    pub struct RustInvokables {
        pub(crate) red: f32,
        pub(crate) green: f32,
        pub(crate) blue: f32,
    }

    impl Default for RustInvokables {
        fn default() -> Self {
            Self {
                red: 0.0,
                green: 0.4667,
                blue: 0.7843,
            }
        }
    }

    // ANCHOR: book_invokable_signature
    unsafe extern "RustQt" {
        /// Immutable invokable method that returns the QColor
        #[qinvokable]
        fn load_color(self: &qobject::RustInvokables) -> QColor;

        /// Mutable invokable method that stores a color
        #[qinvokable]
        fn store_color(self: Pin<&mut qobject::RustInvokables>, red: f32, green: f32, blue: f32);

        /// Mutable invokable method with no parameters that resets the color
        #[qinvokable]
        fn reset(self: Pin<&mut qobject::RustInvokables>);
    }
    // ANCHOR_END: book_invokable_signature
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QColor;

// TODO: this will change to qobject::RustInvokables once
// https://github.com/KDAB/cxx-qt/issues/559 is done
//
// ANCHOR: book_invokable_impl
impl ffi::RustInvokablesQt {
    /// Immutable invokable method that returns the QColor
    fn load_color(&self) -> QColor {
        self.rust().as_qcolor()
    }

    /// Mutable invokable method that stores a color
    fn store_color(self: Pin<&mut Self>, red: f32, green: f32, blue: f32) {
        self.store_helper(red, green, blue);
    }

    /// Mutable invokable method with no parameters that resets the color
    fn reset(self: Pin<&mut Self>) {
        self.store_helper(0.0, 0.4667, 0.7843);
    }

    /// Mutable C++ context method that helps to store the color
    fn store_helper(mut self: Pin<&mut Self>, red: f32, green: f32, blue: f32) {
        let mut rust_mut = unsafe { self.as_mut().rust_mut() };
        rust_mut.red = red;
        rust_mut.green = green;
        rust_mut.blue = blue;
    }
}
// ANCHOR_END: book_invokable_impl

impl RustInvokables {
    /// Immutable Rust context method that returns the QColor
    fn as_qcolor(&self) -> QColor {
        QColor::from_rgb(
            (self.red * 255.0).round() as i32,
            (self.green * 255.0).round() as i32,
            (self.blue * 255.0).round() as i32,
        )
    }
}
// ANCHOR_END: book_macro_code
