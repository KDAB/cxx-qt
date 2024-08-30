// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    naming::Name,
    parser::parameter::ParsedFunctionParameter,
    syntax::{attribute::attribute_take_path, safety::Safety},
};
use std::collections::HashSet;
use syn::{Attribute, Error, ForeignItemFn, Ident, Result};

use crate::parser::{
    check_safety, extract_common_fields, separate_docs, Invokable, InvokableFields,
};
#[cfg(test)]
use quote::format_ident;

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
    /// The rust and cxx name of the function
    pub name: Name,
    /// All the docs (each line) of the method
    pub docs: Vec<Attribute>,
}

impl Invokable for &ParsedMethod {
    fn name(&self) -> &Name {
        &self.name
    }
}

impl ParsedMethod {
    pub fn parse(mut method: ForeignItemFn, safety: Safety) -> Result<Self> {
        check_safety(&method, &safety)?;

        let docs = separate_docs(&mut method);
        let invokable_fields = extract_common_fields(&method, docs)?;

        if invokable_fields.name.namespace().is_some() {
            return Err(Error::new_spanned(
                method.sig.ident,
                "Methods / QInvokables cannot have a namespace attribute",
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
                specifiers.insert(specifier); // Should a fn be able to be Override AND Virtual?
            }
        }

        Ok(ParsedMethod::from_invokable_fields(
            invokable_fields,
            method,
            specifiers,
            is_qinvokable,
        ))
    }

    fn from_invokable_fields(
        fields: InvokableFields,
        method: ForeignItemFn,
        specifiers: HashSet<ParsedQInvokableSpecifiers>,
        is_qinvokable: bool,
    ) -> Self {
        Self {
            method,
            qobject_ident: fields.qobject_ident,
            mutable: fields.mutable,
            safe: fields.safe,
            parameters: fields.parameters,
            specifiers,
            is_qinvokable,
            name: fields.name,
            docs: fields.docs,
        }
    }

    #[cfg(test)]
    pub fn from_method_and_params(
        method: &ForeignItemFn,
        parameters: Vec<ParsedFunctionParameter>,
    ) -> Self {
        ParsedMethod {
            method: method.clone(),
            qobject_ident: format_ident!("MyObject"),
            mutable: false,
            safe: true,
            parameters,
            specifiers: HashSet::new(),
            is_qinvokable: true,
            name: Name::from_rust_ident_and_attrs(&method.sig.ident, &method.attrs, None, None)
                .unwrap(),
            docs: vec![],
        }
    }

    #[cfg(test)]
    pub fn mut_from_method_and_params(
        method: &ForeignItemFn,
        parameters: Vec<ParsedFunctionParameter>,
    ) -> Self {
        ParsedMethod {
            mutable: true,
            ..ParsedMethod::from_method_and_params(method, parameters)
        }
    }
}
