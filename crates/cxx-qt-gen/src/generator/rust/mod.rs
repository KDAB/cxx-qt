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

use crate::generator::rust::fragment::GeneratedRustFragment;
use crate::generator::structuring;
use crate::parser::parameter::ParsedFunctionParameter;
use crate::parser::Parser;
use crate::writer;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Item, ItemMod, Result};

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

        let mut cxx_mod_contents = qenum::generate_cxx_mod_contents(&parser.cxx_qt_data.qenums);
        cxx_mod_contents.push(generate_include(parser)?);

        Ok(GeneratedRustBlocks {
            cxx_mod: parser.passthrough_module.clone(),
            cxx_mod_contents,
            namespace: parser.cxx_qt_data.namespace.clone().unwrap_or_default(),
            fragments,
        })
    }
}

/// Generate the include line for this parsed block
fn generate_include(parser: &Parser) -> Result<Item> {
    let import_path = format!(
        "{header_prefix}/{}.cxxqt.h",
        parser.cxx_file_stem,
        header_prefix = writer::get_header_prefix()
    );

    syn::parse2(quote! {
        unsafe extern "C++" {
            include!(#import_path);
        }
    })
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
        assert!(rust.cxx_mod.content.is_none());
        assert_eq!(rust.cxx_mod_contents.len(), 1);
        assert_tokens_eq(
            &rust.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    include!("cxx-qt-gen/ffi.cxxqt.h");
                }
            },
        );
        assert_eq!(rust.namespace, "");
        assert_eq!(rust.fragments.len(), 1);
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
        assert_eq!(rust.cxx_mod_contents.len(), 1);
        assert_eq!(rust.namespace, "cxx_qt");
        assert_eq!(rust.fragments.len(), 1);
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
        assert!(rust.cxx_mod.content.is_none());
        assert_eq!(rust.cxx_mod_contents.len(), 1);
        assert_tokens_eq(
            &rust.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    include!("cxx-qt-gen/my_object.cxxqt.h");
                }
            },
        );
        assert_eq!(rust.namespace, "");
        assert_eq!(rust.fragments.len(), 1);
    }
}
