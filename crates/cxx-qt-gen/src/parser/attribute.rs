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

pub enum AttributeConstraint {
    /// Indicates that there must be only one of this attribute
    Unique,
    /// Indicates there can be multiple of this attribute
    Duplicate,
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
    /// Will error if an attribute which is not in the allowed list is found, or attribute is used incorrectly
    pub fn require_attributes(
        mut attrs: Vec<Attribute>,
        allowed: &'a [(AttributeConstraint, &str)],
    ) -> Result<ParsedAttributes> {
        let mut output = BTreeMap::<String, Vec<Attribute>>::default();
        // Iterate all attributes found
        for attr in attrs.drain(..) {
            let index = allowed.iter().position(|(_, string)| {
                path_compare_str(attr.meta.path(), &parser::split_path(string))
            });
            if let Some(index) = index {
                match allowed[index].0 {
                    AttributeConstraint::Unique => {
                        match output.entry(allowed[index].1.into()) {
                            Entry::Occupied(_) => return Err(Error::new_spanned(
                                attr,
                                "There must be at most one of this attribute on this given item",
                            )),
                            Entry::Vacant(entry) => {
                                entry.insert(vec![attr]);
                            }
                        }
                    }
                    AttributeConstraint::Duplicate => match output.entry(allowed[index].1.into()) {
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().push(attr);
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(vec![attr]);
                        }
                    },
                }
            } else {
                return Err(Error::new(
                    attr.span(),
                    format!(
                        "Unsupported attribute! The only attributes allowed on this item are\n{}",
                        allowed
                            .iter()
                            .map(|(_, string)| *string)
                            .collect::<Vec<_>>()
                            .join(", ")
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

    /// Check if CXX-Qt or passthrough attributes contains a particular key
    pub fn contains_key(&self, key: &str) -> bool {
        self.cxx_qt_attrs.contains_key(key) // TODO: Check in passthrough too
    }
}
