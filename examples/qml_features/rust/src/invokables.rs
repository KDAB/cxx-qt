// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "rust_invokables")]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
    }

    #[cxx_qt::qobject]
    pub struct RustInvokables {
        red: f32,
        green: f32,
        blue: f32,
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

    impl qobject::RustInvokables {
        /// Immutable invokable method that returns the QColor
        #[qinvokable]
        pub fn load_color(&self) -> QColor {
            self.rust().as_qcolor()
        }

        /// Mutable invokable method that stores a color
        #[qinvokable]
        pub fn store_color(self: Pin<&mut Self>, red: f32, green: f32, blue: f32) {
            self.store_helper(red, green, blue);
        }

        /// Mutable invokable method with no parameters that resets the color
        #[qinvokable]
        pub fn reset(self: Pin<&mut Self>) {
            self.store_helper(0.0, 0.4667, 0.7843);
        }

        /// Mutable C++ context method that helps to store the color
        pub fn store_helper(mut self: Pin<&mut Self>, red: f32, green: f32, blue: f32) {
            self.as_mut().set_red(red);
            self.as_mut().set_green(green);
            self.as_mut().set_blue(blue);
        }
    }

    impl RustInvokables {
        /// Immutable Rust context method that returns the QColor
        fn as_qcolor(&self) -> QColor {
            QColor::from_rgba(
                (self.red * 255.0).round() as i32,
                (self.green * 255.0).round() as i32,
                (self.blue * 255.0).round() as i32,
                255,
            )
        }
    }
}
// ANCHOR_END: book_macro_code
