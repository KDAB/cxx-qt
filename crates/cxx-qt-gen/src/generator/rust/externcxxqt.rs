// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::rust::{
        fragment::{GeneratedRustFragment, RustFragmentPair},
        signals::generate_rust_signal,
    },
    naming::TypeNames,
    parser::externcxxqt::ParsedExternCxxQt,
};
use quote::quote;
use syn::Result;

impl GeneratedRustFragment {
    pub fn from_extern_cxx_qt(
        extern_cxxqt_block: &ParsedExternCxxQt,
        type_names: &TypeNames,
    ) -> Result<Self> {
        let mut generated = Self::default();

        let namespace = if let Some(namespace) = &extern_cxxqt_block.namespace {
            quote! { #[namespace = #namespace ] }
        } else {
            quote! {}
        };

        // Add the pass through blocks
        let unsafety = &extern_cxxqt_block.unsafety;
        let items = &extern_cxxqt_block.passthrough_items;
        let fragment = RustFragmentPair {
            cxx_bridge: vec![quote! {
                #namespace
                #unsafety extern "C++" {
                    #(#items)*
                }
            }],
            implementation: vec![],
        };
        generated
            .cxx_mod_contents
            .append(&mut fragment.cxx_bridge_as_items()?);

        // Build the signals
        for signal in &extern_cxxqt_block.signals {
            let qobject_name = type_names.lookup(&signal.qobject_ident)?;

            generated.append(&mut generate_rust_signal(signal, qobject_name, type_names)?);
        }

        Ok(generated)
    }
}
