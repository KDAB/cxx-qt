// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QColor;

        fn alpha(self: &QColor) -> i32;
        fn blue(self: &QColor) -> i32;
        fn green(self: &QColor) -> i32;
        fn red(self: &QColor) -> i32;
        #[rust_name = "set_alpha"]
        fn setAlpha(self: Pin<&mut QColor>, red: i32);
        #[rust_name = "set_blue"]
        fn setBlue(self: Pin<&mut QColor>, blue: i32);
        #[rust_name = "set_green"]
        fn setGreen(self: Pin<&mut QColor>, green: i32);
        #[rust_name = "set_red"]
        fn setRed(self: Pin<&mut QColor>, red: i32);

        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qcolor_init"]
        fn qcolorInit() -> UniquePtr<QColor>;
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qcolor_init_from_rgba"]
        fn qcolorInitFromRgba(red: i32, green: i32, blue: i32, alpha: i32) -> UniquePtr<QColor>;
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qcolor_init_from_qcolor"]
        fn qcolorInitFromQColor(color: &QColor) -> UniquePtr<QColor>;
    }

    impl UniquePtr<QColor> {}
}

/// The QColorCpp class provides colors based on RGB, HSV or CMYK values.
///
/// Note that we only expose RGB methods for now.
///
/// Note that this is the C++ representation and QColor should be used in Rust.
pub type QColor = ffi::QColor;

impl QColor {
    /// Construct a default null QColor
    pub fn null() -> cxx::UniquePtr<Self> {
        ffi::qcolor_init()
    }

    /// Construct a Rust QColor from an existing QColorCpp, this is a copy operation.
    pub fn from_ref(color: &QColor) -> cxx::UniquePtr<Self> {
        ffi::qcolor_init_from_qcolor(color)
    }

    /// Constructs a QColor with the RGB value r, g, b, and the alpha-channel (transparency) value of a.
    ///
    /// The color is left invalid if any of the arguments are invalid.
    pub fn from_rgba(red: i32, green: i32, blue: i32, alpha: i32) -> cxx::UniquePtr<Self> {
        ffi::qcolor_init_from_rgba(red, green, blue, alpha)
    }
}

impl From<&QColor> for cxx::UniquePtr<QColor> {
    fn from(value: &QColor) -> cxx::UniquePtr<QColor> {
        QColor::from_ref(value)
    }
}
