// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    parser::parameter::ParsedFunctionParameter,
    syntax::{attribute::*, foreignmod, safety::Safety, types},
};
use std::collections::HashSet;
use syn::{spanned::Spanned, Error, ForeignItemFn, Ident, LitStr, Result};

/// Describes a C++ specifier for the Q_INVOKABLE
#[derive(Eq, Hash, PartialEq)]
pub enum ParsedQInvokableSpecifiers {
    Final,
    Override,
    Virtual,
}

/// Describes a single Q_INVOKABLE for a struct
pub struct ParsedQInvokable {
    /// The original [syn::ImplItemFn] of the invokable
    pub method: ForeignItemFn,
    /// The type of the self argument
    pub qobject_ident: Ident,
    /// Whether this invokable is mutable
    pub mutable: bool,
    /// Whether the method is safe to call.
    pub safe: bool,
    /// The parameters of the invokable
    pub parameters: Vec<ParsedFunctionParameter>,
    /// Any specifiers that declared on the invokable
    pub specifiers: HashSet<ParsedQInvokableSpecifiers>,
}

impl ParsedQInvokable {
    pub fn parse(mut method: ForeignItemFn, safety: Safety, index: usize) -> Result<Self> {
        if safety == Safety::Unsafe && method.sig.unsafety.is_none() {
            return Err(Error::new(
                method.span(),
                "Inherited methods must be marked as unsafe or wrapped in an `unsafe extern \"C++\"` block!",
            ));
        }

        // Parse any C++ specifiers
        let mut specifiers = HashSet::new();
        let attrs_map = attribute_tokens_to_map::<Ident, LitStr>(
            &method.attrs[index],
            AttributeDefault::Some(|span| LitStr::new("", span)),
        )?;
        if attrs_map.contains_key(&quote::format_ident!("cxx_final")) {
            specifiers.insert(ParsedQInvokableSpecifiers::Final);
        }
        if attrs_map.contains_key(&quote::format_ident!("cxx_override")) {
            specifiers.insert(ParsedQInvokableSpecifiers::Override);
        }
        if attrs_map.contains_key(&quote::format_ident!("cxx_virtual")) {
            specifiers.insert(ParsedQInvokableSpecifiers::Virtual);
        }
        method.attrs.remove(index);

        // Determine if the invokable is mutable
        let self_receiver = foreignmod::self_type_from_foreign_fn(&method.sig)?;
        let (qobject_ident, mutability) = types::extract_qobject_ident(&self_receiver.ty)?;
        let mutable = mutability.is_some();

        let parameters = ParsedFunctionParameter::parse_all_ignoring_receiver(&method.sig)?;

        let safe = method.sig.unsafety.is_none();

        Ok(ParsedQInvokable {
            method,
            qobject_ident,
            mutable,
            parameters,
            specifiers,
            safe,
        })
    }
}
