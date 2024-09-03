// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro2::TokenStream;
use syn::{Item, Result};

#[derive(Default, Eq, PartialEq, Debug)]
pub struct GeneratedRustFragment {
    /// Module for the CXX bridge
    pub cxx_mod_contents: Vec<Item>,
    /// Items for the CXX-Qt module
    pub cxx_qt_mod_contents: Vec<Item>,
}

impl GeneratedRustFragment {
    pub fn append(&mut self, other: &mut Self) {
        self.cxx_mod_contents.append(&mut other.cxx_mod_contents);
        self.cxx_qt_mod_contents
            .append(&mut other.cxx_qt_mod_contents);
    }
}

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
