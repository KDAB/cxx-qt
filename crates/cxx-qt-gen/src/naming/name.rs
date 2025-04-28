// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::CaseConversion;
use crate::syntax::{attribute::attribute_get_path, expr::expr_to_string};
use convert_case::Casing;
use quote::format_ident;
use syn::{spanned::Spanned, Attribute, Error, Ident, Path, Result};

/// This struct contains all names a certain syntax element may have
///
/// This includes the rust_name, cxx_name, as well as qualifications like
/// the rust module and C++ namespace.
///
/// Naming in CXX can be rather complex.
/// The following Rules apply:
/// - If only a cxx_name **or** a rust_name is given, the identifier of the type/function will be
///   used for part that wasn't specified explicitly.
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

    /// Parse a name from an identifier and a list of attributes.
    ///
    /// This deciphers the rust_name, cxx_name and namespace attributes, including
    /// inheriting the namespace from the parent.
    pub fn from_ident_and_attrs(
        ident: &Ident,
        attrs: &[Attribute],
        parent_namespace: Option<&str>,
        module: Option<&Ident>,
        auto_case: CaseConversion,
    ) -> Result<Self> {
        // Find if there is a namespace (for C++ generation)
        let mut namespace = if let Some(attr) = attribute_get_path(attrs, &["namespace"]) {
            Some(expr_to_string(&attr.meta.require_name_value()?.value)?)
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
        let cxx_name = attribute_get_path(attrs, &["cxx_name"])
            .map(|attr| -> Result<_> { expr_to_string(&attr.meta.require_name_value()?.value) })
            .transpose()?;

        // Find if there is a rust_name mapping
        let rust_name = attribute_get_path(attrs, &["rust_name"])
            .map(|attr| -> Result<_> {
                Ok(format_ident!(
                    "{}",
                    expr_to_string(&attr.meta.require_name_value()?.value)?,
                    span = attr.span()
                ))
            })
            .transpose()?;

        Ok(Self {
            rust: ident.clone(),
            cxx: None,
            namespace,
            module: module.cloned().map(Path::from),
        }
        .with_options(cxx_name, rust_name, auto_case))
    }

    /// Applies naming options to an existing name, applying logic about what should cause renaming
    pub fn with_options(
        self,
        mut cxx_name: Option<String>,
        mut rust_name: Option<Ident>,
        auto_case: CaseConversion,
    ) -> Self {
        // Determine any automatic casing when there is no cxx_name or rust_name
        if cxx_name.is_none() && rust_name.is_none() {
            if let Some(case) = auto_case.cxx {
                cxx_name = Some(self.rust_unqualified().to_string().to_case(case));
            }

            if let Some(case) = auto_case.rust {
                rust_name = Some(format_ident!(
                    "{}",
                    self.rust_unqualified().to_string().to_case(case)
                ));
            }
        }

        // Use the rust name if there is one or fallback to the original ident
        let rust = rust_name.unwrap_or_else(|| self.rust.clone());

        // Use the cxx name if there is one or fallback to the original ident
        // But only if it is different to the resultant rust ident
        let cxx = cxx_name.or_else(|| {
            if rust != self.rust {
                Some(self.rust.to_string())
            } else {
                None
            }
        });

        Self { rust, cxx, ..self }
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

    /// Get the module of the type in Rust
    /// This is usually the name of the bridge module.
    pub fn module(&self) -> Option<&Path> {
        self.module.as_ref()
    }

    /// Destructure the Name into the parts needed to generate a CXX bridge
    /// 1. The ident of the function
    /// 2. Any attributes like cxx_name and namespace
    /// 3. The rust_qualified path to access the function (if not needed use _ during destructuring)
    pub fn into_cxx_parts(self) -> (Ident, Vec<Attribute>, Path) {
        let rust_qualified = self.rust_qualified();
        let cxx_name: Option<Attribute> = self.cxx.map(|cxx| {
            syn::parse_quote! { #[cxx_name = #cxx] }
        });
        let namespace = self
            .namespace
            .map(|namespace| syn::parse_quote! { #[namespace=#namespace] });

        (
            self.rust,
            cxx_name.into_iter().chain(namespace).collect(),
            rust_qualified,
        )
    }

    /// Returns the Ident of this names module if it exists, otherwise errors
    ///
    /// TODO: This should be deprecated! It is mostly used to access other members in the same
    /// module as the QObject.
    /// Preferrable, these other members should have full Name instances and use rust_qualified()
    pub fn require_module(&self) -> Result<&Path> {
        if let Some(ident) = self.module() {
            Ok(ident)
        } else {
            Err(Error::new_spanned(
                self.rust_unqualified(),
                format!("No Module name for {}!", self.rust_unqualified()),
            ))
        }
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
            module: None,
            namespace: None,
        }
        .with_module(Path::from(format_ident!("qobject")))
    }

    #[cfg(test)]
    pub fn mock_namespaced(ident: &str, namespace: &str) -> Self {
        Self {
            rust: format_ident!("{ident}"),
            cxx: None,
            module: None,
            namespace: None,
        }
        .with_namespace(namespace.into())
        .with_module(Path::from(format_ident!("qobject")))
    }

    #[cfg(test)]
    /// Helper for creating cxx_named Names, usually for camelcase cxx names
    pub fn mock_name_with_cxx(name: &str, cxx: &str) -> Name {
        Name::new(format_ident!("{name}")).with_cxx_name(cxx.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_namespace() {
        let mut name = Name::mock_namespaced("my_object", "my_namespace");
        let old_namespace = name.set_namespace(None);

        assert_eq!(old_namespace, Some("my_namespace".into()));
        assert!(name.namespace.is_none())
    }

    #[test]
    fn test_require_without_module() {
        let mut name = Name::mock("my_object");
        name.module = None;
        assert!(name.module().is_none());
        assert!(name.require_module().is_err());
    }
}
