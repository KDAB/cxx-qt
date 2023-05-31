// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::{fragment::CppFragment, types::CppType},
    naming::property::QPropertyName,
};
use indoc::formatdoc;

pub fn generate(
    idents: &QPropertyName,
    qobject_ident: &str,
    cxx_ty: &CppType,
    lock_guard: Option<&str>,
) -> CppFragment {
    CppFragment::Pair {
        header: format!(
            "{return_cxx_ty} const& {ident_getter}() const;",
            return_cxx_ty = cxx_ty.as_cxx_ty(),
            ident_getter = idents.getter.cpp
        ),
        source: formatdoc!(
            r#"
            {return_cxx_ty} const&
            {qobject_ident}::{ident_getter}() const
            {{
                {rust_obj_guard}
                return m_rustObj->{ident_getter}(*this);
            }}
            "#,
            return_cxx_ty = cxx_ty.as_cxx_ty(),
            ident_getter = idents.getter.cpp.to_string(),
            qobject_ident = qobject_ident,
            rust_obj_guard = lock_guard.unwrap_or_default(),
        ),
    }
}
