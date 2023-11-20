// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::parser::qobject::ParsedQObject;
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::Ident;

/// Names for the namespace of this QObject
pub struct NamespaceName {
    pub namespace: String,
    pub internal: String,
}

impl From<&ParsedQObject> for NamespaceName {
    fn from(qobject: &ParsedQObject) -> Self {
        NamespaceName::from_pair_str(&qobject.namespace, &qobject.qobject_ty.ident_left)
    }
}

impl NamespaceName {
    /// Build the namespace names from a given module and qobject ident
    pub fn from_pair_str(namespace: &str, ident: &Ident) -> Self {
        Self {
            namespace: namespace.to_string(),
            internal: namespace_internal_from_pair(namespace, ident),
        }
    }
}

/// For a given namespace and ident combine them into a single string
pub fn namespace_combine_ident(namespace: &str, ident: &Ident) -> String {
    if namespace.is_empty() {
        return ident.to_string();
    }

    format!("{namespace}::{ident}")
}

/// For a given base namespace and QObject ident generate the internal namespace
///
/// The base namespace could be from the module bridge or from the QObject
fn namespace_internal_from_pair(base: &str, ident: &Ident) -> String {
    namespace_combine_ident(
        base,
        &format_ident!("cxx_qt_{}", ident.to_string().to_case(Case::Snake)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::qobject::tests::create_parsed_qobject;

    #[test]
    fn test_namespace_pair() {
        let mut qobject = create_parsed_qobject();
        qobject.namespace = "cxx_qt".to_owned();

        let names = NamespaceName::from(&qobject);
        assert_eq!(names.internal, "cxx_qt::cxx_qt_my_object");
        assert_eq!(names.namespace, "cxx_qt");
    }

    #[test]
    fn test_namespace_pair_empty_base() {
        let mut qobject = create_parsed_qobject();
        qobject.namespace = "".to_owned();

        let names = NamespaceName::from(&qobject);
        assert_eq!(names.internal, "cxx_qt_my_object");
        assert_eq!(names.namespace, "");
    }

    #[test]
    fn test_namespace_combine_ident() {
        let base = "base::namespace";
        let ident = format_ident!("Ident");
        let string = namespace_combine_ident(base, &ident);
        assert_eq!(string, "base::namespace::Ident");
    }
}
