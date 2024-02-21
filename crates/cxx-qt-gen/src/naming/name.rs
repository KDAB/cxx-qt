// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use quote::format_ident;
use syn::{spanned::Spanned, Attribute, Ident, Result};

use crate::syntax::{attribute::attribute_find_path, expr::expr_to_string};

/// This struct contains all names a certain syntax element may have
///
/// This includes the rust_name, cxx_name, as well as qualifications like
/// the rust module and C++ namespace.
#[derive(Debug)]
pub struct Name {
    pub(super) rust: Ident,
    pub(super) cxx: Option<String>,
    pub(super) module: Ident,
    pub(super) namespace: Option<String>,
}

impl Name {
    pub(super) fn from_ident_and_attrs(
        ident: &Ident,
        attrs: &[Attribute],
        parent_namespace: Option<&str>,
        module: &Ident,
    ) -> Result<Self> {
        // Find if there is a namespace (for C++ generation)
        let mut namespace = if let Some(index) = attribute_find_path(attrs, &["namespace"]) {
            Some(expr_to_string(
                &attrs[index].meta.require_name_value()?.value,
            )?)
        } else {
            parent_namespace.map(|namespace| namespace.to_owned())
        };

        // This is an important check as it allows for the namespace to be cleared by assigning an
        // empty namespace (i.e. #[namespace = ""])
        if let Some(namespace_name) = &namespace {
            if namespace_name.is_empty() {
                namespace = None;
            }
        }

        // Find if there is a cxx_name mapping (for C++ generation)
        let cxx_name = attribute_find_path(attrs, &["cxx_name"])
            .map(|index| -> Result<_> {
                expr_to_string(&attrs[index].meta.require_name_value()?.value)
            })
            .transpose()?;

        // Find if there is a rust_name mapping
        let rust_ident = if let Some(index) = attribute_find_path(attrs, &["rust_name"]) {
            format_ident!(
                "{}",
                expr_to_string(&attrs[index].meta.require_name_value()?.value)?,
                span = attrs[index].span()
            )
        } else {
            ident.clone()
        };

        Ok(Self {
            rust: rust_ident,
            cxx: cxx_name,
            namespace,
            module: module.clone(),
        })
    }
}
