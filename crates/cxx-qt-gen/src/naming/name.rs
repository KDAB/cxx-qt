// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use convert_case::{Case, Casing};
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
#[derive(Clone, Debug, PartialEq, Eq)]
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
    /// Create a new name with the given rust name and no other attributes.
    pub fn new(rust_name: Ident) -> Self {
        Self {
            rust: rust_name,
            cxx: None,
            module: None,
            namespace: None,
        }
    }

    /// Create a new name from this name, with the given cxx_name
    pub fn with_cxx_name(self, cxx_name: String) -> Self {
        Self {
            cxx: Some(cxx_name),
            ..self
        }
    }

    /// Create a new name from this name, with the given Rust module
    pub fn with_module(self, module: Path) -> Self {
        Self {
            module: Some(module),
            ..self
        }
    }

    /// Create a new name from this name, with the given C++ namespace
    pub fn with_namespace(self, namespace: String) -> Self {
        Self {
            namespace: Some(namespace),
            ..self
        }
    }

    /// Create a new name from this name, with the given rust_name namespace
    ///
    /// Note: If no cxx_name is set explicitly, this will also change the cxx_name.
    pub fn with_rust_name(self, rust_name: Ident) -> Self {
        Self {
            rust: rust_name,
            ..self
        }
    }

    /// For a given Name generate the Rust and C++ wrapper names
    pub fn wrapper_from(self) -> Self {
        const CXX_WRAPPER_SUFFIX: &str = "Wrapper";
        const RUST_WRAPPER_SUFFIX: &str = "_wrapper";

        let rust_name = self.rust.clone();
        let cxx = self.cxx.clone();

        let cxx_name = if let Some(name) = cxx {
            format!("{name}{CXX_WRAPPER_SUFFIX}")
        } else {
            let camel_case_name = rust_name.to_string().to_case(Case::Camel);
            format!("{camel_case_name}{CXX_WRAPPER_SUFFIX}")
        };

        Self {
            rust: format_ident!("{rust_name}{RUST_WRAPPER_SUFFIX}"),
            cxx: Some(cxx_name),
            ..self
        }
    }

    /// Parse a name from an an identifier and a list of attributes.
    ///
    /// This variant assumes that the name is contained in an `extern "Rust"` block.
    /// If no cxx_name is set, it generates a camelCase cxx_name from the rust name.
    ///
    /// See also: [Self::from_ident_and_attrs]
    pub fn from_rust_ident_and_attrs(
        ident: &Ident,
        attrs: &[Attribute],
        parent_namespace: Option<&str>,
        module: Option<&Ident>,
    ) -> Result<Self> {
        let name = Self::from_ident_and_attrs(ident, attrs, parent_namespace, module)?;
        // No explicit cxx_name set, generate an appropriate camelCase cxx_name
        if name.cxx.is_none() {
            let rust_string = name.rust.to_string();
            let cxx = rust_string.to_case(Case::Camel);
            if cxx != rust_string {
                return Ok(name.with_cxx_name(cxx));
            }
        }
        Ok(name)
    }

    /// Parse a name from an identifier and a list of attributes.
    ///
    /// This deciphers the rust_name, cxx_name and namespace attributes, including
    /// inheriting the namespace from the parent.
    pub fn from_ident_and_attrs(
        ident: &Ident,
        attrs: &[Attribute],
        parent_namespace: Option<&str>,
        module: Option<&Ident>,
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
            module: module.cloned().map(Path::from),
        })
    }

    /// Get the unqualified name of the type in C++.
    /// This is either:
    /// - The cxx_name attribute value, if one is provided
    /// - The original ident, if no cxx_name was provided
    pub fn cxx_unqualified(&self) -> String {
        self.cxx.clone().unwrap_or_else(|| self.rust.to_string())
    }

    /// Get the unqualified name of the type in Rust.
    /// This is either;
    /// - The rust_name attribute value, if one is provided
    /// - The original ident, if no rust_name was provided
    pub fn rust_unqualified(&self) -> &Ident {
        &self.rust
    }

    /// Get the qualified name of this type in Rust, including its source module.
    pub fn rust_qualified(&self) -> Path {
        if let Some(module) = &self.module {
            let mut qualified_ident = module.clone();
            qualified_ident.segments.push(self.rust.clone().into());
            qualified_ident
        } else {
            Path::from(self.rust.clone())
        }
    }

    /// Set the namespace to the given value.
    ///
    /// Returns the previous value of the namespace.
    pub fn set_namespace(&mut self, mut namespace: Option<String>) -> Option<String> {
        std::mem::swap(&mut self.namespace, &mut namespace);
        namespace
    }

    /// Get the namespace of the type in C++.
    /// This is either:
    /// - The namespace attribute value, if one is provided
    /// - The surrounding namespace of the type, if any
    ///
    /// Note that there is a subtle difference between `None` and `Some("")` here.
    /// `Some("")` means that the namespace is explicitly cleared.
    /// `None` means that no namespace was provided or inherited.
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    /// Get the fully qualified name of the type in C++.
    ///
    /// This is the namespace followed by the unqualified name.
    ///
    /// Ideally we'd want this type name to always be **fully** qualified, starting with `::`.
    /// Unfortunately, this isn't always possible, as the Qt5 meta object system doesn't register
    /// types with the fully qualified path :(
    /// E.g. it will recognize `QString`, but not `::QString` from QML.
    ///
    /// This needs to be considered in many places (properties, signals, invokables, etc.)
    /// Therefore, for now we'll use the qualified, but not fully qualified version of `namespace::type`.
    /// This should work in most cases, but it's not perfect.
    pub fn cxx_qualified(&self) -> String {
        let cxx_name = self.cxx_unqualified();

        if let Some(namespace) = &self.namespace {
            format!("{namespace}::{cxx_name}")
        } else {
            cxx_name
        }
    }

    #[cfg(test)]
    pub fn mock(ident: &str) -> Self {
        Self {
            rust: format_ident!("{ident}"),
            cxx: None,
            module: Some(Path::from(format_ident!("qobject"))),
            namespace: None,
        }
    }

    #[cfg(test)]
    pub fn mock_namespaced(ident: &str, namespace: &str) -> Self {
        Self {
            rust: format_ident!("{ident}"),
            cxx: None,
            module: Some(Path::from(format_ident!("qobject"))),
            namespace: Some(namespace.to_owned()),
        }
    }
}
