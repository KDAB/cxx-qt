// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use cxx::ExternType;

pub trait QFlagRepr: Sized {
    /// Qt chooses the integer representation for a `QFlags<T>` as follows:
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

/// # Safety
///
/// By writing the unsafe `QFlag` impl, the programmer asserts that the C++ namespace and type name
/// given in the type id refers to a `QFlags<T>` C++Qt type where T is equivalent to the Rust type
/// that is the `Repr` type of the impl.
///
/// Furthermore, the programmer asserts that `Repr` is the backing integer representation of the
/// enum type that is the `Self` type of the impl.
pub unsafe trait QFlag: Sized {
    /// A type-level representation of the C++ namespace and type name of this type's `QFlags<T>`.
    ///
    /// This will always be defined using `type_id!` in the following form:
    ///
    /// ```
    /// # struct MyType { repr: i32 };
    /// # unsafe impl cxx_qt_lib::QFlag for MyType {
    /// type TypeId = cxx::type_id!("QFlags_MyType");
    /// #     type Repr = i32;
    /// #     fn to_repr(self) -> Self::Repr { self.repr }
    /// # }
    /// ```
    type TypeId;
    /// The backing integer representation of the enum type that is the `Self` type of this impl.
    /// For example, if the enum is defined with `#[repr(i32)]`, then `Repr` should be `i32`.
    type Repr: QFlagRepr + ExternType<Kind = cxx::kind::Trivial>;

    /// Converts the enum type that is the `Self` type of this impl to its backing representation.
    ///
    /// This will always be defined in the following form:
    ///
    /// ```
    /// # struct MyType { repr: i32 };
    /// # unsafe impl cxx_qt_lib::QFlag for MyType {
    /// # type TypeId = cxx::type_id!("QFlags_MyType");
    /// #     type Repr = i32;
    /// fn to_repr(self) -> Self::Repr {
    ///     self.repr
    /// }
    /// # }
    /// ```
    fn to_repr(self) -> Self::Repr;
}

/// Internal utility trait for converting `T` in a `QFlag<T>` to the corresponding integer type.
pub trait QFlagExt: QFlag {
    type Int;

    fn to_int(self) -> Self::Int;
}

impl<T: QFlag> QFlagExt for T {
    type Int = <<T as QFlag>::Repr as QFlagRepr>::Int;

    #[inline(always)]
    fn to_int(self) -> Self::Int {
        self.to_repr().into()
    }
}
