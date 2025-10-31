// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::fmt;
use std::mem::MaybeUninit;

use crate::QString;

#[cxx::bridge]
mod ffi {
    /// This enum describes the different styles of glyphs that are used to display text.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QFontStyle {
        /// Normal glyphs used in unstyled text.
        StyleNormal,
        /// Italic glyphs that are specifically designed for the purpose of representing italicized text.
        StyleItalic,
        /// Glyphs with an italic appearance that are typically based on the unstyled glyphs,
        /// but are not fine-tuned for the purpose of representing italicized text.
        StyleOblique,
    }

    /// This enum describes the different levels of hinting that can be applied to glyphs
    /// to improve legibility on displays where it might be warranted by the density of pixels.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QFontHintingPreference {
        /// Use the default hinting level for the target platform.
        PreferDefaultHinting,
        /// If possible, render text without hinting the outlines of the glyphs.
        /// The text layout will be typographically accurate and scalable, using the same metrics as are used e.g. when printing.
        PreferNoHinting,
        /// If possible, render text with no horizontal hinting, but align glyphs to the pixel grid in the vertical direction.
        /// The text will appear crisper on displays where the density is too low to give an accurate rendering of the glyphs.
        /// But since the horizontal metrics of the glyphs are unhinted, the text's layout will be scalable to higher density
        /// devices (such as printers) without impacting details such as line breaks.
        PreferVerticalHinting,
        /// If possible, render text with hinting in both horizontal and vertical directions. The text will be altered to optimize
        /// legibility on the target device, but since the metrics will depend on the target size of the text, the positions of
        /// glyphs, line breaks, and other typographical detail will not scale, meaning that a text layout may look
        /// different on devices with different pixel densities.
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

    /// The style strategy tells the [font matching](https://doc.qt.io/qt/qfont.html#fontmatching) algorithm what type of fonts should
    /// be used to find an appropriate default family.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QFontStyleStrategy {
        /// The default style strategy. It does not prefer any type of font.
        PreferDefault = 0x0001,
        /// Prefers bitmap fonts (as opposed to outline fonts).
        PreferBitmap = 0x0002,
        /// Prefers device fonts.
        PreferDevice = 0x0004,
        /// Prefers outline fonts (as opposed to bitmap fonts).
        PreferOutline = 0x0008,
        /// Forces the use of outline fonts.
        ForceOutline = 0x0010,
        // Don't antialias the fonts.
        // PreferMatch = 0x0020,
        // Avoid subpixel antialiasing on the fonts if possible
        // PreferQuality = 0x0040,
        /// Antialias if possible.
        PreferAntialias = 0x0080,
        /// Don't antialias the fonts.
        NoAntialias = 0x0100,
        /// Avoid subpixel antialiasing on the fonts if possible.
        NoSubpixelAntialias = 0x0800,
        /// Sometimes, a font will apply complex rules to a set of characters
        /// in order to display them correctly. In some writing systems, such
        /// as Brahmic scripts, this is required in order for the text to be legible,
        /// but in e.g. Latin script, it is merely a cosmetic feature.
        /// The `PreferNoShaping` flag will disable all such features when they are not
        /// required, which will improve performance in most cases (since Qt 5.10).
        PreferNoShaping = 0x1000,
        /// If the font selected for a certain writing system does not contain a character
        /// requested to draw, then Qt automatically chooses a similar looking font that
        /// contains the character. This flag disables this feature. Please note
        /// that enabling this flag will not prevent Qt from automatically picking a
        /// suitable font when the selected font does not support the writing system of the text.
        NoFontMerging = 0x8000,
    }

    /// Style hints are used by the [font matching](https://doc.qt.io/qt/qfont.html#fontmatching) algorithm to find an appropriate default
    /// family if a selected font family is not available.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QFontStyleHint {
        /// The font matcher prefer sans serif fonts.
        Helvetica,
        /// The font matcher prefers serif fonts.
        Times,
        /// The font matcher prefers fixed pitch fonts.
        Courier,
        /// The font matcher prefers decorative fonts.
        OldEnglish,
        /// The font matcher prefers system fonts.
        System,
        /// Leaves the font matching algorithm to choose the family. This is the default.
        AnyStyle,
        /// The font matcher prefers fonts that map to the CSS generic font-family 'cursive'.
        Cursive,
        /// The font matcher prefers fonts that map to the CSS generic font-family 'monospace'.
        Monospace,
        /// The font matcher prefers fonts that map to the CSS generic font-family 'fantasy'.
        Fantasy,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qfont.h");
        type QFont = super::QFont;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;

        /// Returns `true` if [weight](https://doc.qt.io/qt/qfont.html#weight)() is a value greater than 400; otherwise returns `false`.
        fn bold(self: &QFont) -> bool;

        /// Returns the current capitalization type of the font.
        fn capitalization(self: &QFont) -> QFontCapitalization;

        /// Returns the family name that corresponds to the current style hint.
        #[rust_name = "default_family"]
        fn defaultFamily(self: &QFont) -> QString;

        /// Returns `true` if a window system font exactly matching
        /// the settings of this font is available.
        #[rust_name = "exact_match"]
        fn exactMatch(self: &QFont) -> bool;

        /// Returns the requested font family name. This will always be the same as the first entry in [`families`](Self::families).
        #[rust_name = "family_or_default"]
        fn family(self: &QFont) -> QString;

        /// Returns the requested font family names, i.e. the names set in the last [`set_families`](Self::set_families)
        /// call or via the constructor. Otherwise it returns an empty list.
        fn families(self: &QFont) -> QStringList;

        /// Returns `true` if fixed pitch has been set; otherwise returns `false`.
        #[rust_name = "fixed_pitch"]
        fn fixedPitch(self: &QFont) -> bool;

        /// Sets this font to match the description `descrip`. The description is a comma-separated
        /// list of the font attributes, as returned by [`to_qstring`](Self::to_qstring).
        #[rust_name = "from_string"]
        fn fromString(self: &mut QFont, descrip: &QString) -> bool;

        /// Returns the currently preferred hinting level for glyphs rendered with this font.
        #[rust_name = "hinting_preference"]
        fn hintingPreference(self: &QFont) -> QFontHintingPreference;

        /// Returns `true` if this font and `f` are copies of each other, i.e. one of them was created
        /// as a copy of the other and neither has been modified since. This is much stricter than equality.
        #[rust_name = "is_copy_of"]
        fn isCopyOf(self: &QFont, font: &QFont) -> bool;

        /// Returns `true` if the [style](https://doc.qt.io/qt/qfont.html#style)() of the font is not [`QFontStyle::StyleNormal`].
        fn italic(self: &QFont) -> bool;

        /// Returns `true` if kerning should be used when drawing text with this font.
        fn kerning(self: &QFont) -> bool;

        /// Returns the font's key, a textual representation of a font.
        /// It is typically used as the key for a cache or dictionary of fonts.
        fn key(self: &QFont) -> QString;

        /// Returns the letter spacing for the font.
        #[rust_name = "letter_spacing"]
        fn letterSpacing(self: &QFont) -> f64;

        /// Returns the spacing type used for letter spacing.
        #[rust_name = "letter_spacing_type"]
        fn letterSpacingType(self: &QFont) -> QFontSpacingType;

        /// Returns `true` if overline has been set; otherwise returns `false`.
        fn overline(self: &QFont) -> bool;

        /// Returns the pixel size of the font if it was set with [`set_pixel_size`](Self::set_pixel_size). Returns -1 if the size was not specified in pixels.
        #[rust_name = "pixel_size"]
        fn pixelSize(self: &QFont) -> i32;

        /// Returns the point size of the font. Returns -1 if the font size was specified in pixels.
        #[rust_name = "point_size"]
        fn pointSize(self: &QFont) -> i32;

        /// Returns a new `QFont` that has attributes copied from other that have not been previously set on this font.
        fn resolve(self: &QFont, other: &QFont) -> QFont;

        /// If `enable` is `true` sets the font's weight to 700; otherwise sets the weight to 400.
        ///
        /// For finer boldness control use [setWeight](https://doc.qt.io/qt/qfont.html#setWeight)().
        #[rust_name = "set_bold"]
        fn setBold(self: &mut QFont, enable: bool);

        /// Sets the capitalization of the text in this font to `caps`.
        ///
        /// A font's capitalization makes the text appear in the selected capitalization mode.
        #[rust_name = "set_capitalization"]
        fn setCapitalization(self: &mut QFont, caps: QFontCapitalization);

        /// Sets the family name of the font. The name is case insensitive and may include a foundry name.
        ///
        /// The family name may optionally also include a foundry name, e.g. "Helvetica [Cronyx]". If the family is available from more than one foundry and the foundry isn't specified, an arbitrary foundry is chosen. If the family isn't available a family will be set using the [font matching](https://doc.qt.io/qt/qfont.html#fontmatching) algorithm.
        #[rust_name = "set_family"]
        fn setFamily(self: &mut QFont, family: &QString);

        /// Sets the list of family names for the font. The names are case insensitive and may include
        /// a foundry name. The first family in `families` will be set as the main family for the font.
        ///
        /// Each family name entry in families may optionally also include a foundry name, e.g. "Helvetica [Cronyx]". If the family is available from more than one foundry and the foundry isn't specified, an arbitrary foundry is chosen. If the family isn't available a family will be set using the [font matching](https://doc.qt.io/qt/qfont.html#fontmatching) algorithm.
        #[rust_name = "set_families"]
        fn setFamilies(self: &mut QFont, families: &QStringList);

        /// If `enable` is `true`, sets fixed pitch on; otherwise sets fixed pitch off.
        #[rust_name = "set_fixed_pitch"]
        fn setFixedPitch(self: &mut QFont, enable: bool);

        /// Sets the style strategy for the font to `strategy`.
        #[rust_name = "set_style_strategy"]
        fn setStyleStrategy(self: &mut QFont, strategy: QFontStyleStrategy);

        /// Set the preference for the hinting level of the glyphs to `hinting_preference`.
        ///  This is a hint to the underlying font rendering system to use a certain level of hinting, and has varying support across platforms.
        #[rust_name = "set_hinting_preference"]
        fn setHintingPreference(self: &mut QFont, hinting_preference: QFontHintingPreference);

        /// Sets the [style](https://doc.qt.io/qt/qfont.html#style)() of the font to [`QFontStyle::StyleItalic`] if `enable` is `true`; otherwise the style is set to [`QFontStyle::StyleNormal`].
        ///
        /// Note: If [`style_name`](Self::style_name) is set, this value may be ignored, or if supported on the platform, the font may be rendered tilted instead of picking a designed italic font-variant.
        #[rust_name = "set_italic"]
        fn setItalic(self: &mut QFont, enable: bool);

        /// Enables kerning for this font if `enable` is `true`; otherwise disables it. By default, kerning is enabled.
        ///
        /// When kerning is enabled, glyph metrics do not add up anymore, even for Latin text. In other words, the assumption that `width("a") + width("b") = width("ab")` is not necessarily true.
        #[rust_name = "set_kerning"]
        fn setKerning(self: &mut QFont, enable: bool);

        /// Sets the letter spacing for the font to `spacing` and the type of spacing to `spacing_type`.
        ///
        /// Letter spacing changes the default spacing between individual letters in the font. The spacing between the letters can be made smaller as well as larger either in percentage of the character width or in pixels, depending on the selected spacing type.
        #[rust_name = "set_letter_spacing"]
        fn setLetterSpacing(self: &mut QFont, spacing_type: QFontSpacingType, spacing: f64);

        /// If `enable` is `true`, sets overline on; otherwise sets overline off.
        #[rust_name = "set_overline"]
        fn setOverline(self: &mut QFont, enable: bool);

        /// Sets the font size to `pixel_size` pixels.
        #[rust_name = "set_pixel_size"]
        fn setPixelSize(self: &mut QFont, pixel_size: i32);

        /// Sets the stretch `factor` for the font.
        #[rust_name = "set_stretch"]
        fn setStretch(self: &mut QFont, factor: i32);

        /// If `enable` is `true`, sets strikeout on; otherwise sets strikeout off.
        #[rust_name = "set_strikeout"]
        fn setStrikeOut(self: &mut QFont, enable: bool);

        /// Sets the style of the font to `style`.
        #[rust_name = "set_style"]
        fn setStyle(self: &mut QFont, style: QFontStyle);

        /// Sets the style hint and strategy to `hint` and `strategy`, respectively.
        /// Qt does not support style hints on X11 since this information is not provided by the window system.
        #[rust_name = "set_style_hint"]
        fn setStyleHint(self: &mut QFont, hint: QFontStyleHint, strategy: QFontStyleStrategy);

        /// Sets the style name of the font to `style_name`.
        #[rust_name = "set_style_name"]
        fn setStyleName(self: &mut QFont, style_name: &QString);

        /// If `enable` is `true`, sets underline on; otherwise sets underline off.
        #[rust_name = "set_underline"]
        fn setUnderline(self: &mut QFont, enable: bool);

        /// Sets the word spacing for the font to `spacing`.
        #[rust_name = "set_word_spacing"]
        fn setWordSpacing(self: &mut QFont, spacing: f64);

        /// Returns the stretch factor for the font.
        fn stretch(self: &QFont) -> i32;

        /// Returns `true` if strikeout has been set; otherwise returns `false`.
        #[rust_name = "strike_out"]
        fn strikeOut(self: &QFont) -> bool;

        /// Returns the style hint.
        #[rust_name = "style_hint"]
        fn styleHint(self: &QFont) -> QFontStyleHint;

        /// Returns the requested font style name. This can be used to match the font
        /// with irregular styles (that can't be normalized in other style properties).
        #[rust_name = "style_name"]
        fn styleName(self: &QFont) -> QString;

        /// Returns the style strategy.
        #[rust_name = "style_strategy"]
        fn styleStrategy(self: &QFont) -> QFontStyleStrategy;

        /// Returns the font as a string.
        #[rust_name = "to_qstring"]
        fn toString(self: &QFont) -> QString;

        /// Returns `true` if underline has been set; otherwise returns `false`.
        fn underline(self: &QFont) -> bool;

        /// Returns the word spacing for the font.
        #[rust_name = "word_spacing"]
        fn wordSpacing(self: &QFont) -> f64;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
        type QFontStyle;
        type QFontHintingPreference;
        type QFontCapitalization;
        type QFontSpacingType;
        type QFontStyleStrategy;
        type QFontStyleHint;

        #[doc(hidden)]
        #[rust_name = "qfont_init_default"]
        fn construct() -> QFont;

        #[doc(hidden)]
        #[rust_name = "qfont_drop"]
        fn drop(pen: &mut QFont);

        #[doc(hidden)]
        #[rust_name = "qfont_clone"]
        fn construct(font: &QFont) -> QFont;

        #[doc(hidden)]
        #[rust_name = "qfont_eq"]
        fn operatorEq(a: &QFont, b: &QFont) -> bool;
    }
}

pub use ffi::{
    QFontCapitalization, QFontHintingPreference, QFontSpacingType, QFontStyle, QFontStyleHint,
    QFontStyleStrategy,
};

/// The `QFont` class specifies a query for a font used for drawing text.
///
/// Qt Documentation: [QFont](https://doc.qt.io/qt/qfont.html#details)
#[repr(C)]
pub struct QFont {
    _cspec: MaybeUninit<usize>,
    _resolve_mask: MaybeUninit<u32>,
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

impl fmt::Display for QFont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_qstring().fmt(f)
    }
}

impl fmt::Debug for QFont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_qstring().fmt(f)
    }
}

impl PartialEq for QFont {
    fn eq(&self, other: &Self) -> bool {
        ffi::qfont_eq(self, other)
    }
}

impl Eq for QFont {}

impl QFont {
    /// Returns the bounding rectangle of the current clip if there is a clip;
    /// otherwise returns `None`. Note that the clip region is given in logical coordinates.
    pub fn family(&self) -> Option<QString> {
        let result = self.family_or_default();
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QFont {
    type Id = type_id!("QFont");
    type Kind = cxx::kind::Trivial;
}
