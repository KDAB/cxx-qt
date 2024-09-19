// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a Q_INVOKABLE can be used

/// A CXX-Qt bridge which shows how a Q_INVOKABLE can be used
// ANCHOR: book_macro_code
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        /// QColor from cxx_qt_lib
        type QColor = cxx_qt_lib::QColor;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        type RustInvokables = super::RustInvokablesRust;
    }

    // ANCHOR: book_qnamespace
    #[qml_element]
    qnamespace!("Colors");
    // ANCHOR_END: book_qnamespace

    // ANCHOR: book_namespaced_qenum
    #[qenum]
    #[namespace = "Colors"]
    /// An enum of colors
    enum Color {
        /// Red
        Red,
        /// Green
        Green,
        /// Blue
        Blue,
    }
    // ANCHOR_END: book_namespaced_qenum

    // ANCHOR: book_invokable_signature
    unsafe extern "RustQt" {
        /// Immutable invokable method that returns the QColor
        #[qinvokable]
        #[cxx_name = "loadColor"]
        fn load_color(self: &RustInvokables) -> Result<QColor>;

        /// Mutable invokable method that stores a color
        #[qinvokable]
        #[cxx_name = "storeColor"]
        fn store_color(self: Pin<&mut RustInvokables>, red: f32, green: f32, blue: f32);

        /// Mutable invokable method that stores a color with an enum
        #[qinvokable]
        #[cxx_name = "storeColorWithEnum"]
        fn store_color_with_enum(self: Pin<&mut RustInvokables>, color: Color);

        /// Mutable invokable method with no parameters that resets the color
        #[qinvokable]
        fn reset(self: Pin<&mut RustInvokables>);
    }
    // ANCHOR_END: book_invokable_signature

    // ANCHOR: book_cpp_method_signature
    unsafe extern "RustQt" {
        /// C++ only method which returns the red value
        #[cxx_name = "redValue"]
        fn red_value(self: &RustInvokables) -> f32;
    }
    // ANCHOR_END: book_cpp_method_signature
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QColor;

/// A QObject which has Q_INVOKABLEs
pub struct RustInvokablesRust {
    pub(crate) red: f32,
    pub(crate) green: f32,
    pub(crate) blue: f32,
}

impl Default for RustInvokablesRust {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.4667,
            blue: 0.7843,
        }
    }
}

// ANCHOR: book_invokable_impl
impl qobject::RustInvokables {
    /// Immutable invokable method that returns the QColor
    pub fn load_color(&self) -> Result<QColor, i32> {
        Ok(self.as_qcolor())
    }

    /// Mutable invokable method that stores a color
    pub fn store_color(self: Pin<&mut Self>, red: f32, green: f32, blue: f32) {
        self.store_helper(red, green, blue);
    }

    /// QENUMS!
    pub fn store_color_with_enum(self: Pin<&mut Self>, color: qobject::Color) {
        use qobject::Color;
        let (r, g, b) = match color {
            Color::Red => (1.0, 0.0, 0.0),
            Color::Green => (0.0, 1.0, 0.0),
            Color::Blue => (0.0, 0.0, 1.0),
            _ => (0.0, 0.0, 0.0),
        };
        self.store_helper(r, g, b);
    }

    /// Mutable invokable method with no parameters that resets the color
    pub fn reset(self: Pin<&mut Self>) {
        self.store_helper(0.0, 0.4667, 0.7843);
    }
}
// ANCHOR_END: book_invokable_impl

// ANCHOR: book_cpp_method_impl
impl qobject::RustInvokables {
    /// C++ only method which returns the red value
    pub fn red_value(&self) -> f32 {
        self.red
    }
}
// ANCHOR_END: book_cpp_method_impl

impl qobject::RustInvokables {
    /// Mutable C++ context method that helps to store the color
    fn store_helper(mut self: Pin<&mut Self>, red: f32, green: f32, blue: f32) {
        let mut rust_mut = self.as_mut().rust_mut();
        rust_mut.red = red;
        rust_mut.green = green;
        rust_mut.blue = blue;
    }
}

impl RustInvokablesRust {
    /// Immutable Rust context method that returns the QColor
    pub fn as_qcolor(&self) -> QColor {
        QColor::from_rgb(
            (self.red * 255.0).round() as i32,
            (self.green * 255.0).round() as i32,
            (self.blue * 255.0).round() as i32,
        )
    }
}
// ANCHOR_END: book_macro_code
