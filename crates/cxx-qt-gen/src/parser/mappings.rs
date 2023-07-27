// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeMap;

use syn::{Ident, Path};

#[derive(Default)]
pub struct ParsedCxxMappings {
    /// Map of the cxx_name of any types defined in CXX extern blocks
    ///
    /// This is used in the C++ generation to map the Rust type name to the C++ name
    pub cxx_names: BTreeMap<String, String>,
    /// Map of the namespace of any types or methods defined in CXX extern blocks
    ///
    /// This is used in the C++ generation to map the Rust type name to the C++ name
    pub namespaces: BTreeMap<String, String>,
    /// Mappings for CXX types when used outside the bridge
    pub qualified: BTreeMap<Ident, Path>,
}

impl ParsedCxxMappings {
    /// For a given rust ident return the CXX name with its namespace
    pub fn cxx(&self, ident: &str) -> String {
        // Check if there is a cxx_name or namespace to handle
        let cxx_name = self
            .cxx_names
            .get(ident)
            .cloned()
            .unwrap_or_else(|| ident.to_owned());

        if let Some(namespace) = self.namespaces.get(ident) {
            format!("::{namespace}::{cxx_name}")
        } else {
            cxx_name
        }
    }
}
