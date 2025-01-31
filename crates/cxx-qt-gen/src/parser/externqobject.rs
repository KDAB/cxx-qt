// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::naming::Name;
use crate::parser::{require_attributes, CaseConversion};
use syn::{ForeignItemType, Ident, Result};

/// A representation of a QObject to be generated in an extern C++ block
pub struct ParsedExternQObject {
    /// The name of the ExternQObject
    pub name: Name,
    /// Original declaration
    pub declaration: ForeignItemType,
}

impl ParsedExternQObject {
    const ALLOWED_ATTRS: [&'static str; 6] = [
        "cxx_name",
        "rust_name",
        "namespace",
        "cfg",
        "doc",
        "qobject",
        // TODO: support base, qproperty etc here?
    ];

    pub fn parse(
        ty: ForeignItemType,
        module_ident: &Ident,
        parent_namespace: Option<&str>,
    ) -> Result<ParsedExternQObject> {
        require_attributes(&ty.attrs, &Self::ALLOWED_ATTRS)?;

        Ok(Self {
            name: Name::from_ident_and_attrs(
                &ty.ident,
                &ty.attrs,
                parent_namespace,
                Some(module_ident),
                CaseConversion::none(),
            )?,
            declaration: ty,
        })
    }
}
