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
macro_rules! const_assert {
    ($x:expr $(,)?) => {
        const _: () = ::core::assert!($x);
    };
}

use crate::{DateFormat, QString};

/// Types that can be used to format and parse dates and times.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AnyDateFormat<'a> {
    DateFormat(DateFormat),
    QString(&'a QString),
}

impl From<DateFormat> for AnyDateFormat<'static> {
    fn from(value: DateFormat) -> Self {
        Self::DateFormat(value)
    }
}

impl<'a> From<&'a QString> for AnyDateFormat<'a> {
    fn from(value: &'a QString) -> Self {
        Self::QString(value)
    }
}
