// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{
    generator::rust::{fragment::GeneratedRustFragment, signals::generate_rust_signal},
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

        let mut generated = vec![GeneratedRustFragment::from_cxx_item(parse_quote! {
            #extern_block_namespace
            #unsafety extern "C++" {
                #(#items)*

                #(#types)*
            }
        })];

        // Build the signals
        for signal in &extern_cxxqt_block.signals {
            let qobject_name = type_names.lookup(&signal.qobject_ident)?;
            generated.push(generate_rust_signal(signal, qobject_name, type_names)?);
        }

        Ok(GeneratedRustFragment::flatten(generated))
    }
}
