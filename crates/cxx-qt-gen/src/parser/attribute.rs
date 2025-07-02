// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{parser, syntax::path::path_compare_str};
use std::collections::BTreeMap;
use syn::{spanned::Spanned, Attribute, Error, Result};

pub struct ParsedAttribute<'a> {
    pub cxx_qt_attrs: BTreeMap<&'a str, &'a Attribute>,
    pub passthrough_attrs: BTreeMap<&'a str, &'a Attribute>,
}

/// Iterate the attributes of the method to extract cfg attributes
pub fn extract_cfgs(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|attr| path_compare_str(attr.meta.path(), &["cfg"]))
        .cloned()
        .collect()
}

/// Iterate the attributes of the method to extract Doc attributes (doc comments are parsed as this)
pub fn extract_docs(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|attr| path_compare_str(attr.meta.path(), &["doc"]))
        .cloned()
        .collect()
}

impl<'a> ParsedAttribute<'a> {
    /// Collects a Map of all attributes found from the allowed list
    /// Will error if an attribute which is not in the allowed list is found
    pub fn require_attributes(
        attrs: &'a [Attribute],
        allowed: &'a [&str],
    ) -> Result<ParsedAttribute<'a>> {
        let mut output = BTreeMap::default();
        for attr in attrs {
            let index = allowed
                .iter()
                .position(|string| path_compare_str(attr.meta.path(), &parser::split_path(string)));
            if let Some(index) = index {
                output.insert(allowed[index], attr); // TODO: Doesn't error on duplicates
            } else {
                return Err(Error::new(
                    attr.span(),
                    format!(
                        "Unsupported attribute! The only attributes allowed on this item are\n{}",
                        allowed.join(", ")
                    ),
                ));
            }
        }
        Ok(Self {
            cxx_qt_attrs: output,
            passthrough_attrs: Default::default(),
        })
    }

    // Wrapper methods for the internal BTreeMaps
    // TODO: Refactor usage to use more specialised methods / rename

    /// Search in first the CXX-Qt, and then passthrough attributes by key
    pub fn get(&self, key: &str) -> Option<&Attribute> {
        self.cxx_qt_attrs
            .get(key)
            .or(self.passthrough_attrs.get(key))
            .map(|attr| &**attr)
    }

    /// Check if CXX-Qt or passthrough attributes contains a particular key
    pub fn contains_key(&self, key: &str) -> bool {
        self.cxx_qt_attrs.contains_key(key) || self.passthrough_attrs.contains_key(key)
    }
}
