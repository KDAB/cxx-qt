// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::require_attributes;
use syn::{ItemMacro, LitStr, Result};

pub struct ParsedQNamespace {
    /// The name of the namespace
    pub namespace: String,
    /// whether qml_element was specified
    pub qml_element: bool,
}

impl ParsedQNamespace {
    pub fn parse(mac: ItemMacro) -> Result<Self> {
        let attrs = require_attributes(&mac.attrs, &["qml_element"])?;
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

        let qml_element = attrs.contains_key("qml_element");

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

    use crate::tests::assert_parse_errors;

    #[test]
    fn parse_errors() {
        assert_parse_errors! {
            ParsedQNamespace::parse =>

            { qnamespace!(my_namespace); }
            { qnamespace!(); }
            {
                #[my_attribute]
                qnamespace!("hello");
            }
            { qnamespace! test ("hello"); }
            { qnamespace!("hello" "world"); }
            {
                /// A doc comment
                qnamespace!("my_namespace");
            }
            { qnamespace!(""); }
            { qnamespace!(" "); }
            { qnamespace!("my namespace"); }

        }
    }
}
