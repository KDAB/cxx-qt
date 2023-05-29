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

    let ident = &qobject_idents.cpp_class.cpp;
    let cxx_qt_thread_ident = &qobject_idents.cxx_qt_thread_class;

    result.forward_declares.push(format!(
        "using {cxx_qt_thread_ident} = ::rust::cxxqtlib1::CxxQtThread<{ident}>;"
    ));
    result.members.push(CppFragment::Pair {
        header: format!("::std::shared_ptr<::rust::cxxqtlib1::CxxQtGuardedPointer<{ident}>> m_cxxQtThreadObj;"),
        source: format!(", m_cxxQtThreadObj(::std::make_shared<::rust::cxxqtlib1::CxxQtGuardedPointer<{ident}>>(this))"),
    });
    result.methods.push(CppFragment::Pair {
        header: format!("::std::unique_ptr<{cxx_qt_thread_ident}> qtThread() const;"),
        source: formatdoc! {
            r#"
            ::std::unique_ptr<{cxx_qt_thread_ident}>
            {ident}::qtThread() const
            {{
            return ::std::make_unique<{cxx_qt_thread_ident}>(m_cxxQtThreadObj, m_rustObjMutex);
            }}
            "#
        },
    });
    result.deconstructors.push(formatdoc! {
        r#"
        const auto guard = ::std::unique_lock(m_cxxQtThreadObj->mutex);
        m_cxxQtThreadObj->ptr = nullptr;
        "#
    });

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::qobject::tests::create_qobjectname;
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_generate_cpp_threading() {
        let qobject_idents = create_qobjectname();

        let generated = generate(&qobject_idents).unwrap();

        // forward declares
        assert_eq!(generated.forward_declares.len(), 1);

        assert_str_eq!(
            generated.forward_declares[0],
            "using MyObjectCxxQtThread = ::rust::cxxqtlib1::CxxQtThread<MyObject>;"
        );

        // members
        assert_eq!(generated.members.len(), 1);

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.members[0] {
            (header, source)
        } else {
            panic!("Expected pair")
        };
        assert_str_eq!(
            header,
            "::std::shared_ptr<::rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>> m_cxxQtThreadObj;"
        );
        assert_str_eq!(
            source,
            ", m_cxxQtThreadObj(::std::make_shared<::rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>(this))"
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
            "::std::unique_ptr<MyObjectCxxQtThread> qtThread() const;"
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            ::std::unique_ptr<MyObjectCxxQtThread>
            MyObject::qtThread() const
            {
            return ::std::make_unique<MyObjectCxxQtThread>(m_cxxQtThreadObj, m_rustObjMutex);
            }
            "#}
        );

        // deconstructors
        assert_eq!(generated.deconstructors.len(), 1);
        assert_str_eq!(
            generated.deconstructors[0],
            indoc! {r#"
            const auto guard = ::std::unique_lock(m_cxxQtThreadObj->mutex);
            m_cxxQtThreadObj->ptr = nullptr;
            "#}
        );
    }
}
