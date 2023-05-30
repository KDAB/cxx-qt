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

    let ident = &qobject_idents.cpp_class.cpp;
    let cxx_qt_thread_ident = &qobject_idents.cxx_qt_thread_class;

    result.forward_declares.push(format!(
        "using {cxx_qt_thread_ident} = ::rust::cxxqtlib1::CxxQtThread<{ident}>;"
    ));
    // Ensure that the CxxQtThread<T> is of the correct size and alignment
    // which should be two std::shared_ptr which are two size_t each
    result.methods.push(CppFragment::Source(formatdoc! {
        r#"
        static_assert(alignof({cxx_qt_thread_ident}) <= alignof(::std::size_t), "unexpected aligment");
        static_assert(sizeof({cxx_qt_thread_ident}) == sizeof(::std::size_t[4]), "unexpected size");
        "#
    }));
    result.members.push(format!(
        "::std::shared_ptr<::rust::cxxqtlib1::CxxQtGuardedPointer<{ident}>> m_cxxQtThreadObj;"
    ));
    result.methods.push(CppFragment::Pair {
        header: format!("{cxx_qt_thread_ident} qtThread() const;"),
        source: formatdoc! {
            r#"
            {cxx_qt_thread_ident}
            {ident}::qtThread() const
            {{
              return {cxx_qt_thread_ident}(m_cxxQtThreadObj, m_rustObjMutex);
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

    let member_initializer = format!("m_cxxQtThreadObj(::std::make_shared<::rust::cxxqtlib1::CxxQtGuardedPointer<{ident}>>(this))");

    Ok((member_initializer, result))
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

        let (initializer, generated) = generate(&qobject_idents).unwrap();

        // forward declares
        assert_eq!(generated.forward_declares.len(), 1);

        assert_str_eq!(
            generated.forward_declares[0],
            "using MyObjectCxxQtThread = ::rust::cxxqtlib1::CxxQtThread<MyObject>;"
        );

        // members
        assert_eq!(generated.members.len(), 1);
        assert_str_eq!(
            &generated.members[0],
            "::std::shared_ptr<::rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>> m_cxxQtThreadObj;"
        );
        assert_str_eq!(
            initializer,
            "m_cxxQtThreadObj(::std::make_shared<::rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>(this))"
        );

        // methods
        assert_eq!(generated.methods.len(), 2);

        let source = if let CppFragment::Source(source) = &generated.methods[0] {
            source
        } else {
            panic!("Expected source")
        };
        assert_str_eq!(
            source,
            indoc! {r#"
            static_assert(alignof(MyObjectCxxQtThread) <= alignof(::std::size_t), "unexpected aligment");
            static_assert(sizeof(MyObjectCxxQtThread) == sizeof(::std::size_t[4]), "unexpected size");
            "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[1] {
            (header, source)
        } else {
            panic!("Expected pair")
        };
        assert_str_eq!(header, "MyObjectCxxQtThread qtThread() const;");
        assert_str_eq!(
            source,
            indoc! {r#"
            MyObjectCxxQtThread
            MyObject::qtThread() const
            {
              return MyObjectCxxQtThread(m_cxxQtThreadObj, m_rustObjMutex);
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
