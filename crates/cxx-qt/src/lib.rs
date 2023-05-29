// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(missing_docs)]

//! This crate and its associated crates provide a framework for generating QObjects from Rust.
//!
//! See the [book](https://kdab.github.io/cxx-qt/book/) for more information.

pub use cxx_qt_macro::bridge;
pub use cxx_qt_macro::qobject;

/// This trait is automatically implemented for all types which are marked as `#[cxx_qt::qobject]`.
/// It provides information about the type that is wrapped by the QObject, as well as the methods
/// that Cxx-Qt will generate for the QObject.
pub trait CxxQtType {
    /// The Rust type that this QObject is wrapping.
    type Rust;

    /// Retrieve an immutable reference to the Rust struct backing this C++ object
    fn rust(&self) -> &Self::Rust;

    /// Retrieve a mutable reference to the Rust struct backing this C++ object
    ///
    /// # Safety
    /// This method is unsafe because it allows a Q_PROPERTY to be modified without emitting its changed signal.
    /// The property changed signal must be emitted manually.
    unsafe fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust>;
}

/// Types which implement the `Locking` trait are guarded from concurrent access in C++ (the default in CxxQt).
///
/// # Safety
///
/// This is a marker trait used to disable locking.
///
/// By default, CxxQt will guard all access to the generated QObject with a recursive mutex.
/// For performance reasons it may be desirable to disable this behavior for certain QObjects.
/// You can do so by negative implementing this trait `unsafe impl !cxx_qt::Locking for qobject::T {}`.
///
/// However, this is unsafe, as it may lead to concurrent mutable access to the QObject from C++.
/// You are responsible for ensuring this does not happen!
//
// This could be implemented using an auto trait in the future once stable
// https://doc.rust-lang.org/beta/unstable-book/language-features/auto-traits.html
pub trait Locking {
    // empty
}

/// Indicates that the object implements threading and has a method which returns a CxxQtThread
pub trait Threading {
    /// Specialised version of CxxQtThread, which can be moved into other threads.
    ///
    /// CXX doesn't support having generic types in the function yet
    /// so we cannot have `CxxQtThread<T>` in cxx-qt-lib and then use that here
    /// For now we use a type alias in C++ then use it like a normal type here
    /// <https://github.com/dtolnay/cxx/issues/683>
    type Item;

    /// Create an instance of a CxxQtThread
    ///
    /// This allows for queueing closures onto the Qt event loop from a background thread.
    fn qt_thread(&self) -> Self::Item;
}
