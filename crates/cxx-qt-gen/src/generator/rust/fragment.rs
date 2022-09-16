// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro2::TokenStream;
use syn::{Item, Result};

/// A generic Rust CXX bridge definition and the corresponding implementation
pub struct RustFragmentPair {
    pub cxx_bridge: Vec<TokenStream>,
    pub implementation: Vec<TokenStream>,
}

impl RustFragmentPair {
    pub fn cxx_bridge_as_items(&self) -> Result<Vec<Item>> {
        self.cxx_bridge
            .iter()
            .map(|tokens| syn::parse2(tokens.clone()))
            .collect()
    }

    pub fn implementation_as_items(&self) -> Result<Vec<Item>> {
        self.implementation
            .iter()
            .map(|tokens| syn::parse2(tokens.clone()))
            .collect()
    }
}
