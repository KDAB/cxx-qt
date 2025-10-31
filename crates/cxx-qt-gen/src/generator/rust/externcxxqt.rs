// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::generator::naming::qobject::QObjectNames;
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
        let mut generated = extern_cxxqt_block
            .qobjects
            .iter()
            .map(|ty| -> Result<GeneratedRustFragment> {
                let mut generated = vec![];
                let qobject_names = QObjectNames::from_extern_qobject(ty, type_names)?;

                generated.push(GeneratedRustFragment::generate_casting_impl(
                    &qobject_names,
                    type_names,
                    &ty.name,
                    &ty.base_class,
                )?);

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
                    let cxx_name = cpp_name;
                    quote! {
                        #[cxx_name = #cxx_name]
                    }
                };
                let cfgs: Vec<&Attribute> = ty
                    .declaration
                    .attrs
                    .iter()
                    .filter(|attr| path_compare_str(attr.meta.path(), &["cfg"]))
                    .collect();
                let docs: Vec<&Attribute> = ty
                    .declaration
                    .attrs
                    .iter()
                    .filter(|attr| path_compare_str(attr.meta.path(), &["doc"]))
                    .collect();
                generated.push(GeneratedRustFragment::from_cxx_item(parse_quote! {
                    #extern_block_namespace
                    #unsafety extern "C++" {
                        #namespace
                        #cxx_name
                        #(#cfgs)*
                        #(#docs)*
                        #vis type #ident;
                    }
                }));
                Ok(GeneratedRustFragment::flatten(generated))
            })
            .collect::<Result<Vec<_>>>()?;

        if !items.is_empty() {
            generated.push(GeneratedRustFragment::from_cxx_item(parse_quote! {
                #extern_block_namespace
                #unsafety extern "C++" {
                    #(#items)*
                }
            }));
        }

        // Build the signals
        for signal in &extern_cxxqt_block.signals {
            let qobject_name = type_names.lookup(&signal.qobject_ident)?;
            generated.push(generate_rust_signal(
                signal,
                qobject_name,
                type_names,
                // Copy the same safety as the extern C++Qt block into the generated extern C++
                extern_cxxqt_block.unsafety.map(|_| quote! { unsafe }),
            )?);
        }

        Ok(GeneratedRustFragment::flatten(generated))
    }
}
