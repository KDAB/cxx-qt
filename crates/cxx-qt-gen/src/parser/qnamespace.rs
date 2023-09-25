// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{ItemMacro, LitStr, Result};

use crate::syntax::attribute::attribute_take_path;

pub struct ParsedQNamespace {
    /// The name of the namespace
    pub namespace: String,
    /// whether qml_element was specified
    pub qml_element: bool,
}

impl ParsedQNamespace {
    pub fn parse(mut mac: ItemMacro) -> Result<Self> {
        let namespace_literal: LitStr = syn::parse2(mac.mac.tokens)?;
        let namespace = namespace_literal.value();
        if namespace.contains(char::is_whitespace) {
            return Err(syn::Error::new_spanned(
                namespace_literal,
                "qnamespace! may not contain any whitespace!",
            ));
        }
        if namespace.is_empty() {
            return Err(syn::Error::new_spanned(
                namespace_literal,
                "qnamespace! may not be empty!",
            ));
        }

        let qml_element = attribute_take_path(&mut mac.attrs, &["qml_element"]).is_some();

        if let Some(attr) = mac.attrs.first() {
            return Err(syn::Error::new_spanned(
                attr,
                "qnamespace! macro must not have any attributes other than qml_element!",
            ));
        }

        if let Some(ident) = mac.ident {
            return Err(syn::Error::new_spanned(
                ident,
                "qnamespace! macro must not have an additional identifier",
            ));
        }

        Ok(Self {
            namespace,
            qml_element,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! parse_qnamespace {
        { $($input:tt)* } => {
            ParsedQNamespace::parse(syn::parse_quote! { $($input)* }).unwrap()
        }
    }

    #[test]
    fn parse_qnamespace() {
        let parsed = parse_qnamespace! {
            qnamespace!("my_namespace::test");
        };

        assert_eq!(parsed.namespace, "my_namespace::test");
        assert!(!parsed.qml_element);
    }

    #[test]
    fn parse_qml_element_namespaced() {
        let parsed = parse_qnamespace! {
            #[qml_element]
            qnamespace!("my_other_namespace");
        };

        assert_eq!(parsed.namespace, "my_other_namespace");
        assert!(parsed.qml_element);
    }

    macro_rules! assert_parse_error {
        { $($input:tt)* } => {
            assert!(ParsedQNamespace::parse(syn::parse_quote! { $($input)* }).is_err())
        }
    }

    #[test]
    fn parse_errors() {
        assert_parse_error! {
            qnamespace!(my_namespace);
        }
        assert_parse_error! {
            qnamespace!();
        }
        assert_parse_error! {
            #[my_attribute]
            qnamespace!("hello");
        }
        assert_parse_error! {
            qnamespace! test ("hello");
        }
        assert_parse_error! {
            qnamespace!("hello" "world");
        }
        assert_parse_error! {
            /// A doc comment
            qnamespace!("my_namespace");
        }
        assert_parse_error! {
            qnamespace!("");
        }
        assert_parse_error! {
            qnamespace!(" ");
        }
        assert_parse_error! {
            qnamespace!("my namespace");
        }
    }
}
