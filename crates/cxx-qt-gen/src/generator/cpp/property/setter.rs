// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::fragment::CppFragment,
    naming::property::{NameState, QPropertyNames},
};
use crate::naming::Name;
use indoc::formatdoc;

pub fn generate(idents: &QPropertyNames, qobject_name: &Name, cxx_ty: &str) -> Option<CppFragment> {
    // Only generates setter code if the state provided is Auto (not custom provided by user)
    if let (Some(NameState::Auto(setter)), Some(setter_wrapper)) =
        (&idents.setter, &idents.setter_wrapper)
    {
        Some(CppFragment::Pair {
            header: format!(
                "Q_SLOT void {ident_setter}({cxx_ty} const& value);",
                ident_setter = setter.cxx_unqualified(),
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
                qobject_ident = qobject_name.cxx_unqualified(),
                ident_setter = setter.cxx_unqualified(),
                ident_setter_wrapper = setter_wrapper.cxx_unqualified(),
            },
        })
    } else {
        None
    }
}

pub fn generate_wrapper(idents: &QPropertyNames, cxx_ty: &str) -> Option<CppFragment> {
    idents.setter_wrapper.as_ref().map(|setter_wrapper| {
        CppFragment::Header(format!(
            // Note that we pass T not const T& to Rust so that it is by-value
            // https://github.com/KDAB/cxx-qt/issues/463
            "void {ident_setter_wrapper}({cxx_ty} value) noexcept;",
            ident_setter_wrapper = setter_wrapper.cxx_unqualified()
        ))
    })
}
