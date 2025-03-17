// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::ExternType;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

mod qflag;
pub use qflag::QFlag;
use qflag::{QFlagExt, QFlagRepr};

mod util;

type QFlagInt<T> = <T as QFlagExt>::Int;

/// The QFlags<T> class is a template class, where T is an enum type.
/// QFlags is used throughout Qt for storing combinations of enum values.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct QFlags<T: QFlag> {
    repr: QFlagInt<T>,
}

impl<T: QFlag> Copy for QFlags<T> {}

impl<T: QFlag> Clone for QFlags<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: QFlag> From<T> for QFlags<T> {
    /// Returns the value stored in the QFlags object as an integer.
    fn from(value: T) -> Self {
        Self {
            repr: value.to_int(),
        }
    }
}

impl<T: QFlag> Default for QFlags<T> {
    /// Constructs an empty QFlags object.
    fn default() -> Self {
        Self::new()
    }
}

impl<T: QFlag> QFlags<T> {
    /// Constructs an empty QFlags object.
    pub const fn new() -> Self {
        Self::from_int(T::Repr::ZERO)
    }

    /// Constructs a QFlags object representing the integer value *i*.
    pub const fn from_int(i: QFlagInt<T>) -> Self {
        Self { repr: i }
    }

    /// Returns the value stored in the QFlags object as an integer.
    pub const fn to_int(self) -> QFlagInt<T> {
        self.repr
    }

    /// Returns `true` if no flag is set (i.e., if the value stored by the QFlags object is 0);
    /// otherwise returns `false`.
    pub fn is_empty(self) -> bool {
        self.repr == T::Repr::ZERO
    }

    /// Sets the flag *flag* if *on* is `true` or unsets it if *on* is `false`.
    /// Returns a mutable reference to this object.
    pub fn set_flag(&mut self, flag: T, on: bool) -> &mut Self {
        if on {
            self.repr |= flag.to_int();
        } else {
            self.repr &= !flag.to_int();
        }
        self
    }

    /// Returns `true` if the flag `flag` is set, otherwise false.
    ///
    /// Note: if *flag* contains multiple bits set to 1 (for instance, if it's an enumerator equal
    /// to the bitwise-OR of other enumerators) then this function will return `true` if and only if
    /// all the bits are set in this flags object. On the other hand, if *flag* contains no bits set
    /// to 1 (that is, its value as a integer is 0), then this function will return `true` if and
    /// only if this flags object also has no bits set to 1.
    pub fn test_flag(self, flag: T) -> bool {
        self.test_flags(Self::from(flag))
    }

    /// Returns `true` if this *flags* object matches the given flags.
    ///
    /// If *flags* has any flags set, this flags object matches precisely if all flags set in
    /// *flags* are also set in this flags object. Otherwise, when *flags* has no flags set, this
    /// flags object only matches if it also has no flags set.
    pub fn test_flags(self, flags: Self) -> bool {
        if flags.is_empty() {
            self.is_empty()
        } else {
            self.repr & flags.repr == flags.repr
        }
    }

    #[cfg(cxxqt_qt_version_at_least_6_2)]
    /// Returns `true` if any flag set in *flag* is also set in this flags object, otherwise
    /// `false`. If *flag* has no flags set, the return will always be `false`.
    pub fn test_any_flag(self, flag: T) -> bool {
        self.test_any_flags(Self::from(flag))
    }

    #[cfg(cxxqt_qt_version_at_least_6_2)]
    /// Returns `true` if any flag set in *flags* is also set in this flags object, otherwise
    /// `false`. If *flags* has no flags set, the return will always be `false`.
    pub fn test_any_flags(self, flags: Self) -> bool {
        (self.repr & flags.repr) != T::Repr::ZERO
    }
}

impl<T: QFlag> Not for QFlags<T> {
    type Output = Self;

    /// Returns a QFlags object that contains the bitwise negation of this object.
    fn not(self) -> Self::Output {
        Self { repr: !self.repr }
    }
}

impl<T: QFlag> BitAnd for QFlags<T> {
    type Output = Self;

    /// Returns a QFlags object containing the result of the bitwise AND operation on this object
    /// and `mask`.
    fn bitand(self, mask: Self) -> Self::Output {
        Self {
            repr: self.repr & mask.repr,
        }
    }
}
impl<T: QFlag> BitAnd<T> for QFlags<T> {
    type Output = Self;

    /// Returns a QFlags object containing the result of the bitwise AND operation on this object
    /// and `mask`.
    fn bitand(self, mask: T) -> Self::Output {
        Self {
            repr: self.repr & mask.to_int(),
        }
    }
}
impl<T: QFlag> BitAndAssign for QFlags<T> {
    /// Performs a bitwise AND operation with mask and stores the result in this QFlags object.
    fn bitand_assign(&mut self, mask: Self) {
        self.repr &= mask.repr;
    }
}
impl<T: QFlag> BitAndAssign<T> for QFlags<T> {
    /// Performs a bitwise AND operation with mask and stores the result in this QFlags object.
    fn bitand_assign(&mut self, mask: T) {
        self.repr &= mask.to_int();
    }
}

impl<T: QFlag> BitXor for QFlags<T> {
    type Output = Self;

    /// Returns a QFlags object containing the result of the bitwise XOR operation on this object
    /// and `other`.
    fn bitxor(self, other: Self) -> Self::Output {
        Self {
            repr: self.repr ^ other.repr,
        }
    }
}
impl<T: QFlag> BitXor<T> for QFlags<T> {
    type Output = Self;

