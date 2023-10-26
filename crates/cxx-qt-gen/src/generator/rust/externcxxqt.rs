// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::rust::{
        fragment::{GeneratedRustFragment, RustFragmentPair},
        signals::generate_rust_free_signal,
    },
    parser::{externcxxqt::ParsedExternCxxQt, mappings::ParsedCxxMappings},
};
use quote::quote;
use syn::{Ident, Result};

impl GeneratedRustFragment {
    pub fn from_extern_cxx_qt(
        extern_cxxqt_block: &ParsedExternCxxQt,
        cxx_mappings: &ParsedCxxMappings,
        module_ident: &Ident,
    ) -> Result<Self> {
        let mut generated = Self::default();

        // Add the pass through blocks
        let attrs = &extern_cxxqt_block.attrs;
        let unsafety = &extern_cxxqt_block.unsafety;
        let items = &extern_cxxqt_block.passthrough_items;
        let fragment = RustFragmentPair {
            cxx_bridge: vec![quote! {
                #(#attrs)*
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
            generated.append(&mut generate_rust_free_signal(
                signal,
                cxx_mappings,
                module_ident,
            )?);
        }

        Ok(generated)
    }
}
