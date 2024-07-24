// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::naming::Name;
use crate::parser::method::ParsedMethod;
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::{ForeignItemFn, Ident};

/// Names for parts of a method (which could be a Q_INVOKABLE)
pub struct QMethodName {
    pub name: Name,
    pub wrapper: Name,
}

impl From<&ParsedMethod> for QMethodName {
    fn from(invokable: &ParsedMethod) -> Self {
        Self::from(&invokable.method)
    }
}

impl From<&ForeignItemFn> for QMethodName {
    fn from(method: &ForeignItemFn) -> Self {
        let ident = &method.sig.ident;
        let method_name =
            Name::from_rust_ident_and_attrs(&ident, &method.attrs, None, None).unwrap(); // Might need to add a way to get the namespace and module in here

        let wrapper = Name::wrapper_from(method_name.clone());

        Self {
            // TODO: URGENT add error propagation here
            name: method_name,
            wrapper,
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    use std::collections::HashSet;

    #[test]
    fn test_from_impl_method() {
        let parsed = ParsedMethod {
            method: parse_quote! {
                fn my_invokable(self: &MyObject);
            },
            qobject_ident: format_ident!("MyObject"),
            mutable: false,
            safe: true,
            parameters: vec![],
            specifiers: HashSet::new(),
            is_qinvokable: true,
        };

        let invokable = QMethodName::from(&parsed);
        assert_eq!(
            invokable.name.cxx_unqualified(),
            String::from("myInvokable")
        );
        assert_eq!(
            invokable.name.rust_unqualified().to_string(),
            String::from("my_invokable")
        );
        assert_eq!(
            invokable.wrapper.cxx_unqualified(),
            String::from("myInvokableWrapper")
        );
        assert_eq!(
            invokable.wrapper.rust_unqualified().to_string(),
            String::from("my_invokable_wrapper")
        );
    }
}
