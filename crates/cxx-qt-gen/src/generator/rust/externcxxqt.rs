// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{
    generator::rust::{
        fragment::GeneratedRustFragment,
        signals::generate_rust_signal,
    },
    naming::TypeNames,
    parser::externcxxqt::ParsedExternCxxQt,
    syntax::path::path_compare_str,
};
use quote::quote;
use syn::{parse_quote, Attribute, Result};

impl GeneratedRustFragment {
    pub fn from_extern_cxx_qt(
        extern_cxxqt_block: &ParsedExternCxxQt,
        type_names: &TypeNames,
    ) -> Result<Self> {
        let mut generated = Self::default();

        let extern_block_namespace = if let Some(namespace) = &extern_cxxqt_block.namespace {
            quote! { #[namespace = #namespace ] }
        } else {
            quote! {}
        };

        // Add the pass through blocks
        let unsafety = &extern_cxxqt_block.unsafety;
        let items = &extern_cxxqt_block.passthrough_items;
        let types = &extern_cxxqt_block
            .qobjects
            .iter()
            .map(|ty| {
                let namespace = if let Some(namespace) = &ty.name.namespace() {
                    quote! { #[namespace = #namespace ] }
                } else {
                    quote! {}
                };
                let cpp_name = &ty.name.cxx_unqualified();
                let rust_name = &ty.name.rust_unqualified();
                let vis = &ty.declaration.vis;
                let ident = &ty.name.rust_unqualified();
                let cxx_name = if &rust_name.to_string() == cpp_name {
                    quote! {}
                } else {
                    let cxx_name = cpp_name.to_string();
                    quote! {
                        #[cxx_name = #cxx_name]
                    }
                };
                let docs: Vec<&Attribute> = ty
                    .declaration
                    .attrs
                    .iter()
                    .filter(|attr| path_compare_str(attr.meta.path(), &["doc"]))
                    .collect();
                quote! {
                    #namespace
                    #cxx_name
                    #(#docs)*
                    #vis type #ident;
                }
            })
            .collect::<Vec<_>>();

        generated.append(&mut GeneratedRustFragment {
            cxx_mod_contents: vec![
                parse_quote! {
                #extern_block_namespace
                #unsafety extern "C++" {
                    #(#items)*

                    #(#types)*
                }
            }
            ],
            cxx_qt_mod_contents: vec![],
        });

        // Build the signals
        for signal in &extern_cxxqt_block.signals {
            let qobject_name = type_names.lookup(&signal.qobject_ident)?;

            generated.append(&mut generate_rust_signal(signal, qobject_name, type_names)?);
        }

        Ok(generated)
    }
}
