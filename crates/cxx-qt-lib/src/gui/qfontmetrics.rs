// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qfontmetrics.h");
        type QFontMetrics = super::QFontMetrics;
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qfont.h");
        type QFont = crate::QFont;

        /// Returns the ascent of the font.
        fn ascent(self: &QFontMetrics) -> i32;

        /// Returns the average width of glyphs in the font.
        #[rust_name = "average_char_width"]
        fn averageCharWidth(self: &QFontMetrics) -> i32;

        /// Returns the bounding rectangle of the characters in the string specified by text.
        /// The bounding rectangle always covers at least the set of pixels the text would cover if drawn at (0, 0).
        #[rust_name = "bounding_rect"]
        fn boundingRect(self: &QFontMetrics, text: &QString) -> QRect;

        /// Returns the cap height of the font.
        /// The cap height of a font is the height of a capital letter above the baseline. It specifically is
        /// the height of capital letters that are flat - such as H or I - as opposed to round letters such
        /// as O, or pointed letters like A, both of which may display overshoot.
        #[rust_name = "cap_height"]
        fn capHeight(self: &QFontMetrics) -> i32;

        /// Returns the descent of the font.
        /// The descent is the distance from the base line to the lowest point characters extend to. In practice,
        /// some font designers break this rule, e.g. to accommodate a certain character,
        /// so it is possible (though rare) that this value will be too small.
        fn descent(self: &QFontMetrics) -> i32;

        /// Returns the height of the font.
        fn height(self: &QFontMetrics) -> i32;

        /// Returns the leading of the font.
        fn leading(self: &QFontMetrics) -> i32;

        /// Returns the distance from one base line to the next.
        /// This value is always equal to leading()+height().
        #[rust_name = "line_spacing"]
        fn lineSpacing(self: &QFontMetrics) -> i32;

        /// Returns the width of the underline and strikeout lines, adjusted for the point size of the font.
        #[rust_name = "line_width"]
        fn lineWidth(self: &QFontMetrics) -> i32;

        /// Returns the width of the widest character in the font.
        #[rust_name = "max_width"]
        fn maxWidth(self: &QFontMetrics) -> i32;

        /// Returns the minimum left bearing of the font.
        #[rust_name = "min_left_bearing"]
        fn minLeftBearing(self: &QFontMetrics) -> i32;

        /// Returns the minimum right bearing of the font.
        #[rust_name = "min_right_bearing"]
        fn minRightBearing(self: &QFontMetrics) -> i32;

        /// Returns the distance from the base line to where an overline should be drawn.
        #[rust_name = "overline_position"]
        fn overlinePos(self: &QFontMetrics) -> i32;

        /// Returns the distance from the base line to where the strikeout line should be drawn.
        #[rust_name = "strike_out_position"]
        fn strikeOutPos(self: &QFontMetrics) -> i32;

        /// Returns a tight bounding rectangle around the characters in the string specified by text.
        /// The bounding rectangle always covers at least the set of pixels the text would cover if drawn at (0, 0).
        #[rust_name = "tight_bounding_rect"]
        fn tightBoundingRect(self: &QFontMetrics, text: &QString) -> QRect;

        /// Returns the distance from the base line to where an underscore should be drawn.
        #[rust_name = "underline_position"]
        fn underlinePos(self: &QFontMetrics) -> i32;

        /// Returns the 'x' height of the font. This is often but not always the same as the height of the character 'x'.
        #[rust_name = "x_height"]
        fn xHeight(self: &QFontMetrics) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qfontmetrics_drop"]
        fn drop(fontmetics: &mut QFontMetrics);

        #[doc(hidden)]
        #[rust_name = "qfontmetrics_clone"]
        fn construct(font: &QFontMetrics) -> QFontMetrics;

        #[doc(hidden)]
        #[rust_name = "qfontmetrics_init_from_qfont"]
        fn construct(font: &QFont) -> QFontMetrics;
    }
}

#[repr(C)]
pub struct QFontMetrics {
    _cspec: MaybeUninit<i32>,
}

impl Drop for QFontMetrics {
    fn drop(&mut self) {
        ffi::qfontmetrics_drop(self);
    }
}

impl Clone for QFontMetrics {
    fn clone(&self) -> Self {
        ffi::qfontmetrics_clone(self)
    }
}

impl From<&ffi::QFont> for QFontMetrics {
    fn from(font: &ffi::QFont) -> Self {
        ffi::qfontmetrics_init_from_qfont(font)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QFontMetrics {
    type Id = type_id!("QFontMetrics");
    type Kind = cxx::kind::Trivial;
}
