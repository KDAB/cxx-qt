// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod field;
pub mod fragment;
pub mod invokable;
pub mod property;
pub mod qobject;
pub mod signals;

use crate::generator::{naming::module::cxx_stem_from_ident, rust::qobject::GeneratedRustQObject};
use crate::parser::Parser;
use quote::quote;
use syn::{spanned::Spanned, Error, Item, ItemMod, Result};

/// Representation of the generated Rust code for a QObject
pub struct GeneratedRustBlocks {
    /// Module for the CXX bridge with passthrough items
    pub cxx_mod: ItemMod,
    /// Any global extra items for the CXX bridge
    pub cxx_mod_contents: Vec<Item>,
    /// Any global passthrough items for the implementation
    pub cxx_qt_mod_contents: Vec<Item>,
    /// Ident of the namespace of the QObject
    pub namespace: String,
    /// Generated QObject blocks
    pub qobjects: Vec<GeneratedRustQObject>,
}

impl GeneratedRustBlocks {
    pub fn from(parser: &Parser) -> Result<GeneratedRustBlocks> {
        Ok(GeneratedRustBlocks {
            cxx_mod: parser.passthrough_module.clone(),
            cxx_mod_contents: vec![generate_include(parser)?],
            cxx_qt_mod_contents: parser.cxx_qt_data.uses.clone(),
            namespace: parser.cxx_qt_data.namespace.clone(),
            qobjects: parser
                .cxx_qt_data
                .qobjects
                .values()
                .map(GeneratedRustQObject::from)
                .collect::<Result<Vec<GeneratedRustQObject>>>()?,
        })
    }
}

/// Generate the include line for this parsed block
fn generate_include(parser: &Parser) -> Result<Item> {
    // TODO: for now the cxx stem comes from the first QObject name
    // but later this may come from the module or file so would then have
    // a field on the cxx_qt_data
    if parser.cxx_qt_data.qobjects.len() != 1 {
        return Err(Error::new(
            parser.passthrough_module.span(),
            "Only one QObject is currently supported in the ItemMod.",
        ));
    }
    let (qt_ident, _) = parser.cxx_qt_data.qobjects.iter().take(1).next().unwrap();
    let import_path = format!(
        "cxx-qt-gen/include/{}.cxxqt.h",
        cxx_stem_from_ident(qt_ident)
    );

    syn::parse2(quote! {
        unsafe extern "C++" {
            include!(#import_path);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::{assert_tokens_eq, tokens_to_syn};

    #[test]
    fn test_generated_rust_blocks() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge]
            mod ffi {
                #[cxx_qt::qobject]
                struct MyObject;
            }
        });
        let parser = Parser::from(module).unwrap();

        let rust = GeneratedRustBlocks::from(&parser).unwrap();
        assert_eq!(rust.cxx_mod.content.unwrap().1.len(), 0);
        assert_eq!(rust.cxx_mod_contents.len(), 1);
        assert_tokens_eq(
            &rust.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    include!("cxx-qt-gen/include/my_object.cxxqt.h");
                }
            },
        );
        assert_eq!(rust.cxx_qt_mod_contents.len(), 0);
        assert_eq!(rust.namespace, "");
        assert_eq!(rust.qobjects.len(), 1);
    }

    #[test]
    fn test_generated_rust_blocks_namespace() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                #[cxx_qt::qobject]
                struct MyObject;
            }
        });
        let parser = Parser::from(module).unwrap();

        let rust = GeneratedRustBlocks::from(&parser).unwrap();
        assert_eq!(rust.cxx_mod.content.unwrap().1.len(), 0);
        assert_eq!(rust.cxx_mod_contents.len(), 1);
        assert_eq!(rust.cxx_qt_mod_contents.len(), 0);
        assert_eq!(rust.namespace, "cxx_qt");
        assert_eq!(rust.qobjects.len(), 1);
    }

    #[test]
    fn test_generated_rust_blocks_uses() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                use std::collections::HashMap;

                #[cxx_qt::qobject]
                struct MyObject;
            }
        });
        let parser = Parser::from(module).unwrap();

        let rust = GeneratedRustBlocks::from(&parser).unwrap();
        assert_eq!(rust.cxx_mod.content.unwrap().1.len(), 0);
        assert_eq!(rust.cxx_mod_contents.len(), 1);
        assert_eq!(rust.cxx_qt_mod_contents.len(), 1);
        assert_eq!(rust.namespace, "cxx_qt");
        assert_eq!(rust.qobjects.len(), 1);
    }
}
