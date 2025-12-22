// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::{fragment::CppFragment, qobject::GeneratedCppQObjectBlocks},
    naming::qobject::QObjectNames,
};
use indoc::formatdoc;
use syn::Result;

pub fn generate(qobject_idents: &QObjectNames) -> Result<(String, GeneratedCppQObjectBlocks)> {
    let mut result = GeneratedCppQObjectBlocks::default();

    let cpp_class = &qobject_idents.name.cxx_unqualified();
    let cxx_qt_thread_ident = &qobject_idents.cxx_qt_thread_class;

    result.forward_declares.push(format!(
        "using {cxx_qt_thread_ident} = ::rust::cxxqt1::CxxQtThread<{cpp_class}>;"
    ));
    // Ensure that the CxxQtThread<T> is of the correct size and alignment
    // which should be two std::shared_ptr which are two size_t each
    result.methods.push(CppFragment::Source(formatdoc! {
        r#"
        static_assert(alignof({cxx_qt_thread_ident}) <= alignof(::std::size_t), "unexpected aligment");
        static_assert(sizeof({cxx_qt_thread_ident}) == sizeof(::std::size_t[2]), "unexpected size");
        "#
    }));

    result
        .includes
        .insert("#include <cxx-qt/threading.h>".to_owned());

    // This is meant to be used from Rust side only, so use private inheritance for now.
    result.base_classes.push(format!(
        "private ::rust::cxxqt1::CxxQtThreading<{cpp_class}>"
    ));
    // Note: Use auto syntax here, because otherwise it is not possible to fully
    // qualify the qtThread function.
    //
    // e.g.:
    // ```
    // friend rust::cxxqt1::CxxQtThread<T> ::rust::cxxqt1::qtThread<T>(const MyType&
    // qobject);
    // ```
    // is parsed as:
    // ```
    // friend rust::cxxqt1::CxxQtThread<T>::rust::cxxqt1::qtThread<T>(const MyType&
    // qobject);
    // ```
    // Because the `::` after `CxxQtThread<T>` is scope resolution operator it
    // applies to the type `CxxQtThread<T>` rather than starting a new scope
    // resolution from the global namespace.
    result.private_methods.push(CppFragment::Header(
        "template<typename T>\nfriend auto ::rust::cxxqt1::qtThread(const T& qobject) -> ::rust::cxxqt1::CxxQtThread<T>;"
            .to_owned(),
    ));

    let class_initializer = format!("::rust::cxxqt1::CxxQtThreading<{cpp_class}>(this)");

    Ok((class_initializer, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::cpp::property::tests::require_source;
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
            "using MyObjectCxxQtThread = ::rust::cxxqt1::CxxQtThread<MyObject>;"
        );

        // initialiser
        assert_str_eq!(
            initializer,
            "::rust::cxxqt1::CxxQtThreading<MyObject>(this)"
        );

        // methods
        assert_eq!(generated.methods.len(), 1);

        let source = require_source(&generated.methods[0]).unwrap();
        assert_str_eq!(
            source,
            indoc! {r#"
            static_assert(alignof(MyObjectCxxQtThread) <= alignof(::std::size_t), "unexpected aligment");
            static_assert(sizeof(MyObjectCxxQtThread) == sizeof(::std::size_t[2]), "unexpected size");
            "#}
        );

        // includes
        assert_eq!(generated.includes.len(), 1);
        assert!(generated.includes.contains("#include <cxx-qt/threading.h>"));

        // base class
        assert_eq!(generated.base_classes.len(), 1);
        assert_eq!(
            generated.base_classes[0],
            "private ::rust::cxxqt1::CxxQtThreading<MyObject>"
        );
        // friend declaration
        assert_eq!(generated.private_methods.len(), 1);
        let CppFragment::Header(friend_decl) = &generated.private_methods[0] else {
            // CODECOV_EXCLUDE_START
            panic!("Expected header fragment");
            // CODECOV_EXCLUDE_STOP
        };
        assert_str_eq!(
            friend_decl,
            "template<typename T>\nfriend auto ::rust::cxxqt1::qtThread(const T& qobject) -> ::rust::cxxqt1::CxxQtThread<T>;"
        );
    }
}
