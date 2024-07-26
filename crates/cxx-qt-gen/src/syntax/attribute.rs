// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::path::path_compare_str;
use syn::Attribute;

/// Returns the index of the first [syn::Attribute] that matches a given path
pub fn attribute_find_path(attrs: &[Attribute], path: &[&str]) -> Option<usize> {
    for (i, attr) in attrs.iter().enumerate() {
        if path_compare_str(attr.meta.path(), path) {
            return Some(i);
        }
    }

    None
}

/// Takes and returns the first [syn::Attribute] that matches a given path
pub fn attribute_take_path(attrs: &mut Vec<Attribute>, path: &[&str]) -> Option<Attribute> {
    attribute_find_path(attrs, path).map(|index| attrs.remove(index))
}

#[cfg(test)]
mod tests {
    use super::*;

    use syn::{parse_quote, ItemMod};

    #[test]
    fn test_attribute_find_path() {
        let module: ItemMod = parse_quote! {
            #[qinvokable]
            #[cxx_qt::bridge]
            #[cxx_qt::object(MyObject)]
            #[cxx_qt::bridge(namespace = "my::namespace")]
            mod module;
        };

        assert!(attribute_find_path(&module.attrs, &["qinvokable"]).is_some());
        assert!(attribute_find_path(&module.attrs, &["cxx_qt", "bridge"]).is_some());
        assert!(attribute_find_path(&module.attrs, &["cxx_qt", "object"]).is_some());
        assert!(attribute_find_path(&module.attrs, &["cxx_qt", "missing"]).is_none());
    }

    #[test]
    fn test_attribute_take_path() {
        let mut module: ItemMod = parse_quote! {
            #[qinvokable]
            #[cxx_qt::bridge]
            #[cxx_qt::object(MyObject)]
            #[cxx_qt::bridge(namespace = "my::namespace")]
            mod module;
        };

        assert_eq!(module.attrs.len(), 4);
        assert!(attribute_take_path(&mut module.attrs, &["qinvokable"]).is_some());
        assert_eq!(module.attrs.len(), 3);
    }
}
