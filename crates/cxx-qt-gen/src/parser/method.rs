// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{
    naming::Name,
    parser::{check_safety, parameter::ParsedFunctionParameter, parse_attributes},
    syntax::{foreignmod, safety::Safety, types},
};
use core::ops::Deref;
use std::collections::{BTreeMap, HashSet};
use syn::{Attribute, ForeignItemFn, Ident, Result};

/// Describes a C++ specifier for the Q_INVOKABLE
#[derive(Eq, Hash, PartialEq)]
pub enum ParsedQInvokableSpecifiers {
    Final,
    Override,
    Virtual,
}

impl ParsedQInvokableSpecifiers {
    fn as_str(&self) -> &str {
        match self {
            ParsedQInvokableSpecifiers::Final => "cxx_final",
            ParsedQInvokableSpecifiers::Override => "cxx_override",
            ParsedQInvokableSpecifiers::Virtual => "cxx_virtual",
        }
    }

    fn from_attrs(attrs: BTreeMap<&str, &Attribute>) -> HashSet<ParsedQInvokableSpecifiers> {
        let mut output = HashSet::new();
        for specifier in [
            ParsedQInvokableSpecifiers::Final,
            ParsedQInvokableSpecifiers::Override,
            ParsedQInvokableSpecifiers::Virtual,
        ] {
            if attrs.get(specifier.as_str()).is_some() {
                output.insert(specifier);
            }
        }
        output
    }
}

/// Describes a single method (which could be a Q_INVOKABLE) for a struct
pub struct ParsedMethod {
    /// The common fields which are available on all callable types
    pub method_fields: MethodFields,
    /// Any specifiers that declared on the invokable
    pub specifiers: HashSet<ParsedQInvokableSpecifiers>,
    /// Whether the method is qinvokable
    pub is_qinvokable: bool,
    // No docs field since the docs should be on the method implementation outside the bridge
    // This means any docs on the bridge declaration would be ignored
}

impl ParsedMethod {
    const ALLOWED_ATTRS: [&'static str; 6] = [
        "cxx_name",
        "rust_name",
        "qinvokable",
        "cxx_final",
        "cxx_override",
        "cxx_virtual",
    ];

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
            method_fields: MethodFields {
                mutable: true,
                ..self.method_fields
            },
            ..self
        }
    }

    #[cfg(test)]
    pub fn make_unsafe(self) -> Self {
        Self {
            method_fields: MethodFields {
                safe: false,
                ..self.method_fields
            },
            ..self
        }
    }

    #[cfg(test)]
    pub fn with_specifiers(self, specifiers: HashSet<ParsedQInvokableSpecifiers>) -> Self {
        Self { specifiers, ..self }
    }

    pub fn parse(method: ForeignItemFn, safety: Safety) -> Result<Self> {
        check_safety(&method, &safety)?;
        let fields = MethodFields::parse(method)?;
        let attrs = parse_attributes(&fields.method.attrs, &Self::ALLOWED_ATTRS)?;

        // Determine if the method is invokable
        let is_qinvokable = attrs.get("qinvokable").is_some();
        let specifiers = ParsedQInvokableSpecifiers::from_attrs(attrs);

        Ok(Self {
            method_fields: fields,
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
    pub method: ForeignItemFn,
    pub qobject_ident: Ident,
    pub mutable: bool,
    pub parameters: Vec<ParsedFunctionParameter>,
    pub safe: bool,
    pub name: Name,
}

impl MethodFields {
    pub fn parse(method: ForeignItemFn) -> Result<Self> {
        let self_receiver = foreignmod::self_type_from_foreign_fn(&method.sig)?;
        let (qobject_ident, mutability) = types::extract_qobject_ident(&self_receiver.ty)?;
        let mutable = mutability.is_some();

        let parameters = ParsedFunctionParameter::parse_all_ignoring_receiver(&method.sig)?;
        let safe = method.sig.unsafety.is_none();
        let name = Name::from_rust_ident_and_attrs(&method.sig.ident, &method.attrs, None, None)?;

        Ok(MethodFields {
            method,
            qobject_ident,
            mutable,
            parameters,
            safe,
            name,
        })
    }
}
