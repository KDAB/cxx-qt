// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module provides utils for working with syn, CXX, and C++.
//!
//! Such as converting a [syn::Type] into a C++ string, or determining
//! if a type is unsafe in a CXX bridge.
//!
//! The idea of this module is that it should be independent as
//! these methods could be split out into a cxx-utils crate later on.

pub(crate) mod cpp;
mod name;
pub(crate) mod rust;
mod type_names;

pub use name::{AutoCamel, Name};
pub use type_names::TypeNames;
