// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");

        type QColor = super::QColor;

        /// Returns the alpha color component of this color.
        fn alpha(self: &QColor) -> i32;
        /// Returns the blue color component of this color.
        fn blue(self: &QColor) -> i32;
        /// Returns the green color component of this color.
        fn green(self: &QColor) -> i32;
        /// Returns the red color component of this color.
        fn red(self: &QColor) -> i32;
        /// Sets the alpha of this color to alpha. Integer alpha is specified in the range 0-255.
        #[rust_name = "set_alpha"]
        fn setAlpha(self: &mut QColor, red: i32);
        /// Sets the blue color component of this color to blue. Integer components are specified in the range 0-255.
        #[rust_name = "set_blue"]
        fn setBlue(self: &mut QColor, blue: i32);
        /// Sets the green color component of this color to green. Integer components are specified in the range 0-255.
        #[rust_name = "set_green"]
        fn setGreen(self: &mut QColor, green: i32);
        /// Sets the red color component of this color to red. Integer components are specified in the range 0-255.
        #[rust_name = "set_red"]
        fn setRed(self: &mut QColor, red: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qcolor_init_default"]
        fn qcolorInitDefault() -> QColor;
        #[doc(hidden)]
        #[rust_name = "qcolor_init_from_rgba"]
        fn qcolorInitFromRgba(red: i32, green: i32, blue: i32, alpha: i32) -> QColor;
    }
}

/// The QColor class provides colors based on RGB, HSV or CMYK values.
///
/// Note that we only expose RGB methods for now.
#[derive(Clone)]
#[repr(C)]
pub struct QColor {
    _space: MaybeUninit<[usize; 2]>,
}

impl QColor {
    /// Constructs a QColor with the RGB value r, g, b, and the alpha-channel (transparency) value of a.
    ///
    /// The color is left invalid if any of the arguments are invalid.
    pub fn from_rgba(red: i32, green: i32, blue: i32, alpha: i32) -> Self {
        ffi::qcolor_init_from_rgba(red, green, blue, alpha)
    }
}

impl Default for QColor {
    /// Constructs an invalid color. An invalid color is a color that is not properly set up for the underlying window system.
    ///
    /// The alpha value of an invalid color is unspecified.
    fn default() -> Self {
        ffi::qcolor_init_default()
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QColor {
    type Id = type_id!("QColor");
    type Kind = cxx::kind::Trivial;
}
