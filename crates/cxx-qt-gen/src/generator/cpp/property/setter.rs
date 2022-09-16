// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::{fragment::CppFragmentPair, types::CppType, CXX_QT_CONVERT, RUST_OBJ_MUTEX_LOCK_GUARD},
    naming::property::QPropertyName,
};
use indoc::formatdoc;

pub fn generate(idents: &QPropertyName, qobject_ident: &str, cxx_ty: &CppType) -> CppFragmentPair {
    CppFragmentPair {
        header: format!(
            "void {ident_setter}(const {cxx_ty}& value);",
            cxx_ty = cxx_ty.as_cxx_ty(),
            ident_setter = idents.setter.cpp,
        ),
        source: formatdoc! {
            r#"
            void
            {qobject_ident}::{ident_setter}(const {cxx_ty}& value)
            {{
                {rust_obj_guard}
                m_rustObj->{ident_setter}(*this, {convert}<{rust_ty}, const {cxx_ty}&>{{}}(value));
            }}
            "#,
            convert = CXX_QT_CONVERT,
            cxx_ty = cxx_ty.as_cxx_ty(),
            ident_setter = idents.setter.cpp,
            qobject_ident = qobject_ident,
            rust_obj_guard = RUST_OBJ_MUTEX_LOCK_GUARD,
            rust_ty = cxx_ty.as_rust_ty(),
        },
    }
}
