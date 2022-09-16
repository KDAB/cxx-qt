// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{namespace::NamespaceName, qobject::QObjectName},
        rust::{fragment::RustFragmentPair, property::generate_rust_properties},
    },
    parser::qobject::ParsedQObject,
};
use quote::quote;
use syn::{Ident, Item, Result};

#[derive(Default)]
pub struct GeneratedRustQObjectBlocks {
    /// Module for the CXX bridge
    pub cxx_mod_contents: Vec<Item>,
    /// Items for the CXX-Qt module
    pub cxx_qt_mod_contents: Vec<Item>,
}

impl GeneratedRustQObjectBlocks {
    pub fn append(&mut self, other: &mut Self) {
        self.cxx_mod_contents.append(&mut other.cxx_mod_contents);
        self.cxx_qt_mod_contents
            .append(&mut other.cxx_qt_mod_contents);
    }
}

pub struct GeneratedRustQObject {
    /// Ident of the Rust name for the C++ object
    pub cpp_struct_ident: Ident,
    /// Ident of the CxxQtThread object
    pub cxx_qt_thread_ident: Ident,
    /// Ident of the namespace for CXX-Qt internals of the QObject
    pub namespace_internals: String,
    /// Ident of the Rust name for the Rust object
    pub rust_struct_ident: Ident,
    /// The blocks for this QObject
    pub blocks: GeneratedRustQObjectBlocks,
}

impl GeneratedRustQObject {
    pub fn from(qobject: &ParsedQObject) -> Result<GeneratedRustQObject> {
        // Create the base object
        let qobject_idents = QObjectName::from(qobject);
        let namespace_idents = NamespaceName::from(qobject);
        let mut generated = GeneratedRustQObject {
            cpp_struct_ident: qobject_idents.cpp_class.rust.clone(),
            cxx_qt_thread_ident: qobject_idents.cxx_qt_thread_class.clone(),
            namespace_internals: namespace_idents.internal,
            rust_struct_ident: qobject_idents.rust_struct.rust.clone(),
            blocks: GeneratedRustQObjectBlocks {
                cxx_mod_contents: vec![],
                cxx_qt_mod_contents: qobject.others.clone(),
            },
        };

        generated
            .blocks
            .append(&mut generate_qobject_definitions(&qobject_idents)?);

        // Add our QObject struct to the implementation blocks
        generated
            .blocks
            .cxx_qt_mod_contents
            .push(syn::Item::Struct(qobject.qobject_struct.clone().unwrap()));

        // Generate methods for the properties, invokables, signals
        generated.blocks.append(&mut generate_rust_properties(
            &qobject.properties,
            &qobject_idents,
        )?);

        Ok(generated)
    }
}

/// Generate the C++ and Rust CXX definitions for the QObject
fn generate_qobject_definitions(
    qobject_idents: &QObjectName,
) -> Result<GeneratedRustQObjectBlocks> {
    let mut generated = GeneratedRustQObjectBlocks::default();
    let cpp_class_name_cpp = &qobject_idents.cpp_class.cpp.to_string();
    let cpp_class_name_rust = &qobject_idents.cpp_class.rust;
    let rust_struct_name_cpp = &qobject_idents.rust_struct.cpp.to_string();
    let rust_struct_name_rust = &qobject_idents.rust_struct.rust;
    let fragment = RustFragmentPair {
        cxx_bridge: vec![
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = #cpp_class_name_cpp]
                    type #cpp_class_name_rust;
                }
            },
            quote! {
                extern "Rust" {
                    #[cxx_name = #rust_struct_name_cpp]
                    type #rust_struct_name_rust;
                }
            },
        ],
        implementation: vec![],
    };

    generated
        .cxx_mod_contents
        .append(&mut fragment.cxx_bridge_as_items()?);

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::Parser;
    use crate::tests::tokens_to_syn;
    use syn::ItemMod;

    #[test]
    fn test_generated_rust_qobject_blocks() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge]
            mod ffi {
                #[cxx_qt::qobject]
                struct MyObject;
            }
        });
        let parser = Parser::from(module).unwrap();

        let rust = GeneratedRustQObject::from(parser.cxx_qt_data.qobjects.values().next().unwrap())
            .unwrap();
        assert_eq!(rust.cpp_struct_ident, "MyObjectQt");
        assert_eq!(rust.cxx_qt_thread_ident, "MyObjectCxxQtThread");
        assert_eq!(rust.namespace_internals, "cxx_qt_my_object");
        assert_eq!(rust.rust_struct_ident, "MyObject");
    }

    #[test]
    fn test_generated_rust_qobject_blocks_namespace() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                #[cxx_qt::qobject]
                struct MyObject;
            }
        });
        let parser = Parser::from(module).unwrap();

        let rust = GeneratedRustQObject::from(parser.cxx_qt_data.qobjects.values().next().unwrap())
            .unwrap();
        assert_eq!(rust.cpp_struct_ident, "MyObjectQt");
        assert_eq!(rust.cxx_qt_thread_ident, "MyObjectCxxQtThread");
        assert_eq!(rust.namespace_internals, "cxx_qt::cxx_qt_my_object");
        assert_eq!(rust.rust_struct_ident, "MyObject");
    }
}
