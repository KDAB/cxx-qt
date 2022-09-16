// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::{
        fragment::CppFragmentPair, invokable::generate_cpp_invokables,
        property::generate_cpp_properties, signal::generate_cpp_signals,
    },
    naming::{namespace::NamespaceName, qobject::QObjectName},
};
use crate::parser::qobject::ParsedQObject;
use syn::Result;

#[derive(Default)]
pub struct GeneratedCppQObjectBlocks {
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
    pub fn append(&mut self, other: &mut Self) {
        self.metaobjects.append(&mut other.metaobjects);
        self.methods.append(&mut other.methods);
        self.slots.append(&mut other.slots);
        self.signals.append(&mut other.signals);
    }
}

#[derive(Default)]
pub struct GeneratedCppQObject {
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
    /// The blocks of the QObject
    pub blocks: GeneratedCppQObjectBlocks,
}

impl GeneratedCppQObject {
    pub fn from(qobject: &ParsedQObject) -> Result<GeneratedCppQObject> {
        // Create the base object
        let qobject_idents = QObjectName::from(qobject);
        let namespace_idents = NamespaceName::from(qobject);
        let mut generated = GeneratedCppQObject {
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
        generated.blocks.append(&mut generate_cpp_properties(
            &qobject.properties,
            &qobject_idents,
        )?);
        generated.blocks.append(&mut generate_cpp_invokables(
            &qobject.invokables,
            &qobject_idents,
        )?);
        if let Some(signals_enum) = &qobject.signals {
            generated.blocks.append(&mut generate_cpp_signals(
                &signals_enum.signals,
                &qobject_idents,
            )?);
        }

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

        let cpp = GeneratedCppQObject::from(parser.cxx_qt_data.qobjects.values().next().unwrap())
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

        let cpp = GeneratedCppQObject::from(parser.cxx_qt_data.qobjects.values().next().unwrap())
            .unwrap();
        assert_eq!(cpp.namespace_internals, "cxx_qt::cxx_qt_my_object");
        assert_eq!(cpp.base_class, "QStringListModel");
    }
}
