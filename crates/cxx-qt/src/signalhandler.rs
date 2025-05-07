// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::ExternType;

/// A trait which describes the closure to be used with [CxxQtSignalHandler].
#[doc(hidden)]
pub trait CxxQtSignalHandlerClosure {
    /// The Id of the CXX type
    type Id;
    /// The type of the closure
    type FnType<'a>: ?Sized;
}

// A signal handler helper which is used to move a FnMut closure into C++
#[doc(hidden)]
#[repr(transparent)]
pub struct CxxQtSignalHandler<'a, T>
where
    T: CxxQtSignalHandlerClosure,
{
    closure: Box<T::FnType<'a>>,
}

impl<'a, T> CxxQtSignalHandler<'a, T>
where
    T: CxxQtSignalHandlerClosure,
{
    /// Create a new signal handler with the given closure
    //
    // Note that we cannot use From as we cannot infer the type in the caller
    pub fn new(closure: Box<T::FnType<'a>>) -> Self {
        Self { closure }
    }

    /// A mutable reference to the inner closure
    pub fn closure(&mut self) -> &mut Box<T::FnType<'a>> {
        &mut self.closure
    }
}

// Safety:
//
// Static checks on the C++ and Rust side to ensure the size is the same.
unsafe impl<T> ExternType for CxxQtSignalHandler<'_, T>
where
    T: CxxQtSignalHandlerClosure,
{
    type Kind = cxx::kind::Trivial;
    type Id = T::Id;
}
