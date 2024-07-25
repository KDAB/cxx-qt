// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::naming::Name;
use crate::parser::method::ParsedMethod;
use syn::ForeignItemFn;

/// Names for parts of a method (which could be a Q_INVOKABLE)
pub struct QMethodName {
    pub name: Name,
    pub wrapper: Name,
}

impl TryFrom<&ParsedMethod> for QMethodName {
    type Error = syn::Error;

    fn try_from(invokable: &ParsedMethod) -> Result<Self, Self::Error> {
        Self::try_from(&invokable.method)
    }
}

impl TryFrom<&ForeignItemFn> for QMethodName {
    type Error = syn::Error;

    fn try_from(method: &ForeignItemFn) -> Result<Self, Self::Error> {
        let method_name =
            Name::from_rust_ident_and_attrs(&method.sig.ident, &method.attrs, None, None)?; // Might need to add a way to get the namespace and module in here
        let wrapper = Name::wrapper_from(&method_name);

        Ok(Self {
            name: method_name,
            wrapper,
        })
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, ForeignItemFn};

    use super::*;

    use std::collections::HashSet;

    #[test]
    fn test_from_impl_method() {
        let method: ForeignItemFn = parse_quote! {
            fn my_invokable(self: &MyObject);
        };
        let parsed = ParsedMethod {
            method: method.clone(),
            qobject_ident: format_ident!("MyObject"),
            mutable: false,
            safe: true,
            parameters: vec![],
            specifiers: HashSet::new(),
            is_qinvokable: true,
            name: Name::from_rust_ident_and_attrs(&method.sig.ident, &method.attrs, None, None)
                .unwrap(),
        };

        let invokable = QMethodName::try_from(&parsed).unwrap();
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
