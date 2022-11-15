// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::{fragment::CppFragment, types::CppType, CXX_QT_CONVERT, RUST_OBJ_MUTEX_LOCK_GUARD},
    naming::property::QPropertyName,
};
use indoc::formatdoc;

pub fn generate(idents: &QPropertyName, qobject_ident: &str, cxx_ty: &CppType) -> CppFragment {
    CppFragment::Pair {
        header: format!(
            "const {cxx_ty}& {ident_getter}() const;",
            cxx_ty = cxx_ty.as_cxx_ty(),
            ident_getter = idents.getter.cpp
        ),
        source: formatdoc!(
            r#"
            const {cxx_ty}&
            {qobject_ident}::{ident_getter}() const
            {{
                {rust_obj_guard}
                return {convert}<const {cxx_ty}&, const {rust_ty}&>{{}}(m_rustObj->{ident_getter}(*this));
            }}
            "#,
            convert = CXX_QT_CONVERT,
            cxx_ty = cxx_ty.as_cxx_ty(),
            ident_getter = idents.getter.cpp.to_string(),
            qobject_ident = qobject_ident,
            rust_obj_guard = RUST_OBJ_MUTEX_LOCK_GUARD,
            rust_ty = cxx_ty.as_rust_ty(),
        ),
    }
}
