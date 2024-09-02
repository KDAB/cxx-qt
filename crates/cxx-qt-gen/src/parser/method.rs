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
use std::ops::Deref;
use syn::{Error, ForeignItemFn, Ident, Result};

use crate::parser::check_safety;
use crate::syntax::{foreignmod, types};

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
    /// The common fields which are available on all callable types
    pub method_fields: MethodFields,
    /// Any specifiers that declared on the invokable
    pub specifiers: HashSet<ParsedQInvokableSpecifiers>,
    /// Whether the method is qinvokable
    pub is_qinvokable: bool,
}

impl ParsedMethod {
    #[cfg(test)]
    pub fn mock_qinvokable(method: &ForeignItemFn) -> Self {
        Self {
            is_qinvokable: true,
            ..Self::parse(method.clone(), Safety::Safe).unwrap()
        }
    }

    #[cfg(test)]
    pub fn make_mutable(self) -> Self {
        Self {
            mutable: true,
            ..self
        }
    }

    #[cfg(test)]
    pub fn make_unsafe(self) -> Self {
        Self {
            safe: false,
            ..self
        }
    }

    #[cfg(test)]
    pub fn with_specifiers(self, specifiers: HashSet<ParsedQInvokableSpecifiers>) -> Self {
        Self { specifiers, ..self }
    }

    pub fn parse(mut method: ForeignItemFn, safety: Safety) -> Result<Self> {
        check_safety(&method, &safety)?;

        let method_fields = MethodFields::parse(&method)?;

        if method_fields.name.namespace().is_some() {
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

        Ok(Self {
            method,
            method_fields,
            specifiers,
            is_qinvokable,
        })
    }
}

impl Deref for ParsedMethod {
    type Target = MethodFields;

    fn deref(&self) -> &Self::Target {
        &self.method_fields
    }
}

/// Struct with common fields between Invokable types.
/// These types are ParsedSignal, ParsedMethod and ParsedInheritedMethod
#[derive(Clone)]
pub struct MethodFields {
    pub qobject_ident: Ident,
    pub mutable: bool,
    pub parameters: Vec<ParsedFunctionParameter>,
    pub safe: bool,
    pub name: Name,
}

impl MethodFields {
    pub fn parse(method: &ForeignItemFn) -> Result<Self> {
        let self_receiver = foreignmod::self_type_from_foreign_fn(&method.sig)?;
        let (qobject_ident, mutability) = types::extract_qobject_ident(&self_receiver.ty)?;
        let mutable = mutability.is_some();

        let parameters = ParsedFunctionParameter::parse_all_ignoring_receiver(&method.sig)?;
        let safe = method.sig.unsafety.is_none();
        let name = Name::from_rust_ident_and_attrs(&method.sig.ident, &method.attrs, None, None)?;

        Ok(MethodFields {
            qobject_ident,
            mutable,
            parameters,
            safe,
            name,
        })
    }
}
