// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use cxx::ExternType;

mod private {
    pub trait Sealed {}
}

pub trait QFlagRepr: Sized + private::Sealed {
    /// The underlying integer representation for a `QFlags<T>`.
    ///
    /// Qt chooses the integer representation as follows:
    ///
    /// - If `T` is signed, use a signed integer. Otherwise, use an unsigned integer.
    /// - If `T` is 32 bits or less, use a 32-bit integer.
    /// - If `T` is 64 bits and the Qt version is at least 6.9, use a 64-bit integer.
    type Int: From<Self>
        + Copy
        + Debug
        + Default
        + Eq
        + Ord
        + Hash
        + BitAnd<Output = Self::Int>
        + BitAndAssign
        + BitOr<Output = Self::Int>
        + BitOrAssign
        + BitXor<Output = Self::Int>
        + BitXorAssign
        + Not<Output = Self::Int>
        + ExternType<Kind = cxx::kind::Trivial>;

    const ZERO: Self::Int;
}

macro_rules! impl_repr {
    ($t:ty, $i:ty) => {
        impl private::Sealed for $t {}

        impl QFlagRepr for $t {
            type Int = $i;

            const ZERO: Self::Int = 0;
        }
    };
}

impl_repr!(i8, i32);
impl_repr!(i16, i32);
impl_repr!(i32, i32);
impl_repr!(u8, u32);
impl_repr!(u16, u32);
impl_repr!(u32, u32);

#[cfg(cxxqt_qt_version_at_least_6_9)]
impl_repr!(i64, i64);
#[cfg(cxxqt_qt_version_at_least_6_9)]
impl_repr!(u64, u64);
