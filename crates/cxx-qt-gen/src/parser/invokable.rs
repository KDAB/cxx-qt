// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    parser::parameter::ParsedFunctionParameter,
    syntax::{attribute::*, implitemmethod::is_method_mutable},
};
use std::collections::HashSet;
use syn::{Ident, ImplItemMethod, LitStr, Result};

/// Describes a C++ specifier for the Q_INVOKABLE
#[derive(Eq, Hash, PartialEq)]
pub enum ParsedQInvokableSpecifiers {
    Final,
    Override,
    Virtual,
}

/// Describes a single Q_INVOKABLE for a struct
pub struct ParsedQInvokable {
    /// The original [syn::ImplItemMethod] of the invokable
    pub method: ImplItemMethod,
    /// Whether this invokable is mutable
    pub mutable: bool,
    /// The name of the C++ type for the return type if one has been specified
    pub return_cxx_type: Option<String>,
    /// The parameters of the invokable
    pub parameters: Vec<ParsedFunctionParameter>,
    /// Any specifiers that declared on the invokable
    pub specifiers: HashSet<ParsedQInvokableSpecifiers>,
}

impl ParsedQInvokable {
    pub fn try_parse(method: &ImplItemMethod) -> Result<Option<Self>> {
        let index = attribute_find_path(&method.attrs, &["qinvokable"]);

        if index.is_none() {
            return Ok(None);
        }
        Ok(Some(Self::parse(method, index.unwrap())?))
    }

    fn parse(method: &ImplItemMethod, index: usize) -> Result<Self> {
        // Parse any return_cxx_type in the qproperty macro
        let attrs_map = attribute_tokens_to_map::<Ident, LitStr>(
            &method.attrs[index],
            AttributeDefault::Some(|span| LitStr::new("", span)),
        )?;
        let return_cxx_type = attrs_map
            .get(&quote::format_ident!("return_cxx_type"))
            .map(|lit_str| lit_str.value());

        // Parse any C++ specifiers
        let mut specifiers = HashSet::new();
        if attrs_map.contains_key(&quote::format_ident!("cxx_final")) {
            specifiers.insert(ParsedQInvokableSpecifiers::Final);
        }
        if attrs_map.contains_key(&quote::format_ident!("cxx_override")) {
            specifiers.insert(ParsedQInvokableSpecifiers::Override);
        }
        if attrs_map.contains_key(&quote::format_ident!("cxx_virtual")) {
            specifiers.insert(ParsedQInvokableSpecifiers::Virtual);
        }

        // Determine if the invokable is mutable
        let mutable = is_method_mutable(&method.sig);

        // Read the signal inputs into parameter blocks
        let parameters = ParsedFunctionParameter::parse_all_without_receiver(&method.sig)?;

        // Remove the invokable attribute
        let mut method = method.clone();
        method.attrs.remove(index);
        Ok(ParsedQInvokable {
            method,
            mutable,
            parameters,
            return_cxx_type,
            specifiers,
        })
    }
}
