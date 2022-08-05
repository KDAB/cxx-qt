// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::path::path_compare_str;
use std::{collections::HashMap, iter::FromIterator};
use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Comma, Paren},
    Attribute, Error, Ident, Result, Token,
};

/// Representation of a list of idents in an attribute, eg attribute(A, B, C)
pub struct AttributeList {
    pub items: Punctuated<Ident, Comma>,
}

impl Parse for AttributeList {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let items = content.parse_terminated(Ident::parse_any)?;
        Ok(AttributeList { items })
    }
}

/// Representation of a key and value in an attribute, eg attribute(key = value)
pub struct AttributeMapValue<K: Parse, V: Parse> {
    pub key: K,
    pub value: V,
}

impl<K: Parse, V: Parse> Parse for AttributeMapValue<K, V> {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input.parse::<K>()?;
        input.parse::<Token![=]>()?;
        let value = input.parse::<V>()?;
        Ok(AttributeMapValue { key, value })
    }
}

/// Representation of a list of keys and values represented as a map from an attribute, eg attribute(a = b, c = d)
pub struct AttributeMap<K: Parse, V: Parse> {
    pub items: Option<Punctuated<AttributeMapValue<K, V>, Comma>>,
}

impl<K: Parse, V: Parse> Parse for AttributeMap<K, V> {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(AttributeMap {
            items: if input.peek(Paren) {
                let content;
                parenthesized!(content in input);
                Some(content.parse_terminated(AttributeMapValue::parse)?)
            } else {
                None
            },
        })
    }
}

/// Returns the index of the first [syn::Attribute] that matches a given path
pub fn attribute_find_path(attrs: &[Attribute], path: &[&str]) -> Option<usize> {
    for (i, attr) in attrs.iter().enumerate() {
        if path_compare_str(&attr.path, path) {
            return Some(i);
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

/// Returns a map of keys and values from an attribute, eg attribute(a = b, c = d)
pub fn attribute_tokens_to_map<K: std::cmp::Eq + std::hash::Hash + Parse, V: Parse>(
    attr: &Attribute,
) -> Result<HashMap<K, V>> {
    let attrs_map: AttributeMap<K, V> = syn::parse2(attr.tokens.clone())?;
    let mut map = HashMap::new();
    if let Some(items) = attrs_map.items {
        for item in items {
            if let std::collections::hash_map::Entry::Vacant(e) = map.entry(item.key) {
                e.insert(item.value);
            } else {
                return Err(Error::new(attr.span(), "Duplicate keys in the attributes"));
            }
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::{format_ident, quote};
    use syn::{Ident, ItemMod, LitStr};

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
            #[cxx_qt::list()]
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
        assert!(attribute_tokens_to_list(&module.attrs[5]).is_ok());
        assert_eq!(attribute_tokens_to_list(&module.attrs[5]).unwrap().len(), 0);
    }

    #[test]
    fn test_attribute_tokens_to_map() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[invokable]
            #[cxx_qt::bridge]
            #[cxx_qt::signals(MyObject)]
            #[cxx_qt::bridge(namespace = "my::namespace")]
            #[cxx_qt::list(A, B, C)]
            #[cxx_qt::bridge(a = "b", namespace = "my::namespace")]
            #[cxx_qt::bridge(a = "b", namespace = "my::namespace", namespace = "my::namespace")]
            #[cxx_qt::bridge()]
            mod module;
        });

        assert_eq!(
            attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[0])
                .unwrap()
                .len(),
            0
        );
        assert_eq!(
            attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[1])
                .unwrap()
                .len(),
            0
        );
        assert!(attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[2]).is_err());

        let result = attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[3]).unwrap();
        let ident = format_ident!("namespace");
        assert_eq!(result.len(), 1);
        assert!(result.contains_key(&ident));
        assert_eq!(result[&ident].value(), "my::namespace");

        assert!(attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[4]).is_err());

        let result = attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[5]).unwrap();
        let ident = format_ident!("namespace");
        assert_eq!(result.len(), 2);
        assert!(result.contains_key(&ident));
        assert_eq!(result[&ident].value(), "my::namespace");

        assert!(attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[6]).is_err());

        let result = attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[7]).unwrap();
        assert_eq!(result.len(), 0);
    }
}
