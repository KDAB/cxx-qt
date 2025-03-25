// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[macro_export]
macro_rules! unsafe_impl_qflag {
    ( $typeName:ty, $typeId:literal, $repr:ident ) => {
        unsafe impl $crate::QFlag for $typeName {
            type TypeId = ::cxx::type_id!($typeId);
            type Repr = $repr;

            fn to_repr(self) -> Self::Repr {
                self.repr
            }
        }

        impl ::std::ops::BitOr for $typeName {
            type Output = $crate::QFlags<$typeName>;

            fn bitor(self, other: Self) -> Self::Output {
                $crate::QFlags::from(self) | other
            }
        }

        impl ::std::ops::BitOr<$crate::QFlags<$typeName>> for $typeName {
            type Output = $crate::QFlags<$typeName>;

            fn bitor(self, other: $crate::QFlags<$typeName>) -> Self::Output {
                other | self
            }
        }
    };
}
