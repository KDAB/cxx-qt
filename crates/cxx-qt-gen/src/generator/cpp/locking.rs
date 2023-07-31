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

pub fn generate(qobject_idents: &QObjectName) -> Result<(String, GeneratedCppQObjectBlocks)> {
    let mut result = GeneratedCppQObjectBlocks::default();

    let lock_guard_mutex = "::std::lock_guard<::std::recursive_mutex>";
    let qobject_ident = qobject_idents.cpp_class.cpp.to_string();

    result.includes.insert("#include <mutex>".to_owned());

    result.private_methods.push(CppFragment::Pair {
        header: format!("[[nodiscard]] {lock_guard_mutex} unsafeRustLock() const;"),
        source: formatdoc! {
            r#"
            {lock_guard_mutex}
            {qobject_ident}::unsafeRustLock() const
            {{
              return {lock_guard_mutex}(*m_rustObjMutex);
            }}
            "#
        },
    });

    result
        .members
        .push("::std::shared_ptr<::std::recursive_mutex> m_rustObjMutex;".to_owned());

    let member_initializer =
        "m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())".to_owned();

    Ok((member_initializer, result))
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

        let (initializer, generated) = generate(&qobject_idents).unwrap();

        // includes
        assert_eq!(generated.includes.len(), 1);
        assert!(generated.includes.contains("#include <mutex>"));

        // members
        assert_eq!(generated.members.len(), 1);
        assert_str_eq!(
            &generated.members[0],
            "::std::shared_ptr<::std::recursive_mutex> m_rustObjMutex;"
        );
        assert_str_eq!(
            initializer,
            "m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())"
        );

        // private methods
        assert_eq!(generated.private_methods.len(), 1);

        let (header, source) =
            if let CppFragment::Pair { header, source } = &generated.private_methods[0] {
                (header, source)
            } else {
                panic!("Expected pair")
            };
        assert_str_eq!(
            header,
            "[[nodiscard]] ::std::lock_guard<::std::recursive_mutex> unsafeRustLock() const;"
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            ::std::lock_guard<::std::recursive_mutex>
            MyObject::unsafeRustLock() const
            {
              return ::std::lock_guard<::std::recursive_mutex>(*m_rustObjMutex);
            }
            "#}
        );
    }
}
