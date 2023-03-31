// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::path::path_compare_str;
use proc_macro2::Span;
use quote::ToTokens;
use std::collections::HashMap;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream, Parser},
    spanned::Spanned,
    Attribute, Error, Ident, Meta, Result, Token,
};

/// Representation of a key and an optional value in an attribute, eg `attribute(key = value)` or `attribute(key)`
struct AttributeMapValue<K: Parse, V: Parse> {
    pub key: K,
    pub value: Option<V>,
}

impl<K: Parse, V: Parse> Parse for AttributeMapValue<K, V> {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input.parse::<K>()?;
        let value = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Some(input.parse::<V>()?)
        } else {
            None
        };
        Ok(AttributeMapValue { key, value })
    }
}

/// Returns the index of the first [syn::Attribute] that matches a given path
pub fn attribute_find_path(attrs: &[Attribute], path: &[&str]) -> Option<usize> {
    for (i, attr) in attrs.iter().enumerate() {
        if path_compare_str(attr.meta.path(), path) {
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
    attr.meta
        .require_list()?
        .parse_args_with(|input: ParseStream| -> Result<Vec<Ident>> {
            Ok(input
                .parse_terminated(Ident::parse_any, Token![,])?
                .into_iter()
                .collect::<Vec<Ident>>())
        })
}

/// Whether the attribute has a default value if there is one missing
///
/// This is useful in attribute maps where only a key may be specified.
pub enum AttributeDefault<V: Parse> {
    Some(fn(Span) -> V),
    None,
}

/// Returns a map of keys and values from an attribute, eg attribute(a = b, c = d)
///
/// A default value can be specified by using [AttributeDefault].
pub fn attribute_tokens_to_map<K: std::cmp::Eq + std::hash::Hash + Parse, V: Parse>(
    attr: &Attribute,
    default_value: AttributeDefault<V>,
) -> Result<HashMap<K, V>> {
    if let Meta::List(meta_list) = &attr.meta {
        meta_list.parse_args_with(|input: ParseStream| -> Result<HashMap<K, V>> {
            let mut map = HashMap::new();
            for item in input.parse_terminated(AttributeMapValue::parse, Token![,])? {
                if let std::collections::hash_map::Entry::Vacant(e) = map.entry(item.key) {
                    if let Some(value) = item.value {
                        e.insert(value);
                    } else if let AttributeDefault::Some(default_value) = default_value {
                        e.insert(default_value(attr.span()));
                    } else {
                        return Err(Error::new(attr.span(), "Attribute key is missing a value"));
                    }
                } else {
                    return Err(Error::new(attr.span(), "Duplicate keys in the attributes"));
                }
            }
            Ok(map)
        })
    } else {
        Ok(HashMap::default())
    }
}

/// Returns the value in a attribute, eg the value in #[key = value]
pub fn attribute_tokens_to_value<V: Parse>(attr: &Attribute) -> Result<V> {
    let parse_value = |input: ParseStream| -> Result<V> { input.parse::<V>() };
    parse_value.parse2(attr.meta.require_name_value()?.value.to_token_stream())
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
            #[qinvokable]
            #[cxx_qt::bridge]
            #[cxx_qt::qsignals(MyObject)]
            #[cxx_qt::bridge(namespace = "my::namespace")]
            mod module;
        });

        assert!(attribute_find_path(&module.attrs, &["qinvokable"]).is_some());
        assert!(attribute_find_path(&module.attrs, &["cxx_qt", "bridge"]).is_some());
        assert!(attribute_find_path(&module.attrs, &["cxx_qt", "qsignals"]).is_some());
        assert!(attribute_find_path(&module.attrs, &["cxx_qt", "missing"]).is_none());
    }

    #[test]
    fn test_attribute_tokens_to_ident() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[qinvokable]
            #[cxx_qt::bridge]
            #[cxx_qt::qsignals(MyObject)]
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
            #[qinvokable]
            #[cxx_qt::bridge]
            #[cxx_qt::qsignals(MyObject)]
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
            #[qinvokable]
            #[cxx_qt::bridge]
            #[cxx_qt::qsignals(MyObject)]
            #[cxx_qt::bridge(namespace = "my::namespace")]
            #[cxx_qt::list(A, B, C)]
            #[cxx_qt::bridge(a = "b", namespace = "my::namespace")]
            #[cxx_qt::bridge(a = "b", namespace = "my::namespace", namespace = "my::namespace")]
            #[cxx_qt::bridge()]
            #[qinvokable(cxx_override, return_cxx_type = "T")]
            mod module;
        });

        assert_eq!(
            attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[0], AttributeDefault::None)
                .unwrap()
                .len(),
            0
        );
        assert_eq!(
            attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[1], AttributeDefault::None)
                .unwrap()
                .len(),
            0
        );
        assert!(
            attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[2], AttributeDefault::None)
                .is_err()
        );

        let result =
            attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[3], AttributeDefault::None)
                .unwrap();
        let ident = format_ident!("namespace");
        assert_eq!(result.len(), 1);
        assert!(result.contains_key(&ident));
        assert_eq!(result[&ident].value(), "my::namespace");

        assert!(
            attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[4], AttributeDefault::None)
                .is_err()
        );

        let result =
            attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[5], AttributeDefault::None)
                .unwrap();
        let ident = format_ident!("namespace");
        assert_eq!(result.len(), 2);
        assert!(result.contains_key(&ident));
        assert_eq!(result[&ident].value(), "my::namespace");

        assert!(
            attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[6], AttributeDefault::None)
                .is_err()
        );

        let result =
            attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[7], AttributeDefault::None)
                .unwrap();
        assert_eq!(result.len(), 0);

        assert!(
            attribute_tokens_to_map::<Ident, LitStr>(&module.attrs[8], AttributeDefault::None)
                .is_err()
        );
        let result = attribute_tokens_to_map::<Ident, LitStr>(
            &module.attrs[8],
            AttributeDefault::Some(|span| LitStr::new("", span)),
        )
        .unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains_key(&format_ident!("cxx_override")));
        assert!(result.contains_key(&format_ident!("return_cxx_type")));
    }

    #[test]
    fn test_attribute_tokens_to_value() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_type = "QColor"]
            #[cxx_type]
            mod module;
        });

        assert_eq!(
            attribute_tokens_to_value::<LitStr>(&module.attrs[0])
                .unwrap()
                .value(),
            "QColor"
        );
        assert!(attribute_tokens_to_value::<LitStr>(&module.attrs[1]).is_err());
    }
}
