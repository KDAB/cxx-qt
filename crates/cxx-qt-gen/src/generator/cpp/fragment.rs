// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(PartialEq, Eq, Debug)]
/// A fragment of C++ code
pub enum CppFragment {
    /// A fragment which only both a header and a source
    Pair {
        /// The header of the fragment
        header: String,
        /// The source of the fragment
        source: String,
    },
    /// A fragment which only has a header
    Header(String),
    /// A fragment which only has a source
    Source(String),
}

impl Default for CppFragment {
    fn default() -> Self {
        CppFragment::Pair {
            header: String::new(),
            source: String::new(),
        }
    }
}

pub struct CppNamedType {
    pub ident: String,
    pub ty: String,
}
