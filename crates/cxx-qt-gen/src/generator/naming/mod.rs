// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
pub mod functions;
pub mod invokable;
pub mod namespace;
pub mod property;
pub mod qobject;
pub mod signals;

use syn::Ident;

/// Describes an ident which potentially has a different name in C++ and Rust
pub struct CombinedIdent {
    /// The ident for C++
    pub cpp: Ident,
    /// The ident for rust
    pub rust: Ident,
}
