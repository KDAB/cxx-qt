// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {

    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QFontStyle {
        StyleNormal,
        StyleItalic,
        StyleOblique,
    }

    /// This enum describes the different levels of hinting that can be applied to glyphs
    /// to improve legibility on displays where it might be warranted by the density of pixels.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QFontHintingPreference {
        PreferDefaultHinting,
        PreferNoHinting,
        PreferVerticalHinting,
        PreferFullHinting,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qfont.h");
        type QFont = super::QFont;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Returns true if weight() is a value greater than QFont::Medium; otherwise returns false.
        fn bold(self: &QFont) -> bool;

        /// Returns the family name that corresponds to the current style hint.
        #[rust_name = "default_family"]
        fn defaultFamily(self: &QFont) -> QString;

        /// Returns the currently preferred hinting level for glyphs rendered with this font.
        #[rust_name = "hinting_preference"]
        fn hintingPreference(self: &QFont) -> QFontHintingPreference;

        /// Returns true if the style() of the font is not QFont::StyleNormal
        fn italic(self: &QFont) -> bool;

        /// Returns true if kerning should be used when drawing text with this font.
        fn kerning(self: &QFont) -> bool;

        /// Returns the pixel size of the font if it was set with setPixelSize(). Returns -1 if the size was set with setPointSize() or setPointSizeF().
        #[rust_name = "pixel_size"]
        fn pixelSize(self: &QFont) -> i32;

        /// Returns the point size of the font. Returns -1 if the font size was specified in pixels.
        #[rust_name = "point_size"]
        fn pointSize(self: &QFont) -> i32;

        /// If enable is true sets the font's weight to QFont::Bold; otherwise sets the weight to QFont::Normal.
        #[rust_name = "set_bold"]
        fn setBold(self: &mut QFont, enable: bool);

        /// Sets the family name of the font. The name is case insensitive and may include a foundry name.
        #[rust_name = "set_family"]
        fn setFamily(self: &mut QFont, family: &QString);

        /// If enable is true, sets fixed pitch on; otherwise sets fixed pitch off.
        #[rust_name = "set_fixed_pitch"]
        fn setFixedPitch(self: &mut QFont, enable: bool);

        /// Sets the style() of the font to QFont::StyleItalic if enable is true; otherwise the style is set to QFont::StyleNormal.
        #[rust_name = "set_italic"]
        fn setItalic(self: &mut QFont, enable: bool);

        /// If enable is true, sets strikeout on; otherwise sets strikeout off.
        #[rust_name = "set_strikeout"]
        fn setStrikeOut(self: &mut QFont, enable: bool);

        /// Sets the style of the font to style.
        #[rust_name = "set_style"]
        fn setStyle(self: &mut QFont, style: QFontStyle);

        /// Returns true if strikeout has been set; otherwise returns false.
        fn strikeOut(self: &QFont) -> bool;

        /// Returns true if underline has been set; otherwise returns false.
        fn underline(self: &QFont) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
        type QFontStyle;
        type QFontHintingPreference;

        #[doc(hidden)]
        #[rust_name = "qfont_init_default"]
        fn construct() -> QFont;

        #[doc(hidden)]
        #[rust_name = "qfont_drop"]
        fn drop(pen: &mut QFont);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct QFont {
    _cspec: MaybeUninit<i32>,
    _resolve_mask: MaybeUninit<u16>,
}

impl Default for QFont {
    fn default() -> Self {
        ffi::qfont_init_default()
    }
}

impl Drop for QFont {
    fn drop(&mut self) {
        ffi::qfont_drop(self);
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QFont {
    type Id = type_id!("QFont");
    type Kind = cxx::kind::Trivial;
}
