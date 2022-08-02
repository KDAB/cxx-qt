// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::path::path_compare_str;
use std::iter::FromIterator;
use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    token::Paren,
    Attribute, Error, Ident, Result,
};

/// Representation of a list of idents in an attribute, eg attribute(A, B, C)
pub struct AttributeList {
    pub paren: Paren,
    pub items: Punctuated<Ident, Comma>,
}

impl Parse for AttributeList {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let paren = parenthesized!(content in input);
        let items = content.parse_terminated(Ident::parse_any)?;
        Ok(AttributeList { paren, items })
    }
}

/// Returns the first [syn::Attribute] that matches a given path
pub fn attribute_find_path<'a>(attrs: &'a [Attribute], path: &[&str]) -> Option<&'a Attribute> {
    for attr in attrs {
        if path_compare_str(&attr.path, path) {
            return Some(attr);
        }
    }

    None
}

/// Returns the [syn::Ident] T from attribute(T) and errors if there is none or many
pub fn attribute_tokens_to_ident(attr: &Attribute) -> Result<Ident> {
    let attrs = attribute_tokens_to_list(attr)?;
    if attrs.len() == 1 {
        Ok(attrs[0].clone())
    } else {
        Err(Error::new(
            attr.span(),
            "Expected only one ident in the attribute",
        ))
    }
}

/// Returns the list of [syn::Ident]'s A, B, C from attribute(A, B, C)
/// and errors if there is a parser error
pub fn attribute_tokens_to_list(attr: &Attribute) -> Result<Vec<Ident>> {
    let attrs: AttributeList = syn::parse2(attr.tokens.clone())?;
    Ok(Vec::from_iter(attrs.items.into_iter()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::quote;
    use syn::ItemMod;

    #[test]
    fn test_attribute_find_path() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[invokable]
            #[cxx_qt::bridge]
            #[cxx_qt::signals(MyObject)]
            #[cxx_qt::bridge(namespace = "my::namespace")]
            mod module;
        });

        assert!(attribute_find_path(&module.attrs, &["invokable"]).is_some());
        assert!(attribute_find_path(&module.attrs, &["cxx_qt", "bridge"]).is_some());
        assert!(attribute_find_path(&module.attrs, &["cxx_qt", "signals"]).is_some());
        assert!(attribute_find_path(&module.attrs, &["cxx_qt", "missing"]).is_none());
    }

    #[test]
    fn test_attribute_tokens_to_ident() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[invokable]
            #[cxx_qt::bridge]
            #[cxx_qt::signals(MyObject)]
            #[cxx_qt::bridge(namespace = "my::namespace")]
            #[cxx_qt::list(A, B, C)]
            #[cxx_qt::empty()]
            mod module;
        });

        assert!(attribute_tokens_to_ident(&module.attrs[0]).is_err());
        assert!(attribute_tokens_to_ident(&module.attrs[1]).is_err());
        assert!(attribute_tokens_to_ident(&module.attrs[2]).is_ok());
        assert_eq!(
            attribute_tokens_to_ident(&module.attrs[2]).unwrap(),
            "MyObject"
        );
        assert!(attribute_tokens_to_ident(&module.attrs[3]).is_err());
        assert!(attribute_tokens_to_ident(&module.attrs[4]).is_err());
        assert!(attribute_tokens_to_ident(&module.attrs[5]).is_err());
    }

    #[test]
    fn test_attribute_tokens_to_list() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[invokable]
            #[cxx_qt::bridge]
            #[cxx_qt::signals(MyObject)]
            #[cxx_qt::bridge(namespace = "my::namespace")]
            #[cxx_qt::list(A, B, C)]
            mod module;
        });

        assert!(attribute_tokens_to_list(&module.attrs[0]).is_err());
        assert!(attribute_tokens_to_list(&module.attrs[1]).is_err());
        assert!(attribute_tokens_to_list(&module.attrs[2]).is_ok());
        assert_eq!(attribute_tokens_to_list(&module.attrs[2]).unwrap().len(), 1);
        assert_eq!(
            attribute_tokens_to_list(&module.attrs[2]).unwrap()[0],
            "MyObject"
        );
        assert!(attribute_tokens_to_list(&module.attrs[3]).is_err());
        assert!(attribute_tokens_to_list(&module.attrs[4]).is_ok());
        assert_eq!(attribute_tokens_to_list(&module.attrs[4]).unwrap().len(), 3);
        assert_eq!(attribute_tokens_to_list(&module.attrs[4]).unwrap()[0], "A");
        assert_eq!(attribute_tokens_to_list(&module.attrs[4]).unwrap()[1], "B");
        assert_eq!(attribute_tokens_to_list(&module.attrs[4]).unwrap()[2], "C");
    }
}
