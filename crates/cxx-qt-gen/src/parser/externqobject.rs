// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::naming::Name;
use crate::parser::{require_attributes, CaseConversion};
use syn::{Error, Expr, ForeignItemType, Ident, Result};

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
    const ALLOWED_ATTRS: [&'static str; 7] = [
        "cxx_name",
        "rust_name",
        "namespace",
        "cfg",
        "doc",
        "qobject",
        "base",
    ];

    pub fn parse(
        ty: ForeignItemType,
        module_ident: &Ident,
        parent_namespace: Option<&str>,
    ) -> Result<ParsedExternQObject> {
        let attributes = require_attributes(&ty.attrs, &Self::ALLOWED_ATTRS)?;

        let base_class = attributes
            .get("base")
            .map(|attr| -> Result<Ident> {
                let expr = &attr.meta.require_name_value()?.value;
                if let Expr::Path(path_expr) = expr {
                    Ok(path_expr.path.require_ident()?.clone())
                } else {
                    Err(Error::new_spanned(
                        expr,
                        "Base must be a identifier and cannot be empty!",
                    ))
                }
            })
            .transpose()?;

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
        })
    }
}
