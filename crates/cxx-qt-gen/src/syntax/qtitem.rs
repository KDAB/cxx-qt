// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::path::path_compare_str;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Item, ItemMod, Result, Token, Visibility};

#[derive(Clone, PartialEq, Eq)]
/// Representation of either a Syn Item, a CXX module, or a CXX-Qt module
pub enum CxxQtItem {
    /// A normal syntax item that we pass through
    Item(Item),
    /// A CXX module that we need to generate code for
    Cxx(ItemMod),
    /// A CxxQt module block that we need to parse and later generate code for
    CxxQt(ItemMod),
}

impl std::fmt::Debug for CxxQtItem {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CxxQtItem::Item(v0) => {
                let mut formatter = formatter.debug_tuple("Item");
                formatter.field(v0);
                formatter.finish()
            }
            CxxQtItem::Cxx(v0) => {
                let mut formatter = formatter.debug_tuple("Cxx");
                formatter.field(v0);
                formatter.finish()
            }
            CxxQtItem::CxxQt(v0) => {
                let mut formatter = formatter.debug_tuple("CxxQt");
                formatter.field(v0);
                formatter.finish()
            }
        }
    }
}

impl Parse for CxxQtItem {
    fn parse(input: ParseStream) -> Result<Self> {
        // Fork and skip over the attributes as we want to read the next token
        let ahead = input.fork();
        let attributes = ahead.call(Attribute::parse_outer)?;

        // See if the next token is a mod
        ahead.parse::<Visibility>()?;
        ahead.parse::<Option<Token![unsafe]>>()?;

        if ahead.peek(Token![mod]) {
            for attribute in &attributes {
                if path_compare_str(attribute.meta.path(), &["cxx", "bridge"]) {
                    return input.parse().map(CxxQtItem::Cxx);
                } else if path_compare_str(attribute.meta.path(), &["cxx_qt", "bridge"]) {
                    return input.parse().map(CxxQtItem::CxxQt);
                }
            }
        }

        // Fallback to using normal Item
        input.parse().map(CxxQtItem::Item)
    }
}

impl ToTokens for CxxQtItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            CxxQtItem::Item(item) => {
                item.to_tokens(tokens);
            }
            CxxQtItem::Cxx(module) => {
                module.to_tokens(tokens);
            }
            CxxQtItem::CxxQt(module) => {
                module.to_tokens(tokens);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;
    #[test]
    fn test_format_cxx() {
        let cxx: CxxQtItem = parse_quote! {
          #[cxx::bridge]
          mod ffi {}
        };
        let debug_formatted = format!("{:?}", cxx);
        assert!(debug_formatted.starts_with("Cxx(ItemMod"))
    }

    #[test]
    fn test_format_cxx_qt() {
        let cxx_qt: CxxQtItem = parse_quote! {
          #[cxx_qt::bridge]
          mod ffi {}
        };
        let debug_formatted = format!("{:?}", cxx_qt);
        assert!(debug_formatted.starts_with("CxxQt(ItemMod"))
    }

    #[test]
    fn test_format_non_cxx() {
        let cxx: CxxQtItem = parse_quote! {
            #[attr]
            mod ffi {}
        };
        let debug_formatted = format!("{:?}", cxx);
        assert!(debug_formatted.starts_with("Item(Item::Mod"))
    }

    #[test]
    fn test_format_rust_item() {
        let rust: CxxQtItem = parse_quote! {
          struct MyStruct {
                name: &str
          }
        };
        let debug_formatted = format!("{:?}", rust);
        assert!(debug_formatted.starts_with("Item(Item::Struct"))
    }
}
