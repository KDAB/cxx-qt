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

    /// Rendering option for text this font applies to.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QFontCapitalization {
        /// This is the normal text rendering option where no capitalization change is applied.
        MixedCase,
        /// This alters the text to be rendered in all uppercase type.
        AllUppercase,
        /// This alters the text to be rendered in all lowercase type.
        AllLowercase,
        /// This alters the text to be rendered in small-caps type.
        SmallCaps,
        /// This alters the text to be rendered with the first character of each word as an uppercase character.
        Capitalize,
    }

    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QFontSpacingType {
        /// A value of 100 will keep the spacing unchanged; a value of 200 will enlarge
        /// the spacing after a character by the width of the character itself.
        PercentageSpacing,
        /// A positive value increases the letter spacing by the corresponding pixels;
        /// a negative value decreases the spacing.
        AbsoluteSpacing,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qfont.h");
        type QFont = super::QFont;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Returns true if weight() is a value greater than QFont::Medium; otherwise returns false.
        fn bold(self: &QFont) -> bool;

        /// Returns the current capitalization type of the font.
        fn capitalization(self: &QFont) -> QFontCapitalization;

        /// Returns the family name that corresponds to the current style hint.
        #[rust_name = "default_family"]
        fn defaultFamily(self: &QFont) -> QString;

        /// Returns true if fixed pitch has been set; otherwise returns false.
        #[rust_name = "fixed_pitch"]
        fn fixedPitch(self: &QFont) -> bool;

        /// Returns the currently preferred hinting level for glyphs rendered with this font.
        #[rust_name = "hinting_preference"]
        fn hintingPreference(self: &QFont) -> QFontHintingPreference;

        /// Returns true if this font and f are copies of each other, i.e. one of them was created
        /// as a copy of the other and neither has been modified since. This is much stricter than equality.
        #[rust_name = "is_copy_of"]
        fn isCopyOf(self: &QFont, font: &QFont) -> bool;

        /// Returns true if the style() of the font is not QFont::StyleNormal
        fn italic(self: &QFont) -> bool;

        /// Returns true if kerning should be used when drawing text with this font.
        fn kerning(self: &QFont) -> bool;

        /// Returns the font's key, a textual representation of a font.
        /// It is typically used as the key for a cache or dictionary of fonts.
        fn key(self: &QFont) -> QString;

        /// Returns the spacing type used for letter spacing.
        #[rust_name = "letter_spacing_type"]
        fn letterSpacingType(self: &QFont) -> QFontSpacingType;

        /// Returns the pixel size of the font if it was set with setPixelSize(). Returns -1 if the size was set with setPointSize() or setPointSizeF().
        #[rust_name = "pixel_size"]
        fn pixelSize(self: &QFont) -> i32;

        /// Returns the point size of the font. Returns -1 if the font size was specified in pixels.
        #[rust_name = "point_size"]
        fn pointSize(self: &QFont) -> i32;

        /// Returns a new QFont that has attributes copied from other that have not been previously set on this font.
        fn resolve(self: &QFont, other: &QFont) -> QFont;

        /// If enable is true sets the font's weight to QFont::Bold; otherwise sets the weight to QFont::Normal.
        #[rust_name = "set_bold"]
        fn setBold(self: &mut QFont, enable: bool);

        /// Sets the capitalization of the text in this font to caps.
        #[rust_name = "set_capitalization"]
        fn setCapitalization(self: &mut QFont, caps: QFontCapitalization);

        /// Sets the family name of the font. The name is case insensitive and may include a foundry name.
        #[rust_name = "set_family"]
        fn setFamily(self: &mut QFont, family: &QString);

        /// If enable is true, sets fixed pitch on; otherwise sets fixed pitch off.
        #[rust_name = "set_fixed_pitch"]
        fn setFixedPitch(self: &mut QFont, enable: bool);

        /// Set the preference for the hinting level of the glyphs to hintingPreference.
        #[rust_name = "set_hinting_preference"]
        fn setHintingPreference(self: &mut QFont, hintingPreference: QFontHintingPreference);

        /// Sets the style() of the font to QFont::StyleItalic if enable is true; otherwise the style is set to QFont::StyleNormal.
        #[rust_name = "set_italic"]
        fn setItalic(self: &mut QFont, enable: bool);

        /// Enables kerning for this font if enable is true; otherwise disables it. By default, kerning is enabled.
        #[rust_name = "set_kerning"]
        fn setKerning(self: &mut QFont, enable: bool);

        /// Sets the letter spacing for the font to spacing and the type of spacing to type.
        #[rust_name = "set_letter_spacing"]
        fn setLetterSpacing(self: &mut QFont, spacingType: QFontSpacingType, spacing: f64);

        /// If enable is true, sets strikeout on; otherwise sets strikeout off.
        #[rust_name = "set_strikeout"]
        fn setStrikeOut(self: &mut QFont, enable: bool);

        /// Sets the style of the font to style.
        #[rust_name = "set_style"]
        fn setStyle(self: &mut QFont, style: QFontStyle);

        /// Sets the style name of the font to styleName.
        #[rust_name = "set_style_name"]
        fn setStyleName(self: &mut QFont, styleName: &QString);

        /// Returns true if strikeout has been set; otherwise returns false.
        fn strikeOut(self: &QFont) -> bool;

        /// If enable is true, sets underline on; otherwise sets underline off.
        #[rust_name = "set_underline"]
        fn setUnderline(self: &mut QFont, enable: bool);

        /// Returns true if underline has been set; otherwise returns false.
        fn underline(self: &QFont) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
        type QFontStyle;
        type QFontHintingPreference;
        type QFontCapitalization;
        type QFontSpacingType;

        #[doc(hidden)]
        #[rust_name = "qfont_init_default"]
        fn construct() -> QFont;

        #[doc(hidden)]
        #[rust_name = "qfont_drop"]
        fn drop(pen: &mut QFont);

        #[doc(hidden)]
        #[rust_name = "qfont_clone"]
        fn construct(font: &QFont) -> QFont;
    }
}

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

impl Clone for QFont {
    fn clone(&self) -> Self {
        ffi::qfont_clone(self)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QFont {
    type Id = type_id!("QFont");
    type Kind = cxx::kind::Trivial;
}
