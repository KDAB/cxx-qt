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
use syn::{parse_quote, Attribute, Item, Result};

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
        let extern_block_docs = &extern_cxxqt_block.common_attrs.docs;

        // Add the pass through blocks
        let unsafety = &extern_cxxqt_block.unsafety;
        let items = &extern_cxxqt_block.passthrough_items;

        let mut qobject_items: Vec<Item> = vec![];
        let mut cxx_qt_mod = vec![];
        let mut cxx_mod = vec![];
        for obj in &extern_cxxqt_block.qobjects {
            let qobject_names = QObjectNames::from_extern_qobject(obj, type_names)?;

            let casting = GeneratedRustFragment::generate_casting_impl(
                &qobject_names,
                type_names,
                &obj.name,
                &obj.base_class,
            )?;
            cxx_mod.extend(casting.cxx_mod_contents);
            cxx_qt_mod.extend(casting.cxx_qt_mod_contents);

            let namespace = if let Some(namespace) = &obj.name.namespace() {
                quote! { #[namespace = #namespace ] }
            } else {
                quote! {}
            };
            let cpp_name = &obj.name.cxx_unqualified();
            let rust_name = &obj.name.rust_unqualified();
            let vis = &obj.declaration.vis;
            let ident = &obj.name.rust_unqualified();
            let cxx_name = if &rust_name.to_string() == cpp_name {
                quote! {}
            } else {
                let cxx_name = cpp_name.to_string();
                quote! {
                    #[cxx_name = #cxx_name]
                }
            };
            // TODO! Can we make extract_docs return references, and then use here?
            let cfgs: Vec<&Attribute> = obj
                .declaration
                .attrs
                .iter()
                .filter(|attr| path_compare_str(attr.meta.path(), &["cfg"]))
                .collect();
            let docs: Vec<&Attribute> = obj
                .declaration
                .attrs
                .iter()
                .filter(|attr| path_compare_str(attr.meta.path(), &["doc"]))
                .collect();
            qobject_items.push(parse_quote! {
                #namespace
                #cxx_name
                #(#cfgs)*
                #(#docs)*
                #vis type #ident;
            });
        }

        let passthrough_items = if !items.is_empty() {
            quote! {
                #(#items)*
            }
        } else {
            quote! {}
        };

        cxx_mod.push(parse_quote! {
            #extern_block_namespace
            #(#extern_block_docs)*
            #unsafety extern "C++" {
                #(#qobject_items)*

                #passthrough_items
            }
        });

        let mut generated = vec![GeneratedRustFragment {
            cxx_mod_contents: cxx_mod,
            cxx_qt_mod_contents: cxx_qt_mod,
        }];

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
