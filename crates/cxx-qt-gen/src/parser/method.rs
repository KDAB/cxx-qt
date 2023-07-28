// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    parser::parameter::ParsedFunctionParameter,
    syntax::{attribute::attribute_take_path, foreignmod, safety::Safety, types},
};
use std::collections::HashSet;
use syn::{spanned::Spanned, Error, ForeignItemFn, Ident, Result};

/// Describes a C++ specifier for the Q_INVOKABLE
#[derive(Eq, Hash, PartialEq)]
pub enum ParsedQInvokableSpecifiers {
    Final,
    Override,
    Virtual,
}

impl ParsedQInvokableSpecifiers {
    fn as_str_slice(&self) -> &[&str] {
        match self {
            ParsedQInvokableSpecifiers::Final => &["cxx_final"],
            ParsedQInvokableSpecifiers::Override => &["cxx_override"],
            ParsedQInvokableSpecifiers::Virtual => &["cxx_virtual"],
        }
    }
}

/// Describes a single method (which could be a Q_INVOKABLE) for a struct
pub struct ParsedMethod {
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
    /// Whether the method is qinvokable
    pub is_qinvokable: bool,
}

impl ParsedMethod {
    pub fn parse(mut method: ForeignItemFn, safety: Safety) -> Result<Self> {
        if safety == Safety::Unsafe && method.sig.unsafety.is_none() {
            return Err(Error::new(
                method.span(),
                "Invokable methods must be marked as unsafe or wrapped in an `unsafe extern \"RustQt\"` block!",
            ));
        }

        // Determine if the method is invokable
        let is_qinvokable = attribute_take_path(&mut method.attrs, &["qinvokable"]).is_some();

        // Parse any C++ specifiers
        let mut specifiers = HashSet::new();
        for specifier in [
            ParsedQInvokableSpecifiers::Final,
            ParsedQInvokableSpecifiers::Override,
            ParsedQInvokableSpecifiers::Virtual,
        ] {
            if attribute_take_path(&mut method.attrs, specifier.as_str_slice()).is_some() {
                specifiers.insert(specifier);
            }
        }

        // Determine if the invokable is mutable
        let self_receiver = foreignmod::self_type_from_foreign_fn(&method.sig)?;
        let (qobject_ident, mutability) = types::extract_qobject_ident(&self_receiver.ty)?;
        let mutable = mutability.is_some();

        let parameters = ParsedFunctionParameter::parse_all_ignoring_receiver(&method.sig)?;

        let safe = method.sig.unsafety.is_none();

        Ok(ParsedMethod {
            method,
            qobject_ident,
            mutable,
            parameters,
            specifiers,
            safe,
            is_qinvokable,
        })
    }
}
