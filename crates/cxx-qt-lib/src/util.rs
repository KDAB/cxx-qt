// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Asserts that a boolean expression is true at compile time.
///
/// See [`core::assert!`] for more information.
///
/// ```compile_fail
/// const_assert!(5 == 4);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! const_assert {
    ($x:expr $(,)?) => {
        const _: () = ::core::assert!($x);
    };
}
