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

    let cpp_class = &qobject_idents.cpp_class.cpp;
    let cxx_qt_thread_ident = &qobject_idents.cxx_qt_thread_class;

    result.forward_declares.push(format!(
        "using {cxx_qt_thread_ident} = ::rust::cxxqt1::CxxQtThread<{cpp_class}>;"
    ));
    // Ensure that the CxxQtThread<T> is of the correct size and alignment
    // which should be two std::shared_ptr which are two size_t each
    result.methods.push(CppFragment::Source(formatdoc! {
        r#"
        static_assert(alignof({cxx_qt_thread_ident}) <= alignof(::std::size_t), "unexpected aligment");
        static_assert(sizeof({cxx_qt_thread_ident}) == sizeof(::std::size_t[4]), "unexpected size");
        "#
    }));

    result
        .includes
        .insert("#include <cxx-qt-common/cxxqt_threading.h>".to_owned());

    result
        .base_classes
        .push(format!("::rust::cxxqt1::CxxQtThreading<{cpp_class}>"));

    let class_initializer = format!("::rust::cxxqt1::CxxQtThreading<{cpp_class}>(this)");

    Ok((class_initializer, result))
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
            "using MyObjectCxxQtThread = ::rust::cxxqt1::CxxQtThread<MyObject>;"
        );

        // initialiser
        assert_str_eq!(
            initializer,
            "::rust::cxxqt1::CxxQtThreading<MyObject>(this)"
        );

        // methods
        assert_eq!(generated.methods.len(), 1);

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

        // includes
        assert_eq!(generated.includes.len(), 1);
        assert!(generated
            .includes
            .contains("#include <cxx-qt-common/cxxqt_threading.h>"));

        // base class
        assert_eq!(generated.base_classes.len(), 1);
        assert_eq!(
            generated.base_classes[0],
            "::rust::cxxqt1::CxxQtThreading<MyObject>"
        );
    }
}
