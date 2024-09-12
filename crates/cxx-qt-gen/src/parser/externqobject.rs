// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::naming::Name;
use syn::{ForeignItemType, Ident, Result};

/// A representation of a QObject to be generated in an extern C++ block
pub struct ParsedExternQObject {
    /// The name of the ExternQObject
    pub name: Name,
    /// Original declaration
    pub declaration: ForeignItemType,
}

impl ParsedExternQObject {
    pub fn parse(
        ty: ForeignItemType,
        module_ident: &Ident,
        parent_namespace: Option<&str>,
    ) -> Result<ParsedExternQObject> {
        Ok(Self {
            name: Name::from_ident_and_attrs(
                &ty.ident,
                &ty.attrs,
                parent_namespace,
                Some(module_ident),
            )?,
            declaration: ty,
        })
    }
}
