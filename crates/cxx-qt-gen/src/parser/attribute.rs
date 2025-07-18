// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{parser, syntax::path::path_compare_str};
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use syn::{spanned::Spanned, Attribute, Error, Result};

#[derive(Clone)]
pub struct ParsedAttributes {
    pub cxx_qt_attrs: BTreeMap<String, Vec<Attribute>>,
    pub passthrough_attrs: Vec<Attribute>,
}

// TODO: ATTR could this instead be used as Result<ParsedAttribute> to encapsulate error states
pub enum ParsedAttribute<'a> {
    /// A single attribute was found
    Single(&'a Attribute),
    /// An attribute was not found, but this is ok
    Absent,
    /// An attribute was not found, and this is an error
    AbsentRequired,
    /// Multiple attributes were found, but this is ok
    Multiple(Vec<&'a Attribute>),
    /// Multiple attributes were found, but this is an error
    MultipleDisallowed(Vec<&'a Attribute>),
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

impl<'a> ParsedAttributes {
    /// Collects a Map of all attributes found from the allowed list
    /// Will error if an attribute which is not in the allowed list is found
    pub fn require_attributes(
        mut attrs: Vec<Attribute>,
        allowed: &'a [&str],
    ) -> Result<ParsedAttributes> {
        let mut output = BTreeMap::<String, Vec<Attribute>>::default();
        for attr in attrs.drain(..) {
            let index = allowed
                .iter()
                .position(|string| path_compare_str(attr.meta.path(), &parser::split_path(string)));
            if let Some(index) = index {
                // TODO: ATTR Doesn't error on duplicates / distinguish allowed and disallowed duplicates
                match output.entry(allowed[index].into()) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(attr);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![attr]);
                    }
                }
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
            passthrough_attrs: Default::default(), // TODO: ATTR Pass the actual docs, cfgs, etc... here
        })
    }

    // TODO: ATTR Can this return references instead?
    pub fn extract_docs(&self) -> Vec<Attribute> {
        self.cxx_qt_attrs
            .values()
            .flatten()
            .filter(|attr| path_compare_str(attr.meta.path(), &["doc"]))
            .map(|attr| (*attr).clone())
            .collect()
    }

    // TODO: ATTR Can this return references instead
    pub fn extract_cfgs(&self) -> Vec<Attribute> {
        self.cxx_qt_attrs
            .values()
            .flatten()
            .filter(|attr| path_compare_str(attr.meta.path(), &["cfg"]))
            .map(|attr| (*attr).clone())
            .collect()
    }

    /// Returns all the attributes stored within the struct
    /// TODO: ATTR Can we use this without clone
    pub fn clone_attrs(&self) -> Vec<Attribute> {
        self.cxx_qt_attrs
            .values()
            .flatten()
            .cloned()
            .collect::<Vec<_>>()
    }

    // Wrapper methods for the internal BTreeMaps
    // TODO: Refactor usage to use more specialised methods / rename

    /// Search in first the CXX-Qt, and then passthrough attributes by key
    pub fn get_one(&self, key: &str) -> Option<&Attribute> {
        self.cxx_qt_attrs.get(key)?.first()
    }

    pub fn require_one(&self, key: &str) -> ParsedAttribute {
        if let Some(attrs) = self.cxx_qt_attrs.get(key) {
            if attrs.len() != 1 {
                ParsedAttribute::MultipleDisallowed(attrs.iter().by_ref().collect())
            } else {
                ParsedAttribute::Single(attrs.first().expect("Expected at least one attribute"))
            }
        } else {
            ParsedAttribute::Absent
        }
    }

    /// Check if CXX-Qt or passthrough attributes contains a particular key
    pub fn contains_key(&self, key: &str) -> bool {
        self.cxx_qt_attrs.contains_key(key) // TODO: Check in passthrough too
    }
}
