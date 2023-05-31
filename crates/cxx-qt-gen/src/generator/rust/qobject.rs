// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{namespace::NamespaceName, qobject::QObjectName},
        rust::{
            field::generate_rust_fields, fragment::RustFragmentPair, inherit,
            invokable::generate_rust_invokables, property::generate_rust_properties,
            signals::generate_rust_signals, threading,
        },
    },
    parser::qobject::ParsedQObject,
};
use quote::quote;
use syn::{Ident, ImplItem, Item, Result};

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
            namespace_internals: namespace_idents.internal.clone(),
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
            .push(syn::Item::Struct(qobject.qobject_struct.clone()));

        // Generate methods for the properties, fields, invokables, signals
        generated.blocks.append(&mut generate_rust_properties(
            &qobject.properties,
            &qobject_idents,
        )?);
        generated
            .blocks
            .append(&mut generate_rust_fields(&qobject.fields, &qobject_idents)?);
        generated.blocks.append(&mut generate_rust_invokables(
            &qobject.invokables,
            &qobject_idents,
        )?);
        generated.blocks.append(&mut generate_passthrough_impl(
            &qobject.passthrough_impl_items,
            &qobject_idents,
        )?);
        generated.blocks.append(&mut inherit::generate(
            &qobject_idents,
            &qobject.inherited_methods,
        )?);

        if let Some(signals_enum) = &qobject.signals {
            generated
                .blocks
                .append(&mut generate_rust_signals(signals_enum, &qobject_idents)?);
        }

        // If this type is a singleton then we need to add an include
        if let Some(qml_metadata) = &qobject.qml_metadata {
            if qml_metadata.singleton {
                let fragment = RustFragmentPair {
                    cxx_bridge: vec![quote! {
                        unsafe extern "C++" {
                            include!(<QtQml/QQmlEngine>);
                        }
                    }],
                    implementation: vec![],
                };
                generated
                    .blocks
                    .cxx_mod_contents
                    .append(&mut fragment.cxx_bridge_as_items()?);
            }
        }

        // If this type has threading enabled then add generation
        if qobject.threading {
            generated.blocks.append(&mut threading::generate(
                &qobject_idents,
                &namespace_idents,
            )?);
        }

        // If this type has locking enabling then implement the trait
        //
        // This could be implemented using an auto trait in the future once stable
        // https://doc.rust-lang.org/beta/unstable-book/language-features/auto-traits.html
        if qobject.locking {
            let cpp_class_name_rust = &qobject_idents.cpp_class.rust;
            generated
                .blocks
                .cxx_qt_mod_contents
                .push(syn::parse_quote! {
                    impl cxx_qt::Locking for #cpp_class_name_rust {}
                });
        }

        Ok(generated)
    }
}

/// Generate the non invokable methods for the Rust side
pub fn generate_passthrough_impl(
    impl_items: &[ImplItem],
    qobject_idents: &QObjectName,
) -> Result<GeneratedRustQObjectBlocks> {
    let mut blocks = GeneratedRustQObjectBlocks::default();
    blocks.cxx_qt_mod_contents.extend_from_slice(
        &impl_items
            .iter()
            // Build non invokables on the C++ struct
            .map(|impl_item| {
                let cpp_class_name_rust = &qobject_idents.cpp_class.rust;
                syn::parse2(quote! {
                    impl #cpp_class_name_rust {
                        #impl_item
                    }
                })
            })
            .collect::<Result<Vec<Item>>>()?,
    );

    Ok(blocks)
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
    let rust_struct_name_rust_str = rust_struct_name_rust.to_string();
    let fragment = RustFragmentPair {
        cxx_bridge: vec![
            quote! {
                unsafe extern "C++" {
                    #[doc = "The C++ type for the QObject "]
                    #[doc = #rust_struct_name_rust_str]
                    #[doc = "\n"]
                    #[doc = "Use this type when referring to the QObject as a pointer"]
                    #[doc = "\n"]
                    #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
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
    use crate::tests::assert_tokens_eq;
    use syn::{parse_quote, ItemMod};

    #[test]
    fn test_generated_rust_qobject_blocks() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                #[cxx_qt::qobject]
                pub struct MyObject;
            }
        };
        let parser = Parser::from(module).unwrap();

        let rust = GeneratedRustQObject::from(parser.cxx_qt_data.qobjects.values().next().unwrap())
            .unwrap();
        assert_eq!(rust.cpp_struct_ident, "MyObjectQt");
        assert_eq!(rust.namespace_internals, "cxx_qt_my_object");
        assert_eq!(rust.rust_struct_ident, "MyObject");
    }

    #[test]
    fn test_generated_rust_qobject_blocks_namespace() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                #[cxx_qt::qobject]
                pub struct MyObject;
            }
        };
        let parser = Parser::from(module).unwrap();

        let rust = GeneratedRustQObject::from(parser.cxx_qt_data.qobjects.values().next().unwrap())
            .unwrap();
        assert_eq!(rust.cpp_struct_ident, "MyObjectQt");
        assert_eq!(rust.namespace_internals, "cxx_qt::cxx_qt_my_object");
        assert_eq!(rust.rust_struct_ident, "MyObject");
    }

    #[test]
    fn test_generated_rust_qobject_blocks_singleton() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                #[cxx_qt::qobject(qml_uri = "com.kdab", qml_version = "1.0", qml_singleton)]
                pub struct MyObject;
            }
        };
        let parser = Parser::from(module).unwrap();

        let rust = GeneratedRustQObject::from(parser.cxx_qt_data.qobjects.values().next().unwrap())
            .unwrap();
        assert_eq!(rust.blocks.cxx_mod_contents.len(), 3);
        assert_tokens_eq(
            &rust.blocks.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[doc = "The C++ type for the QObject "]
                    #[doc = "MyObject"]
                    #[doc = "\n"]
                    #[doc = "Use this type when referring to the QObject as a pointer"]
                    #[doc = "\n"]
                    #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
                    #[cxx_name = "MyObject"]
                    type MyObjectQt;
                }
            },
        );
        assert_tokens_eq(
            &rust.blocks.cxx_mod_contents[1],
            quote! {
                extern "Rust" {
                    #[cxx_name = "MyObjectRust"]
                    type MyObject;
                }
            },
        );
        assert_tokens_eq(
            &rust.blocks.cxx_mod_contents[2],
            quote! {
                unsafe extern "C++" {
                    include!(<QtQml/QQmlEngine>);
                }
            },
        );
    }
}
