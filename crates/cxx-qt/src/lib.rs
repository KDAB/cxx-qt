// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(missing_docs)]

//! This crate and its associated crates provide a framework for generating QObjects from Rust.
//!
//! See the [book](https://kdab.github.io/cxx-qt/book/) for more information.

pub use cxx_qt_macro::bridge;
pub use cxx_qt_macro::inherit;
pub use cxx_qt_macro::qobject;
pub use cxx_qt_macro::qsignals;

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