    /// Returns a QFlags object containing the result of the bitwise XOR operation on this object
    /// and `other`.
    fn bitxor(self, other: T) -> Self::Output {
        Self {
            repr: self.repr ^ other.to_int(),
        }
    }
}
impl<T: QFlag> BitXorAssign for QFlags<T> {
    /// Performs a bitwise XOR operation with `other` and stores the result in this QFlags object.
    fn bitxor_assign(&mut self, other: Self) {
        self.repr ^= other.repr;
    }
}
impl<T: QFlag> BitXorAssign<T> for QFlags<T> {
    /// Performs a bitwise XOR operation with `other` and stores the result in this QFlags object.
    fn bitxor_assign(&mut self, other: T) {
        self.repr ^= other.to_int();
    }
}

impl<T: QFlag> BitOr for QFlags<T> {
    type Output = Self;

    /// Returns a QFlags object containing the result of the bitwise OR operation on this object and
    /// `other`.
    fn bitor(self, other: Self) -> Self::Output {
        Self {
            repr: self.repr | other.repr,
        }
    }
}
impl<T: QFlag> BitOr<T> for QFlags<T> {
    type Output = Self;

    /// Returns a QFlags object containing the result of the bitwise OR operation on this object and
    /// `other`.
    fn bitor(self, other: T) -> Self::Output {
        Self {
            repr: self.repr | other.to_int(),
        }
    }
}
impl<T: QFlag> BitOrAssign for QFlags<T> {
    /// Performs a bitwise OR operation with `other` and stores the result in this QFlags object.
    fn bitor_assign(&mut self, other: Self) {
        self.repr |= other.repr;
    }
}
impl<T: QFlag> BitOrAssign<T> for QFlags<T> {
    /// Performs a bitwise OR operation with `other` and stores the result in this QFlags object.
    fn bitor_assign(&mut self, mask: T) {
        self.repr |= mask.to_int();
    }
}

impl<T: QFlag> FromIterator<T> for QFlags<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let repr = iter
            .into_iter()
            .fold(T::Repr::ZERO, |repr, item| repr | item.to_int());
        Self { repr }
    }
}

// Safety:
//
// Established by the `QFlag` contract.
unsafe impl<T: QFlag> ExternType for QFlags<T> {
    type Id = T::TypeId;

    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod test {
    use super::*;

    const ALL_KEYBOARD_MODIFIERS: &[KeyboardModifier] = &[
        KeyboardModifier::AltModifier,
        KeyboardModifier::ControlModifier,
        KeyboardModifier::GroupSwitchModifier,
        KeyboardModifier::KeypadModifier,
        KeyboardModifier::MetaModifier,
        KeyboardModifier::ShiftModifier,
    ];

    #[test]
    fn qflags_set_flag() {
        let mut flags = KeyboardModifiers::new();
        flags
            .set_flag(KeyboardModifier::AltModifier, true)
            .set_flag(KeyboardModifier::ControlModifier, true)
            .set_flag(KeyboardModifier::ShiftModifier, true)
            .set_flag(KeyboardModifier::AltModifier, false);
        let contained = ALL_KEYBOARD_MODIFIERS
            .iter()
            .copied()
            .filter(|&key| flags.test_flag(key))
            .collect::<Vec<_>>();
        assert_eq!(
            contained,
            vec![
                KeyboardModifier::ControlModifier,
                KeyboardModifier::ShiftModifier
            ]
        );
    }

    #[test]
    fn qflags_test_flags() {
        let flags = KeyboardModifier::ControlModifier
            | KeyboardModifier::ShiftModifier
            | KeyboardModifier::KeypadModifier;
        let mut other = KeyboardModifier::AltModifier
            | KeyboardModifier::ControlModifier
            | KeyboardModifier::KeypadModifier;
        assert!(!flags.test_flags(other));
        other.set_flag(KeyboardModifier::AltModifier, false);
        assert!(flags.test_flags(other));
    }

    #[cfg(cxxqt_qt_version_at_least_6_2)]
    #[test]
    fn qflags_test_any_flags() {
        let flags = KeyboardModifier::ControlModifier
            | KeyboardModifier::ShiftModifier
            | KeyboardModifier::KeypadModifier;
        let mut other = KeyboardModifier::AltModifier | KeyboardModifier::ControlModifier;
        assert!(flags.test_any_flags(other));
        other.set_flag(KeyboardModifier::ControlModifier, false);
        assert!(!flags.test_any_flags(other));
    }

    #[test]
    fn qflags_test_no_flags() {
        let mut flags = KeyboardModifiers::from(KeyboardModifier::AltModifier);
        assert!(!flags.test_flag(KeyboardModifier::NoModifier));
        flags.set_flag(KeyboardModifier::AltModifier, false);
        assert!(flags.test_flag(KeyboardModifier::NoModifier));
    }

    #[test]
    fn qflags_from_iter() {
        let flags = [
            KeyboardModifier::AltModifier,
            KeyboardModifier::MetaModifier,
            KeyboardModifier::ShiftModifier,
        ]
        .iter()
        .copied()
        .collect::<QFlags<_>>();
        assert_eq!(
            flags.to_int(),
            KeyboardModifier::AltModifier.repr
                | KeyboardModifier::MetaModifier.repr
                | KeyboardModifier::ShiftModifier.repr
        );
    }
}
