// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use std::char::{CharTryFromError, TryFromCharError};
use std::fmt;

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qchar.h");
        #[allow(unused)]
        type QChar = super::QChar;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QChar {
    ucs: u16,
}

impl fmt::Display for QChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(c) = self.to_char() {
            c.fmt(f)
        } else {
            write!(f, "\\u{{{:X}}}", self.unicode())
        }
    }
}

impl fmt::Debug for QChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(c) = self.to_char() {
            c.fmt(f)
        } else {
            write!(f, "\'\\u{{{:X}}}\'", self.unicode())
        }
    }
}

impl QChar {
    /// Constructs a `QChar` from UTF-16 character `c`.
    pub const fn new(c: u16) -> Self {
        Self { ucs: c }
    }

    /// Constructs a `QChar` from a byte if it is valid ASCII (i.e. <= 127).
    pub const fn from_ascii(b: u8) -> Option<Self> {
        if b.is_ascii() {
            Some(Self { ucs: b as u16 })
        } else {
            None
        }
    }

    /// Converts a Rust `char` to a `QChar` if it is within range (i.e. <= [`u16::MAX`]).
    pub const fn from_char(c: char) -> Option<Self> {
        if c as u32 > u16::MAX as u32 {
            None
        } else {
            Some(Self::new(c as u16))
        }
    }

    /// Returns the numeric Unicode value of the `QChar`.
    pub const fn unicode(self) -> u16 {
        self.ucs
    }

    /// Returns `true` if the `QChar` contains a code point that is in either the high or the low part of the UTF-16 surrogate range (for example if its code point is in range `[0xd800..0xdfff]`); `false` otherwise.
    pub const fn is_surrogate(self) -> bool {
        self.ucs - 0xd800 < 2048
    }

    /// Returns `true` if the `QChar` is the high part of a UTF16 surrogate (for example if its code point is in range `[0xd800..0xdbff]`); `false` otherwise.
    pub const fn is_high_surrogate(self) -> bool {
        (self.ucs & 0xfc00) == 0xd800
    }

    /// Returns `true` if the `QChar` is the low part of a UTF16 surrogate (for example if its code point is in range `[0xdc00..0xdfff]`); `false` otherwise.
    pub const fn is_low_surrogate(self) -> bool {
        (self.ucs & 0xfc00) == 0xdc00
    }

    /// Converts a `QChar` to a Rust `char` if it is a valid character (i.e. not a surrogate).
    pub const fn to_char(self) -> Option<char> {
        char::from_u32(self.ucs as u32)
    }
}

impl From<u16> for QChar {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<QChar> for u16 {
    fn from(value: QChar) -> Self {
        value.unicode()
    }
}

impl TryFrom<char> for QChar {
    type Error = TryFromCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        u16::try_from(value).map(QChar::from)
    }
}

impl TryFrom<QChar> for char {
    type Error = CharTryFromError;

    fn try_from(value: QChar) -> Result<Self, Self::Error> {
        u32::from(value.unicode()).try_into()
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QChar {
    type Id = type_id!("QChar");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qchar_display_char() {
        let qchar = QChar::from_char('ᥱ').unwrap();
        assert_eq!(qchar.to_string(), "ᥱ");
    }

    #[test]
    fn qchar_display_surrogate() {
        let qchar = QChar::new(0xD834);
        assert_eq!(qchar.to_string(), "\\u{D834}");
    }

    #[test]
    fn qchar_debug_surrogate() {
        let qchar = QChar::new(0xD834);
        assert_eq!(format!("{qchar:?}"), "'\\u{D834}'");
    }
}
