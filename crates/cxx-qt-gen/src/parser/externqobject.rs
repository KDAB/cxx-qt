// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::naming::Name;
use crate::parser::{parse_base_type, require_attributes, CaseConversion, CommonAttrs};
use syn::{ForeignItemType, Ident, Result};

/// A representation of a QObject to be generated in an extern C++ block
pub struct ParsedExternQObject {
    /// The name of the ExternQObject
    pub name: Name,
    /// Original declaration
    pub declaration: ForeignItemType,
    /// The base class of the struct
    pub base_class: Option<Ident>,
    /// Common attrs for this object
    pub common_attrs: CommonAttrs,
}

impl ParsedExternQObject {
    const ALLOWED_ATTRS: [&'static str; 7] = [
        "cfg",
        "doc",
        "cxx_name",
        "rust_name",
        "namespace",
        "qobject",
        "base",
    ];

    pub fn parse(
        ty: ForeignItemType,
        module_ident: &Ident,
        parent_namespace: Option<&str>,
    ) -> Result<ParsedExternQObject> {
        // TODO: (cfg everywhere) should these have docs kept with them in the generated code?
        let (attributes, common_attrs) = require_attributes(&ty.attrs, &Self::ALLOWED_ATTRS)?;

        let base_class = parse_base_type(&attributes)?;

        Ok(Self {
            name: Name::from_ident_and_attrs(
                &ty.ident,
                &ty.attrs,
                parent_namespace,
                Some(module_ident),
                CaseConversion::none(),
            )?,
            declaration: ty,
            base_class,
            common_attrs,
        })
    }
}
