// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::QFlagRepr;

use cxx::ExternType;

/// Trait implementation for an element in a [`QFlags`](crate::QFlags).
///
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
    /// This will always be defined using [cxx::type_id!].
    type TypeId;
    /// The backing integer representation of `Self`.
    /// For example, if the enum is defined with `#[repr(i32)]`, then `Repr` should be `i32`.
    type Repr: QFlagRepr + ExternType<Kind = cxx::kind::Trivial>;

    /// Converts the enum type that is the `Self` type of this impl to its backing representation.
    ///
    /// This will always be defined in the following form:
    ///
    /// ```ignore
    /// fn to_repr(self) -> Self::Repr {
    ///     self.repr
    /// }
    /// ```
    fn to_repr(self) -> Self::Repr;
}

/// Internal utility trait for converting `T` in a `QFlag<T>` to the corresponding integer type.
pub(super) trait QFlagExt: QFlag {
    fn to_int(self) -> <Self::Repr as QFlagRepr>::Int;
}

impl<T: QFlag> QFlagExt for T {
    #[inline(always)]
    fn to_int(self) -> <Self::Repr as QFlagRepr>::Int {
        self.to_repr().into()
    }
}
