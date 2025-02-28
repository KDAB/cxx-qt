// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::Item;

#[derive(Default, Eq, PartialEq, Debug)]
pub struct GeneratedRustFragment {
    /// Module for the CXX bridge
    pub cxx_mod_contents: Vec<Item>,
    /// Items for the CXX-Qt module
    pub cxx_qt_mod_contents: Vec<Item>,
}

impl GeneratedRustFragment {
    pub fn append(&mut self, other: Self) {
        self.cxx_mod_contents.extend(other.cxx_mod_contents);
        self.cxx_qt_mod_contents.extend(other.cxx_qt_mod_contents);
    }

    // Create a singular GeneratedRustFragment from a Vector of multiple
    pub fn flatten(others: Vec<Self>) -> Self {
        let mut this = Self::default();
        for other in others {
            this.append(other);
        }
        this
    }

    pub fn from_cxx_item(contents: Item) -> Self {
        Self {
            cxx_mod_contents: vec![contents],
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cxx_mod_contents.is_empty() && self.cxx_qt_mod_contents.is_empty()
    }
}
