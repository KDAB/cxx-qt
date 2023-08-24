// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

// SPDX-License-Identifier: MIT OR Apache-2.0

use quote::format_ident;
use syn::{Attribute, Ident};

use crate::syntax::{attribute::attribute_find_path, expr::expr_to_string};

use super::CombinedIdent;

impl CombinedIdent {
    /// Generate a CombinedIdent from a rust function name.
    ///
    /// This will consider any cxx_name or rust_name attributes
    pub fn from_rust_function(attrs: &[Attribute], ident: &Ident) -> Self {
        let mut combined = Self {
            cpp: ident.clone(),
            rust: ident.clone(),
        };

        // Find any cxx_name
        if let Some(index) = attribute_find_path(attrs, &["cxx_name"]) {
            if let Ok(name_value) = &attrs[index].meta.require_name_value() {
                if let Ok(value_str) = expr_to_string(&name_value.value) {
                    combined.cpp = format_ident!("{value_str}");
                }
            }
        }

        // Find any rust_name
        if let Some(index) = attribute_find_path(attrs, &["rust_name"]) {
            if let Ok(name_value) = &attrs[index].meta.require_name_value() {
                if let Ok(value_str) = expr_to_string(&name_value.value) {
                    combined.rust = format_ident!("{value_str}");
                }
            }
        }

        combined
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, ForeignItemFn};

    use super::*;

    #[test]
    fn test_from_rust_function() {
        let method: ForeignItemFn = parse_quote! {
            fn test_function();
        };
        let combined = CombinedIdent::from_rust_function(&method.attrs, &method.sig.ident);
        assert_eq!(combined.cpp, format_ident!("test_function"));
        assert_eq!(combined.rust, format_ident!("test_function"));
    }

    #[test]
    fn test_from_rust_function_cxx_name() {
        let method: ForeignItemFn = parse_quote! {
            #[cxx_name = "testFunction"]
            fn test_function();
        };
        let combined = CombinedIdent::from_rust_function(&method.attrs, &method.sig.ident);
        assert_eq!(combined.cpp, format_ident!("testFunction"));
        assert_eq!(combined.rust, format_ident!("test_function"));
    }

    #[test]
    fn test_from_rust_function_rust_name() {
        let method: ForeignItemFn = parse_quote! {
            #[rust_name = "test_function"]
            fn testFunction();
        };
        let combined = CombinedIdent::from_rust_function(&method.attrs, &method.sig.ident);
        assert_eq!(combined.cpp, format_ident!("testFunction"));
        assert_eq!(combined.rust, format_ident!("test_function"));
    }
}
