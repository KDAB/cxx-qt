// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(PartialEq, Eq, Debug)]
pub enum CppFragment {
    Pair { header: String, source: String },
    Header(String),
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
