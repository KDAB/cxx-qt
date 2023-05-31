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
            "Q_SLOT void {ident_setter}({cxx_ty} const& value);",
            cxx_ty = cxx_ty.as_cxx_ty(),
            ident_setter = idents.setter.cpp,
        ),
        source: formatdoc! {
            r#"
            void
            {qobject_ident}::{ident_setter}({cxx_ty} const& value)
            {{
                {rust_obj_guard}
                m_rustObj->{ident_setter}(*this, value);
            }}
            "#,
            cxx_ty = cxx_ty.as_cxx_ty(),
            ident_setter = idents.setter.cpp,
            qobject_ident = qobject_ident,
            rust_obj_guard = lock_guard.unwrap_or_default(),
        },
    }
}
