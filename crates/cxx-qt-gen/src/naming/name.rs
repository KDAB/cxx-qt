// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use quote::format_ident;
use syn::{spanned::Spanned, Attribute, Ident, Path, Result};

use crate::syntax::{attribute::attribute_find_path, expr::expr_to_string};

/// This struct contains all names a certain syntax element may have
///
/// This includes the rust_name, cxx_name, as well as qualifications like
/// the rust module and C++ namespace.
///
/// Naming in CXX can be rather complex.
/// The following Rules apply:
/// - If only a cxx_name **or** a rust_name is given, the identifier of the type/function will be
///     used for part that wasn't specified explicitly.
/// - If **both** attributes are present, the identifier itself is not used!
/// - The `rust_name` is always used to refer to the type within the bridge!.
#[derive(Debug, PartialEq, Eq)]
pub struct Name {
    /// The name of the type in Rust. This is also the name used to refer to the type within the
    /// bridge.
    /// Usually set by either the `rust_name` attribute or the identifier of the type.
    pub(super) rust: Ident,

    /// The name of the type in C++. This is used to refer to the type in C++.
    /// Usually set by the `cxx_name` attribute, or by using the identifier when specifying a `rust_name` attribute.
    pub(super) cxx: Option<String>,

    /// The module of the type in Rust.
    pub(super) module: Option<Path>,

    /// The namespace of the type in C++.
    /// Originates from the `namespace` attribute
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
        let mut cxx_name = attribute_find_path(attrs, &["cxx_name"])
            .map(|index| -> Result<_> {
                expr_to_string(&attrs[index].meta.require_name_value()?.value)
            })
            .transpose()?;

        // Find if there is a rust_name mapping
        let rust_ident = if let Some(index) = attribute_find_path(attrs, &["rust_name"]) {
            // If we have a rust_name, but no cxx_name, the original ident is the cxx_name.
            if cxx_name.is_none() {
                cxx_name = Some(ident.to_string());
            }

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
            module: Some(module.clone().into()),
        })
    }

    /// Get the unqualified name of the type in C++.
    /// This is either:
    /// - The cxx_name attribute value, if one is provided
    /// - The original ident, if no cxx_name was provided
    pub(super) fn cxx_unqualified(&self) -> String {
        self.cxx.clone().unwrap_or_else(|| self.rust.to_string())
    }
}
