// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod types;
pub use types::*;

// Provide a separate depending on the platform
// this is because include_str requires th correct and non-mixed path separators
//
// https://github.com/rust-lang/rust/issues/75075
#[cfg(not(windows))]
macro_rules! sep {
    () => {
        "/"
    };
}

#[cfg(windows)]
macro_rules! sep {
    () => {
        "\\"
    };
}

/// The header for qt_types
pub const QT_TYPES_HEADER: &str =
    include_str!(concat!("..", sep!(), "include", sep!(), "qt_types.h"));
/// The source for qt_types
pub const QT_TYPES_SOURCE: &str = include_str!("qt_types.cpp");

pub trait UpdateRequestHandler<C> {
    fn handle_update_request(&mut self, cpp: &mut C);
}

pub trait ToUniquePtr {
    type CppType;

    fn to_unique_ptr(self) -> cxx::UniquePtr<Self::CppType>
    where
        Self::CppType: cxx::memory::UniquePtrTarget;
}
