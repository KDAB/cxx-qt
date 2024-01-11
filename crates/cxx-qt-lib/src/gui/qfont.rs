// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
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

        /// Returns true if the style() of the font is not QFont::StyleNormal
        fn italic(self: &QFont) -> bool;

        /// Returns true if kerning should be used when drawing text with this font.
        fn kerning(self: &QFont) -> bool;

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

        /// Returns true if strikeout has been set; otherwise returns false.
        fn strikeOut(self: &QFont) -> bool;

        /// Returns true if underline has been set; otherwise returns false.
        fn underline(self: &QFont) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

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
