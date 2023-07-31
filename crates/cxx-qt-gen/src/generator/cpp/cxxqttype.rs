// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::{fragment::CppFragment, qobject::GeneratedCppQObjectBlocks},
    naming::qobject::QObjectName,
};
use indoc::formatdoc;
use syn::Result;

pub fn generate(qobject_idents: &QObjectName) -> Result<GeneratedCppQObjectBlocks> {
    let mut result = GeneratedCppQObjectBlocks::default();

    let rust_ident = qobject_idents.rust_struct.cpp.to_string();
    let qobject_ident = qobject_idents.cpp_class.cpp.to_string();

    result.includes.insert("#include <memory>".to_owned());

    result.methods.push(CppFragment::Pair {
        header: formatdoc! {
            r#"
            {rust_ident} const& unsafeRust() const;
            {rust_ident}& unsafeRustMut();
            "#
        },
        source: formatdoc! {
            r#"
            {rust_ident} const&
            {qobject_ident}::unsafeRust() const
            {{
              return *m_rustObj;
            }}

            {rust_ident}&
            {qobject_ident}::unsafeRustMut()
            {{
              return *m_rustObj;
            }}
            "#
        },
    });

    result
        .members
        .push(format!("::rust::Box<{rust_ident}> m_rustObj;"));

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::qobject::tests::create_qobjectname;
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_generate_cpp_locking() {
        let qobject_idents = create_qobjectname();

        let generated = generate(&qobject_idents).unwrap();

        // includes
        assert_eq!(generated.includes.len(), 1);
        assert!(generated.includes.contains("#include <memory>"));

        // members
        assert_eq!(generated.members.len(), 1);
        assert_str_eq!(
            &generated.members[0],
            "::rust::Box<MyObjectRust> m_rustObj;"
        );

        // methods
        assert_eq!(generated.methods.len(), 1);

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[0] {
            (header, source)
        } else {
            panic!("Expected pair")
        };
        assert_str_eq!(
            header,
            indoc! {r#"
            MyObjectRust const& unsafeRust() const;
            MyObjectRust& unsafeRustMut();
            "#}
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            MyObjectRust const&
            MyObject::unsafeRust() const
            {
              return *m_rustObj;
            }

            MyObjectRust&
            MyObject::unsafeRustMut()
            {
              return *m_rustObj;
            }
            "#}
        );
    }
}
