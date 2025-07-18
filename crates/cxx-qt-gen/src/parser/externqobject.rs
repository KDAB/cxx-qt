// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::naming::Name;
use crate::parser::attribute::{AttributeConstraint, ParsedAttributes};
use crate::parser::{parse_base_type, CaseConversion};
use syn::{ForeignItemType, Ident, Result};

/// A representation of a QObject to be generated in an extern C++ block
pub struct ParsedExternQObject {
    /// The name of the ExternQObject
    pub name: Name,
    /// Original declaration
    pub declaration: ForeignItemType,
    /// The base class of the struct
    pub base_class: Option<Ident>,
}

impl ParsedExternQObject {
    const ALLOWED_ATTRS: [(AttributeConstraint, &'static str); 7] = [
        (AttributeConstraint::Unique, "cxx_name"),
        (AttributeConstraint::Unique, "rust_name"),
        (AttributeConstraint::Unique, "namespace"),
        (AttributeConstraint::Duplicate, "cfg"),
        (AttributeConstraint::Duplicate, "doc"),
        (AttributeConstraint::Unique, "qobject"),
        (AttributeConstraint::Unique, "base"),
    ];

    pub fn parse(
        ty: ForeignItemType,
        module_ident: &Ident,
        parent_namespace: Option<&str>,
    ) -> Result<ParsedExternQObject> {
        // TODO: ATTR Can this be done without clone
        let attrs = ParsedAttributes::require_attributes(ty.attrs.clone(), &Self::ALLOWED_ATTRS)?;

        let base_class = parse_base_type(&attrs)?;

        Ok(Self {
            name: Name::from_ident_and_attrs(
                &ty.ident,
                &attrs.clone_attrs(),
                parent_namespace,
                Some(module_ident),
                CaseConversion::none(),
            )?,
            declaration: ty,
            base_class,
        })
    }
}
