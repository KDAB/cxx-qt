// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::{fragment::CppFragmentPair, property::generate_cpp_properties},
    naming::{namespace::NamespaceName, qobject::QObjectName},
};
use crate::parser::qobject::ParsedQObject;
use syn::Result;

#[derive(Default)]
pub struct GeneratedCppQObjectBlocks {
    /// Ident of the C++ QObject
    pub ident: String,
    /// Ident of the Rust object
    pub rust_ident: String,
    /// Ident of the CxxQtThread object
    pub cxx_qt_thread_ident: String,
    /// Ident of the namespace for CXX-Qt internals of the QObject
    pub namespace_internals: String,
    /// Base class of the QObject
    pub base_class: String,
    /// List of Qt Meta Object items (eg Q_PROPERTY)
    pub metaobjects: Vec<String>,
    /// List of public methods for the QObject
    pub methods: Vec<CppFragmentPair>,
    /// List of public Q_SLOTS for the QObject
    pub slots: Vec<CppFragmentPair>,
    /// List of public Q_SIGNALS for the QObject
    pub signals: Vec<String>,
}

impl GeneratedCppQObjectBlocks {
    pub fn from(qobject: &ParsedQObject) -> Result<GeneratedCppQObjectBlocks> {
        // Create the base object
        let qobject_idents = QObjectName::from(qobject);
        let namespace_idents = NamespaceName::from(qobject);
        let mut generated = GeneratedCppQObjectBlocks {
            ident: qobject_idents.cpp_class.cpp.to_string(),
            rust_ident: qobject_idents.rust_struct.cpp.to_string(),
            cxx_qt_thread_ident: qobject_idents.cxx_qt_thread_class.to_string(),
            namespace_internals: namespace_idents.internal,
            base_class: qobject
                .base_class
                .clone()
                .unwrap_or_else(|| "QObject".to_string()),
            ..Default::default()
        };

        // Generate methods for the properties, invokables, signals
        generate_cpp_properties(&mut generated, &qobject.properties, &qobject_idents)?;
        // generate_cpp_invokables(&mut generated, qobject)?;
        // generate_cpp_signals(&mut generated, qobject)?;

        Ok(generated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::Parser;
    use crate::tests::tokens_to_syn;
    use quote::quote;
    use syn::ItemMod;

    #[test]
    fn test_generated_cpp_qobject_blocks() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge]
            mod ffi {
                #[cxx_qt::qobject]
                struct MyObject;
            }
        });
        let parser = Parser::from(module).unwrap();

        let cpp =
            GeneratedCppQObjectBlocks::from(parser.cxx_qt_data.qobjects.values().next().unwrap())
                .unwrap();
        assert_eq!(cpp.ident, "MyObject");
        assert_eq!(cpp.rust_ident, "MyObjectRust");
        assert_eq!(cpp.cxx_qt_thread_ident, "MyObjectCxxQtThread");
        assert_eq!(cpp.namespace_internals, "cxx_qt_my_object");
        assert_eq!(cpp.base_class, "QObject");
    }

    #[test]
    fn test_generated_cpp_qobject_blocks_base_and_namespace() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                #[cxx_qt::qobject(base = "QStringListModel")]
                struct MyObject;
            }
        });
        let parser = Parser::from(module).unwrap();

        let cpp =
            GeneratedCppQObjectBlocks::from(parser.cxx_qt_data.qobjects.values().next().unwrap())
                .unwrap();
        assert_eq!(cpp.namespace_internals, "cxx_qt::cxx_qt_my_object");
        assert_eq!(cpp.base_class, "QStringListModel");
    }
}
