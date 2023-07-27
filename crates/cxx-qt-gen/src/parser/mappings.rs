// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeMap;

use quote::format_ident;
use syn::{spanned::Spanned, Attribute, Ident, Path, Result};

use crate::syntax::{attribute::attribute_find_path, expr::expr_to_string, path::path_from_idents};

#[derive(Default)]
pub struct ParsedCxxMappings {
    /// Map of the cxx_name of any types defined in CXX extern blocks
    ///
    /// This is used in the C++ generation to map the Rust type name to the C++ name
    pub cxx_names: BTreeMap<String, String>,
    /// Map of the namespace of any types or methods defined in CXX extern blocks
    ///
    /// This is used in the C++ generation to map the Rust type name to the C++ name
    pub namespaces: BTreeMap<String, String>,
    /// Mappings for CXX types when used outside the bridge
    ///
    /// This is used in the Rust generation to map the bridge type A to ffi::B
    pub qualified: BTreeMap<Ident, Path>,
}

impl ParsedCxxMappings {
    /// For a given rust ident return the CXX name with its namespace
    pub fn cxx(&self, ident: &str) -> String {
        // Check if there is a cxx_name or namespace to handle
        let cxx_name = self
            .cxx_names
            .get(ident)
            .cloned()
            .unwrap_or_else(|| ident.to_owned());

        if let Some(namespace) = self.namespaces.get(ident) {
            format!("::{namespace}::{cxx_name}")
        } else {
            cxx_name
        }
    }

    /// Helper which builds mappings from namespace, cxx_name, and rust_name attributes
    pub fn populate(
        &mut self,
        ident: &Ident,
        attrs: &[Attribute],
        parent_namespace: &str,
        module_ident: &Ident,
    ) -> Result<()> {
        // Find if there is a namespace (for C++ generation)
        let namespace = if let Some(index) = attribute_find_path(attrs, &["namespace"]) {
            expr_to_string(&attrs[index].meta.require_name_value()?.value)?
        } else {
            parent_namespace.to_string()
        };

        if !namespace.is_empty() {
            self.namespaces.insert(ident.to_string(), namespace);
        }

        // Find if there is a cxx_name mapping (for C++ generation)
        if let Some(index) = attribute_find_path(attrs, &["cxx_name"]) {
            self.cxx_names.insert(
                ident.to_string(),
                expr_to_string(&attrs[index].meta.require_name_value()?.value)?,
            );
        }

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

        // Add the rust_ident to qualified mappings (for Rust generation using ffi::T)
        self.qualified
            .insert(ident.clone(), path_from_idents(module_ident, &rust_ident));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quote::format_ident;
    use syn::parse_quote;

    #[test]
    fn test_attribute_none() {
        let mut mappings = ParsedCxxMappings::default();
        assert!(mappings
            .populate(&format_ident!("A"), &[], "", &format_ident!("ffi"))
            .is_ok());

        assert!(mappings.cxx_names.is_empty());
        assert!(mappings.namespaces.is_empty());
        assert_eq!(mappings.qualified.len(), 1);
        assert_eq!(
            mappings.qualified.get(&format_ident!("A")).unwrap(),
            &parse_quote! { ffi::A }
        );
    }

    #[test]
    fn test_attribute_cxx_name() {
        let mut mappings = ParsedCxxMappings::default();
        assert!(mappings
            .populate(
                &format_ident!("A"),
                &[parse_quote! { #[cxx_name = "B"] }],
                "",
                &format_ident!("ffi")
            )
            .is_ok());

        assert_eq!(mappings.cxx_names.len(), 1);
        assert_eq!(mappings.cxx_names.get("A").unwrap(), "B");
        assert!(mappings.namespaces.is_empty());
        assert_eq!(
            mappings.qualified.get(&format_ident!("A")).unwrap(),
            &parse_quote! { ffi::A }
        );
    }

    #[test]
    fn test_attribute_namespace() {
        let mut mappings = ParsedCxxMappings::default();
        assert!(mappings
            .populate(
                &format_ident!("A"),
                &[parse_quote! { #[namespace = "type_namespace"] }],
                "bridge_namespace",
                &format_ident!("ffi")
            )
            .is_ok());

        assert!(mappings.cxx_names.is_empty());
        assert_eq!(mappings.namespaces.get("A").unwrap(), "type_namespace");
        assert_eq!(mappings.qualified.len(), 1);
        assert_eq!(
            mappings.qualified.get(&format_ident!("A")).unwrap(),
            &parse_quote! { ffi::A }
        );
    }

    #[test]
    fn test_attribute_rust_name() {
        let mut mappings = ParsedCxxMappings::default();
        assert!(mappings
            .populate(
                &format_ident!("A"),
                &[parse_quote! { #[rust_name = "B"] }],
                "",
                &format_ident!("ffi")
            )
            .is_ok());

        assert!(mappings.cxx_names.is_empty());
        assert!(mappings.namespaces.is_empty());
        assert_eq!(mappings.qualified.len(), 1);
        assert_eq!(
            mappings.qualified.get(&format_ident!("A")).unwrap(),
            &parse_quote! { ffi::B }
        );
    }

    #[test]
    fn test_parent_namespace() {
        let mut mappings = ParsedCxxMappings::default();
        assert!(mappings
            .populate(
                &format_ident!("A"),
                &[],
                "bridge_namespace",
                &format_ident!("ffi")
            )
            .is_ok());

        assert!(mappings.cxx_names.is_empty());
        assert_eq!(mappings.namespaces.get("A").unwrap(), "bridge_namespace");
        assert_eq!(mappings.qualified.len(), 1);
        assert_eq!(
            mappings.qualified.get(&format_ident!("A")).unwrap(),
            &parse_quote! { ffi::A }
        );
    }

    #[test]
    fn test_qualified() {
        let mut mappings = ParsedCxxMappings::default();
        assert!(mappings
            .populate(&format_ident!("A"), &[], "", &format_ident!("my_module"))
            .is_ok());

        assert!(mappings.cxx_names.is_empty());
        assert!(mappings.namespaces.is_empty());
        assert_eq!(mappings.qualified.len(), 1);
        assert_eq!(
            mappings.qualified.get(&format_ident!("A")).unwrap(),
            &parse_quote! { my_module::A }
        );
    }
}
