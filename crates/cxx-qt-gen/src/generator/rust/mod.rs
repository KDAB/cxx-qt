// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod constructor;
pub mod cxxqttype;
pub mod externcxxqt;
pub mod fragment;
pub mod inherit;
pub mod method;
pub mod property;
pub mod qenum;
pub mod qobject;
pub mod signals;
pub mod threading;

use crate::generator::rust::{externcxxqt::GeneratedExternCxxQt, qobject::GeneratedRustQObject};
use crate::parser::Parser;
use quote::quote;
use syn::{Item, ItemMod, Result};

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
    /// Generated extern "C++Qt" blocks
    pub extern_cxx_qt: Vec<GeneratedExternCxxQt>,
}

impl GeneratedRustBlocks {
    pub fn from(parser: &Parser) -> Result<GeneratedRustBlocks> {
        let mut cxx_mod_contents = qenum::generate_cxx_mod_contents(&parser.cxx_qt_data.qenums);
        cxx_mod_contents.push(generate_include(parser)?);

        Ok(GeneratedRustBlocks {
            cxx_mod: parser.passthrough_module.clone(),
            cxx_mod_contents,
            cxx_qt_mod_contents: vec![],
            namespace: parser.cxx_qt_data.namespace.clone(),
            qobjects: parser
                .cxx_qt_data
                .qobjects
                .values()
                .map(|qobject| {
                    GeneratedRustQObject::from(
                        qobject,
                        &parser.cxx_qt_data.cxx_mappings.qualified,
                        &parser.passthrough_module.ident,
                    )
                })
                .collect::<Result<Vec<GeneratedRustQObject>>>()?,
            extern_cxx_qt: parser
                .cxx_qt_data
                .extern_cxxqt_blocks
                .iter()
                .map(|extern_cxx_block| {
                    GeneratedExternCxxQt::from(
                        extern_cxx_block,
                        &parser.cxx_qt_data.cxx_mappings,
                        &parser.passthrough_module.ident,
                    )
                })
                .collect::<Result<Vec<GeneratedExternCxxQt>>>()?,
        })
    }
}

/// Generate the include line for this parsed block
fn generate_include(parser: &Parser) -> Result<Item> {
    let import_path = format!("cxx-qt-gen/{}.cxxqt.h", parser.cxx_file_stem);

    syn::parse2(quote! {
        unsafe extern "C++" {
            include!(#import_path);
        }
    })
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    use crate::tests::assert_tokens_eq;

    #[test]
    fn test_generated_rust_blocks() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();

        let rust = GeneratedRustBlocks::from(&parser).unwrap();
        assert_eq!(rust.cxx_mod.content.unwrap().1.len(), 0);
        assert_eq!(rust.cxx_mod_contents.len(), 1);
        assert_tokens_eq(
            &rust.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    include!("cxx-qt-gen/ffi.cxxqt.h");
                }
            },
        );
        assert_eq!(rust.cxx_qt_mod_contents.len(), 0);
        assert_eq!(rust.namespace, "");
        assert_eq!(rust.qobjects.len(), 1);
    }

    #[test]
    fn test_generated_rust_blocks_namespace() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();

        let rust = GeneratedRustBlocks::from(&parser).unwrap();
        assert_eq!(rust.cxx_mod.content.unwrap().1.len(), 0);
        assert_eq!(rust.cxx_mod_contents.len(), 1);
        assert_eq!(rust.cxx_qt_mod_contents.len(), 0);
        assert_eq!(rust.namespace, "cxx_qt");
        assert_eq!(rust.qobjects.len(), 1);
    }

    #[test]
    fn test_generated_rust_blocks_cxx_file_stem() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(cxx_file_stem = "my_object")]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();

        let rust = GeneratedRustBlocks::from(&parser).unwrap();
        assert_eq!(rust.cxx_mod.content.unwrap().1.len(), 0);
        assert_eq!(rust.cxx_mod_contents.len(), 1);
        assert_tokens_eq(
            &rust.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    include!("cxx-qt-gen/my_object.cxxqt.h");
                }
            },
        );
        assert_eq!(rust.cxx_qt_mod_contents.len(), 0);
        assert_eq!(rust.namespace, "");
        assert_eq!(rust.qobjects.len(), 1);
    }
}
