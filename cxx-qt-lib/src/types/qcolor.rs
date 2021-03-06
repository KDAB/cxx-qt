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
pub type QColorCpp = ffi::QColor;

impl QColorCpp {
    /// Create a new Rust QColor from this QColorCpp.
    /// This is a copy operation so any changes will not propagate to the original QColorCpp.
    pub fn to_rust(&self) -> QColor {
        QColor::from_qcolor(self)
    }
}

/// The Rust representation of Qt's QColor
///
/// Internally this holds a UniquePtr to a QColorCpp which has been constructed on the C++ side.
pub struct QColor {
    inner: cxx::UniquePtr<QColorCpp>,
}

impl Default for QColor {
    fn default() -> Self {
        QColor::from_unique_ptr(ffi::qcolor_init())
    }
}

impl QColor {
    /// Construct a Rust QColor from an existing UniquePtr<QColorCpp> this is a move operation
    ///
    /// This is used in QVariant::value so that we don't need to perform another copy
    pub(crate) fn from_unique_ptr(color: cxx::UniquePtr<QColorCpp>) -> Self {
        Self { inner: color }
    }

    /// Construct a Rust QColor from an existing QColorCpp, this is a copy operation.
    pub fn from_qcolor(color: &QColorCpp) -> Self {
        Self {
            inner: ffi::qcolor_init_from_qcolor(color),
        }
    }

    /// Constructs a QColor with the RGB value r, g, b, and the alpha-channel (transparency) value of a.
    ///
    /// The color is left invalid if any of the arguments are invalid.
    pub fn from_rgba(red: i32, green: i32, blue: i32, alpha: i32) -> Self {
        QColor::from_unique_ptr(ffi::qcolor_init_from_rgba(red, green, blue, alpha))
    }

    /// Returns the alpha color component of this color.
    pub fn alpha(&self) -> i32 {
        if let Some(inner) = self.inner.as_ref() {
            inner.alpha()
        } else {
            0
        }
    }

    /// Returns the blue color component of this color.
    pub fn blue(&self) -> i32 {
        if let Some(inner) = self.inner.as_ref() {
            inner.blue()
        } else {
            0
        }
    }

    /// Returns the green color component of this color.
    pub fn green(&self) -> i32 {
        if let Some(inner) = self.inner.as_ref() {
            inner.green()
        } else {
            0
        }
    }

    /// Returns the red color component of this color.
    pub fn red(&self) -> i32 {
        if let Some(inner) = self.inner.as_ref() {
            inner.red()
        } else {
            0
        }
    }

    /// Sets the alpha of this color to alpha. Integer alpha is specified in the range 0-255.
    pub fn set_alpha(&mut self, alpha: i32) {
        if let Some(inner) = self.inner.as_mut() {
            inner.set_alpha(alpha);
        }
    }

    /// Sets the blue color component of this color to blue. Integer components are specified in the range 0-255.
    pub fn set_blue(&mut self, blue: i32) {
        if let Some(inner) = self.inner.as_mut() {
            inner.set_blue(blue);
        }
    }

    /// Sets the green color component of this color to green. Integer components are specified in the range 0-255.
    pub fn set_green(&mut self, green: i32) {
        if let Some(inner) = self.inner.as_mut() {
            inner.set_green(green);
        }
    }

    /// Sets the red color component of this color to red. Integer components are specified in the range 0-255.
    pub fn set_red(&mut self, red: i32) {
        if let Some(inner) = self.inner.as_mut() {
            inner.set_red(red);
        }
    }
}

impl crate::ToUniquePtr for QColor {
    type CppType = QColorCpp;

    /// Retrieve the UniquePtr to the Qt QColorCpp of this Rust QColor
    /// so that this object can be passed back to C++.
    fn to_unique_ptr(self) -> cxx::UniquePtr<QColorCpp> {
        self.inner
    }
}

impl From<&QColorCpp> for QColor {
    fn from(qcolor: &QColorCpp) -> Self {
        qcolor.to_rust()
    }
}
