// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::fragment::CppFragment,
    naming::property::{NameState, QPropertyNames},
};

pub fn generate(idents: &QPropertyNames, return_cxx_ty: &str) -> Option<CppFragment> {
    if let NameState::Auto(name) = &idents.getter {
        Some(CppFragment::Header(format!(
            "{return_cxx_ty} const& {ident_getter}() const noexcept;",
            ident_getter = name.cxx_unqualified()
        )))
    } else {
        None
    }
}
