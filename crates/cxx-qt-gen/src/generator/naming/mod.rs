// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
pub mod invokable;
pub mod namespace;
pub mod property;
pub mod qobject;
pub mod signals;

use convert_case::{Case, Casing};
use quote::format_ident;
use syn::Ident;

/// Describes an ident which potentially has a different name in C++ and Rust
pub struct CombinedIdent {
    /// The ident for C++
    pub cpp: Ident,
    /// The ident for rust
    pub rust: Ident,
}

impl CombinedIdent {
    /// Generate a CombinedIdent from a rust function name.
    /// C++ will use the CamelCase version of the function name.
    pub fn from_rust_function(ident: Ident) -> Self {
        Self {
            cpp: format_ident!("{}", ident.to_string().to_case(Case::Camel)),
            rust: ident,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_rust_function_camel_case_conversion() {
        let ident = format_ident!("test_function");
        let combined = CombinedIdent::from_rust_function(ident.clone());
        assert_eq!(combined.cpp, format_ident!("testFunction"));
        assert_eq!(combined.rust, ident);
    }

    #[test]
    fn test_from_rust_function_single_word() {
        let ident = format_ident!("test");
        let combined = CombinedIdent::from_rust_function(ident.clone());
        assert_eq!(combined.cpp, ident);
        assert_eq!(combined.rust, ident);
    }
}
