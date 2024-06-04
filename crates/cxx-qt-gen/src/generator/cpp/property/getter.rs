// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{cpp::fragment::CppFragment, naming::property::QPropertyNames};
use indoc::formatdoc;

pub fn generate(idents: &QPropertyNames, qobject_ident: &str, return_cxx_ty: &str) -> CppFragment {
    CppFragment::Pair {
        header: format!(
            "{return_cxx_ty} const& {ident_getter}() const;",
            ident_getter = idents.getter.cxx_unqualified()
        ),
        source: formatdoc!(
            r#"
            {return_cxx_ty} const&
            {qobject_ident}::{ident_getter}() const
            {{
                const ::rust::cxxqt1::MaybeLockGuard<{qobject_ident}> guard(*this);
                return {ident_getter_wrapper}();
            }}
            "#,
            ident_getter = idents.getter.cxx_unqualified(),
            ident_getter_wrapper = idents.getter_wrapper.cxx_unqualified(),
        ),
    }
}

pub fn generate_wrapper(idents: &QPropertyNames, cxx_ty: &str) -> CppFragment {
    CppFragment::Header(format!(
        "{cxx_ty} const& {ident_getter_wrapper}() const noexcept;",
        ident_getter_wrapper = idents.getter_wrapper.cxx_unqualified()
    ))
}
