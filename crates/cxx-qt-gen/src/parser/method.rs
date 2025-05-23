// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::parser::CaseConversion;
use crate::{
    naming::Name,
    parser::{extract_cfgs, parameter::ParsedFunctionParameter, require_attributes},
    syntax::{foreignmod, types},
};
use core::ops::Deref;
use quote::format_ident;
use std::collections::{BTreeMap, HashSet};
use std::ops::DerefMut;
use syn::{Attribute, ForeignItemFn, Ident, Result};

/// Describes a C++ specifier for the Q_INVOKABLE
#[derive(Eq, Hash, PartialEq)]
pub enum ParsedQInvokableSpecifiers {
    Final,
    Override,
    Virtual,
    Pure,
}

impl ParsedQInvokableSpecifiers {
    fn as_str(&self) -> &str {
        match self {
            ParsedQInvokableSpecifiers::Final => "cxx_final",
            ParsedQInvokableSpecifiers::Override => "cxx_override",
            ParsedQInvokableSpecifiers::Virtual => "cxx_virtual",
            ParsedQInvokableSpecifiers::Pure => "cxx_pure",
        }
    }

    fn from_attrs(attrs: BTreeMap<&str, &Attribute>) -> HashSet<ParsedQInvokableSpecifiers> {
        let mut output = HashSet::new();
        for specifier in [
            ParsedQInvokableSpecifiers::Final,
            ParsedQInvokableSpecifiers::Override,
            ParsedQInvokableSpecifiers::Virtual,
            ParsedQInvokableSpecifiers::Pure,
        ] {
            if attrs.contains_key(specifier.as_str()) {
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
    /// Whether the method is a pure virtual method
    pub is_pure: bool,
    // No docs field since the docs should be on the method implementation outside the bridge
    // This means any docs on the bridge declaration would be ignored
    /// Cfgs for the method
    pub cfgs: Vec<Attribute>,
    /// Whether the block containing the method is safe or unsafe
    pub unsafe_block: bool,
}

impl ParsedMethod {
    const ALLOWED_ATTRS: [&'static str; 9] = [
        "cxx_name",
        "rust_name",
        "qinvokable",
        "cxx_final",
        "cxx_override",
        "cxx_virtual",
        "cxx_pure",
        "doc",
        "cfg",
    ];

    #[cfg(test)]
    pub fn mock_qinvokable(method: &ForeignItemFn) -> Self {
        Self {
            is_qinvokable: true,
            ..Self::parse(method.clone(), CaseConversion::none(), false).unwrap()
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

    pub fn parse(
        method: ForeignItemFn,
        auto_case: CaseConversion,
        unsafe_block: bool,
    ) -> Result<Self> {
        let fields = MethodFields::parse(method, auto_case)?;
        let attrs = require_attributes(&fields.method.attrs, &Self::ALLOWED_ATTRS)?;
        let cfgs = extract_cfgs(&fields.method.attrs);

        // Determine if the method is invokable
        let is_qinvokable = attrs.contains_key("qinvokable");
        let is_pure = attrs.contains_key("cxx_pure");
        let specifiers = ParsedQInvokableSpecifiers::from_attrs(attrs);

        Ok(Self {
            method_fields: fields,
            specifiers,
            is_qinvokable,
            is_pure,
            cfgs,
            unsafe_block,
        })
    }
}

impl Deref for ParsedMethod {
    type Target = MethodFields;

    fn deref(&self) -> &Self::Target {
        &self.method_fields
    }
}

impl DerefMut for ParsedMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.method_fields
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
    pub fn parse(method: ForeignItemFn, auto_case: CaseConversion) -> Result<Self> {
        let self_receiver = foreignmod::self_type_from_foreign_fn(&method.sig)?;
        let (qobject_ident, mutability) = types::extract_qobject_ident(&self_receiver.ty)?;
        let mutable = mutability.is_some();

        let parameters = ParsedFunctionParameter::parse_all_ignoring_receiver(&method.sig)?;
        let safe = method.sig.unsafety.is_none();
        let name =
            Name::from_ident_and_attrs(&method.sig.ident, &method.attrs, None, None, auto_case)?;

        Ok(MethodFields {
            method,
            qobject_ident,
            mutable,
            parameters,
            safe,
            name,
        })
    }

    pub(crate) fn self_unresolved(&self) -> bool {
        self.qobject_ident == format_ident!("Self")
    }
}
