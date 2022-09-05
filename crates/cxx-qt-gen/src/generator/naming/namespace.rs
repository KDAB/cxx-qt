// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::parser::qobject::ParsedQObject;
use convert_case::{Case, Casing};
use syn::Ident;

/// Names for the namespace of this QObject
pub struct NamespaceName {
    pub namespace: String,
    pub internal: String,
}

impl From<&ParsedQObject> for NamespaceName {
    fn from(qobject: &ParsedQObject) -> Self {
        NamespaceName::from_pair_str(
            &qobject.namespace,
            &qobject.qobject_struct.as_ref().unwrap().ident,
        )
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

    use crate::tests::tokens_to_syn;
    use quote::quote;
    use syn::ItemStruct;

    #[test]
    fn test_namespace_pair() {
        let qobject_struct: ItemStruct = tokens_to_syn(quote! {
            struct MyObject;
        });
        let module = ParsedQObject {
            namespace: "cxx_qt".to_owned(),
            qobject_struct: Some(qobject_struct),
            ..Default::default()
        };
        let names = NamespaceName::from(&module);
        assert_eq!(names.internal, "cxx_qt::cxx_qt_my_object");
        assert_eq!(names.namespace, "cxx_qt");
    }

    #[test]
    fn test_namespace_pair_empty_base() {
        let qobject_struct: ItemStruct = tokens_to_syn(quote! {
            struct MyObject;
        });
        let module = ParsedQObject {
            namespace: "".to_owned(),
            qobject_struct: Some(qobject_struct),
            ..Default::default()
        };
        let names = NamespaceName::from(&module);
        assert_eq!(names.internal, "cxx_qt_my_object");
        assert_eq!(names.namespace, "");
    }
}
