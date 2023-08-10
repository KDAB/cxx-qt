// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{cpp::fragment::CppFragment, naming::property::QPropertyName};
use indoc::formatdoc;

pub fn generate(idents: &QPropertyName, qobject_ident: &str, return_cxx_ty: &str) -> CppFragment {
    CppFragment::Pair {
        header: format!(
            "{return_cxx_ty} const& {ident_getter}() const;",
            ident_getter = idents.getter.cpp
        ),
        source: formatdoc!(
            r#"
            {return_cxx_ty} const&
            {qobject_ident}::{ident_getter}() const
            {{
                const ::rust::cxxqtlib1::MaybeLockGuard<{qobject_ident}> guard(*this);
                return {ident_getter_wrapper}();
            }}
            "#,
            ident_getter = idents.getter.cpp.to_string(),
            ident_getter_wrapper = idents.getter_wrapper.cpp.to_string(),
        ),
    }
}

pub fn generate_wrapper(idents: &QPropertyName, cxx_ty: &str) -> CppFragment {
    CppFragment::Header(format!(
        "{cxx_ty} const& {ident_getter_wrapper}() const noexcept;",
        ident_getter_wrapper = idents.getter_wrapper.cpp
    ))
}
