// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#[cfg(test)]
use crate::parser::cxxqtdata::ParsedCxxQtData;
use convert_case::{Case, Casing};
use syn::Ident;

/// Names for the namespace of this QObject
pub struct NamespaceName {
    pub namespace: String,
    pub internal: String,
}

impl NamespaceName {
    /// Build the namespace names from a given module and qobject ident
    #[cfg(test)]
    pub fn from_pair(module: &ParsedCxxQtData, ident: &Ident) -> Self {
        Self {
            namespace: module.namespace.clone(),
            internal: namespace_internal_from_pair(&module.namespace, ident),
        }
    }

    /// Build the namespace names from a given module and qobject ident
    pub fn from_pair_str(namespace: &str, ident: &Ident) -> Self {
        Self {
            namespace: namespace.to_string(),
            internal: namespace_internal_from_pair(namespace, ident),
        }
    }
}

/// For a given base namespace and QObject ident generate the internal namespace
///
/// The base namespace could be from the module bridge or from the QObject
fn namespace_internal_from_pair(base: &str, ident: &Ident) -> String {
    let mut namespace_internals = vec![];
    if !base.is_empty() {
        namespace_internals.push(base.to_owned());
    }
    namespace_internals.push(format!("cxx_qt_{}", ident.to_string().to_case(Case::Snake)));
    namespace_internals.join("::")
}

#[cfg(test)]
mod tests {
    use super::*;

    use quote::format_ident;

    #[test]
    fn test_namespace_pair() {
        let module = ParsedCxxQtData {
            namespace: "cxx_qt".to_owned(),
            ..Default::default()
        };
        let names = NamespaceName::from_pair(&module, &format_ident!("MyObject"));
        assert_eq!(names.internal, "cxx_qt::cxx_qt_my_object");
        assert_eq!(names.namespace, "cxx_qt");
    }

    #[test]
    fn test_namespace_pair_empty_base() {
        let module = ParsedCxxQtData::default();
        let names = NamespaceName::from_pair(&module, &format_ident!("MyObject"));
        assert_eq!(names.internal, "cxx_qt_my_object");
        assert_eq!(names.namespace, "");
    }
}
