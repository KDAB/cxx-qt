// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{cpp::fragment::CppFragment, naming::property::QPropertyName};
use indoc::formatdoc;

pub fn generate(idents: &QPropertyName, qobject_ident: &str, cxx_ty: &str) -> CppFragment {
    CppFragment::Pair {
        header: format!(
            "Q_SLOT void {ident_setter}({cxx_ty} const& value);",
            ident_setter = idents.setter.cpp,
        ),
        source: formatdoc! {
            r#"
            void
            {qobject_ident}::{ident_setter}({cxx_ty} const& value)
            {{
                const ::rust::cxxqt1::MaybeLockGuard<{qobject_ident}> guard(*this);
                {ident_setter_wrapper}(value);
            }}
            "#,
            ident_setter = idents.setter.cpp,
            ident_setter_wrapper = idents.setter_wrapper.cpp.to_string(),
        },
    }
}

pub fn generate_wrapper(idents: &QPropertyName, cxx_ty: &str) -> CppFragment {
    CppFragment::Header(format!(
        // Note that we pass T not const T& to Rust so that it is by-value
        // https://github.com/KDAB/cxx-qt/issues/463
        "void {ident_setter_wrapper}({cxx_ty} value) noexcept;",
        ident_setter_wrapper = idents.setter_wrapper.cpp
    ))
}
