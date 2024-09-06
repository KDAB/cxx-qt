// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::fragment::CppFragment,
    naming::property::{NameState, QPropertyNames},
};

pub fn generate(idents: &QPropertyNames, cxx_ty: &str) -> Option<CppFragment> {
    // Only generates setter code if the state provided is Auto (not custom provided by user)
    if let Some(NameState::Auto(setter)) = &idents.setter {
        Some(CppFragment::Header(format!(
            "Q_SLOT void {ident_setter}({cxx_ty} value) noexcept;",
            ident_setter = setter.cxx_unqualified(),
        )))
    } else {
        None
    }
}
