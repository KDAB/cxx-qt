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

use crate::generator::{rust::fragment::GeneratedRustFragment, structuring};
use crate::parser::cxxqtdata::ParsedCxxQtData;
use crate::parser::{parameter::ParsedFunctionParameter, Parser};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_quote, Item, ItemMod, Result};

/// Representation of the generated Rust code for a QObject
pub struct GeneratedRustBlocks {
    /// Module for the CXX bridge with passthrough items
    pub cxx_mod: ItemMod,
    /// Any global extra items for the CXX bridge
    pub cxx_mod_contents: Vec<Item>,
    /// Ident of the namespace of the QObject
    pub namespace: String,
    /// Rust fragments
    pub fragments: Vec<GeneratedRustFragment>,
}

impl GeneratedRustBlocks {
    /// Create a [GeneratedRustBlocks] from the given [Parser] object
    pub fn from(parser: &Parser) -> Result<GeneratedRustBlocks> {
        let structures = structuring::Structures::new(&parser.cxx_qt_data)?;

        let mut fragments = vec![];
        fragments.extend(
            structures
                .qobjects
                .iter()
                .map(|qobject| GeneratedRustFragment::from_qobject(qobject, &parser.type_names))
                .collect::<Result<Vec<GeneratedRustFragment>>>()?,
        );
        fragments.extend(
            parser
                .cxx_qt_data
                .extern_cxxqt_blocks
                .iter()
                .map(|extern_cxx_block| {
                    GeneratedRustFragment::from_extern_cxx_qt(extern_cxx_block, &parser.type_names)
                })
                .collect::<Result<Vec<GeneratedRustFragment>>>()?,
        );

        let namespace = parser.cxx_qt_data.namespace.clone().unwrap_or_default();
        let passthrough_mod = &parser.passthrough_module;

        if let Some(qobject_import) = add_qobject_import(&parser.cxx_qt_data) {
            fragments.push(qobject_import);
        }

        let vis = &passthrough_mod.vis;
        let ident = &passthrough_mod.module_ident;
        let docs = &passthrough_mod.docs;
        let module = passthrough_mod.items.clone().map_or(
            // If no items are present, curly braces aren't needed
            quote! {
                #vis mod #ident;
            },
            |items| {
                quote! {
                    #vis mod #ident {
                        #(#items)*
                    }
                }
            },
        );
        let cxx_mod = parse_quote! {
            #[cxx::bridge(namespace = #namespace)]
            // We need to allow for unused unsafe otherwise we get build failures
            // due to changes in CXX 1.0.130
            // https://github.com/dtolnay/cxx/commit/46fedc68464f80587057c436b4f6b6debeb9f714
            #[allow(unused_unsafe)]
            #(#docs)*
            #module
        };

        Ok(GeneratedRustBlocks {
            cxx_mod,
            cxx_mod_contents: qenum::generate_cxx_mod_contents(&parser.cxx_qt_data.qenums),
            namespace,
            fragments,
        })
    }
}

// Generate a type declaration for `QObject` if necessary
fn add_qobject_import(cxx_qt_data: &ParsedCxxQtData) -> Option<GeneratedRustFragment> {
    let includes = cxx_qt_data
        .qobjects
        .iter()
        .any(|obj| obj.has_qobject_macro && obj.base_class.is_none());
    if includes
        || cxx_qt_data
            .extern_cxxqt_blocks
            .iter()
            .filter(|block| !block.qobjects.is_empty())
            .count()
            > 0
    {
        Some(GeneratedRustFragment::qobject_import())
    } else {
        None
    }
}

/// Return the [TokenStream] of the parsed parameters for use in generation
pub fn get_params_tokens(
    mutable: bool,
    parameters: &[ParsedFunctionParameter],
    class_name: &Ident,
) -> TokenStream {
    let struct_sig = if mutable {
        quote! { Pin<&mut #class_name> }
    } else {
        quote! { &#class_name }
    };
    if parameters.is_empty() {
        quote! { self: #struct_sig }
    } else {
        let parameters = parameters
            .iter()
            .map(|parameter| {
                let ident = &parameter.ident;
                let ty = &parameter.ty;
                quote! { #ident: #ty }
            })
            .collect::<Vec<TokenStream>>();
        quote! { self: #struct_sig, #(#parameters),* }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

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
        assert!(rust.cxx_mod.content.is_none());
        assert_eq!(rust.cxx_mod_contents.len(), 0);
        assert_eq!(rust.namespace, "");
        assert_eq!(rust.fragments.len(), 2);
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
        assert!(rust.cxx_mod.content.is_none());
        assert_eq!(rust.cxx_mod_contents.len(), 0);
        assert_eq!(rust.namespace, "cxx_qt");
        assert_eq!(rust.fragments.len(), 2);
    }

    #[test]
    fn test_generated_rust_blocks_foreign_qobject() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "C++Qt" {
                    #[qobject]
                    type MyObject;
                }
            }
        };
        let parser = Parser::from(module).unwrap();

        let rust = GeneratedRustBlocks::from(&parser).unwrap();
        assert!(rust.cxx_mod.content.is_none());
        assert_eq!(rust.cxx_mod_contents.len(), 0);
        assert_eq!(rust.namespace, "");
        assert_eq!(rust.fragments.len(), 2);
    }
}
