// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod header;
pub mod source;

use crate::generator::cpp::{fragment::CppFragment, GeneratedCppBlocks};
use clang_format::{clang_format_with_style, ClangFormatStyle};
use header::write_cpp_header;
use source::write_cpp_source;

/// For a given GeneratedCppBlocks write the namespace pair
pub fn namespace_pair(generated: &GeneratedCppBlocks) -> (String, String) {
    let namespace_start = if generated.namespace.is_empty() {
        "".to_owned()
    } else {
        format!("namespace {namespace} {{", namespace = generated.namespace)
    };
    let namespace_end = if generated.namespace.is_empty() {
        "".to_owned()
    } else {
        format!(
            "}} // namespace {namespace}",
            namespace = generated.namespace
        )
    };
    (namespace_start, namespace_end)
}

/// For a given GeneratedCppBlocks write this into a C++ header and source pair
pub fn write_cpp(generated: &GeneratedCppBlocks) -> CppFragment {
    let header = write_cpp_header(generated);
    let source = write_cpp_source(generated);

    CppFragment::Pair {
        header: clang_format_with_style(&header, &ClangFormatStyle::File).unwrap_or(header),
        source: clang_format_with_style(&source, &ClangFormatStyle::File).unwrap_or(source),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    use crate::{
        generator::cpp::qobject::{GeneratedCppQObject, GeneratedCppQObjectBlocks},
        tests::format_cpp,
    };
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;

    /// Helper to create a GeneratedCppBlocks for testing
    pub fn create_generated_cpp() -> GeneratedCppBlocks {
        GeneratedCppBlocks {
            cxx_file_stem: "cxx_file_stem".to_owned(),
            namespace: "cxx_qt::my_object".to_owned(),
            qobjects: vec![
                GeneratedCppQObject {
                    ident: "MyObject".to_owned(),
                    rust_ident: "MyObjectRust".to_owned(),
                    namespace_internals: "cxx_qt::my_object::cxx_qt_my_object".to_owned(),
                    blocks: GeneratedCppQObjectBlocks {
                        forward_declares: vec![],
                        base_classes: vec!["QStringListModel".to_owned()],
                        includes: {
                          let mut includes = BTreeSet::<String>::default();
                          includes.insert("#include <test>".to_owned());
                          includes
                        },
                        metaobjects: vec![
                            "Q_PROPERTY(int count READ count WRITE setCount NOTIFY countChanged)".to_owned(),
                            "Q_PROPERTY(bool longPropertyNameThatWrapsInClangFormat READ getToggle WRITE setToggle NOTIFY toggleChanged)"
                                .to_owned(),
                        ],
                        methods: vec![
                            CppFragment::Pair {
                                header: "int count() const;".to_owned(),
                                source: indoc! {r#"
                                    int
                                    MyObject::count() const
                                    {
                                      // getter
                                    }
                                "#}
                                .to_owned(),
                            },
                            CppFragment::Pair {
                                header: "bool toggle() const;".to_owned(),
                                source: indoc! {r#"
                                    bool
                                    MyObject::toggle() const
                                    {
                                      // getter
                                    }
                                "#}
                                .to_owned(),
                            },
                            CppFragment::Pair {
                                header: "Q_INVOKABLE void invokable();".to_owned(),
                                source: indoc! {r#"
                                    void
                                    MyObject::invokable()
                                    {
                                      // invokable method
                                    }
                                "#}
                                .to_owned(),
                            },
                            CppFragment::Pair {
                                header: "void cppMethod();".to_owned(),
                                source: indoc! {r#"
                                    void
                                    MyObject::cppMethod()
                                    {
                                      // cpp method
                                    }
                                "#}
                                .to_owned(),
                            },
                            CppFragment::Pair {
                                header: "Q_SLOT void setCount(int count);".to_owned(),
                                source: indoc! {r#"
                                    void
                                    MyObject::setCount(int count) const
                                    {
                                      // setter
                                    }
                                    "#}
                                .to_owned(),
                            },
                            CppFragment::Pair {
                                header: "Q_SLOT void setToggle(bool toggle);".to_owned(),
                                source: indoc! {r#"
                                    void
                                    MyObject::setToggle(bool toggle) const
                                    {
                                      // setter
                                    }
                                    "#}
                                .to_owned(),
                            },
                            CppFragment::Header (
                                "Q_SIGNAL void countChanged();".to_owned(),
                            ),
                            CppFragment::Header (
                                "Q_SIGNAL void toggleChanged();".to_owned(),
                            ),
                        ],
                        private_methods: vec![CppFragment::Pair{
                                header: "void privateMethod() const;".to_owned(),
                                source: indoc! {r#"
                                    void MyObject::privateMethod() const {
                                        // private method
                                    }
                                    "#}.to_owned(),
                            },
                        CppFragment::Pair{
                                header: "void privateMethod();".to_owned(),
                                source: indoc! {r#"
                                    void MyObject::privateMethod()
                                    {
                                        // non-const private method
                                    }
                                    "#}.to_owned(),
                            }]
                    }
                }
            ],
        }
    }

    /// Helper to create a GeneratedCppBlocks for testing with multiple qobjects
    pub fn create_generated_cpp_multi_qobjects() -> GeneratedCppBlocks {
        GeneratedCppBlocks {
            cxx_file_stem: "cxx_file_stem".to_owned(),
            namespace: "cxx_qt".to_owned(),
            qobjects: vec![
                GeneratedCppQObject {
                    ident: "FirstObject".to_owned(),
                    rust_ident: "FirstObjectRust".to_owned(),
                    namespace_internals: "cxx_qt::cxx_qt_first_object".to_owned(),
                    blocks: GeneratedCppQObjectBlocks {
                        forward_declares: vec![],
                        base_classes: vec!["QStringListModel".to_owned()],
                        includes: {
                          let mut includes = BTreeSet::<String>::default();
                          includes.insert("#include <test>".to_owned());
                          includes
                        },
                        metaobjects: vec![
                            "Q_PROPERTY(int longPropertyNameThatWrapsInClangFormat READ count WRITE setCount NOTIFY countChanged)"
                                .to_owned(),
                        ],
                        methods: vec![CppFragment::Pair {
                            header: "int count() const;".to_owned(),
                            source: indoc! {r#"
                                    int
                                    FirstObject::count() const
                                    {
                                      // getter
                                    }
                                "#}
                            .to_owned(),
                        },
                        CppFragment::Pair {
                            header: "Q_SLOT void setCount(int count);".to_owned(),
                            source: indoc! {r#"
                                    void
                                    FirstObject::setCount(int count) const
                                    {
                                      // setter
                                    }
                                "#}
                            .to_owned(),
                        },
                        CppFragment::Header("Q_SIGNAL void countChanged();".to_owned()),
                        ],
                        private_methods: vec![],
                    }
                },
                GeneratedCppQObject {
                    ident: "SecondObject".to_owned(),
                    rust_ident: "SecondObjectRust".to_owned(),
                    namespace_internals: "cxx_qt::cxx_qt_second_object".to_owned(),
                    blocks: GeneratedCppQObjectBlocks {
                        forward_declares: vec![],
                        base_classes: vec!["QStringListModel".to_owned()],
                        includes: {
                          let mut includes = BTreeSet::<String>::default();
                          includes.insert("#include <test>".to_owned());
                          includes
                        },
                        metaobjects: vec![
                            "Q_PROPERTY(int count READ count WRITE setCount NOTIFY countChanged)"
                                .to_owned(),
                        ],
                        methods: vec![CppFragment::Pair {
                            header: "int count() const;".to_owned(),
                            source: indoc! {r#"
                                    int
                                    SecondObject::count() const
                                    {
                                      // getter
                                    }
                                "#}
                            .to_owned(),
                        },
                        CppFragment::Pair {
                            header: "Q_SLOT void setCount(int count);".to_owned(),
                            source: indoc! {r#"
                                void
                                SecondObject::setCount(int count) const
                                {
                                  // setter
                                }
                                "#}
                            .to_owned(),
                        },
                        CppFragment::Header("Q_SIGNAL void countChanged();".to_owned()),
                        ],
                        private_methods: vec![
                            CppFragment::Pair{
                                header: "void privateMethod() const;".to_owned(),
                                source: indoc! {r#"
                                    void SecondObject::privateMethod() const {
                                        // private method
                                    }
                                    "#}.to_owned(),
                            }]
                    },
                }
            ]
        }
    }

    /// Helper to create a GeneratedCppBlocks with no namespace for testing
    pub fn create_generated_cpp_no_namespace() -> GeneratedCppBlocks {
        let mut generated = create_generated_cpp();
        generated.namespace = "".to_owned();
        generated.qobjects[0].namespace_internals = "cxx_qt_my_object".to_owned();
        generated
    }

    /// Helper for the expected header
    pub fn expected_header() -> &'static str {
        indoc! {r#"
        #pragma once

        #include <test>

        namespace cxx_qt::my_object {
        class MyObject;

        } // namespace cxx_qt::my_object

        #include "cxx-qt-gen/cxx_file_stem.cxx.h"

        namespace cxx_qt::my_object {
        class MyObject : public QStringListModel
        {
          Q_OBJECT
          Q_PROPERTY(int count READ count WRITE setCount NOTIFY countChanged)
          Q_PROPERTY(bool longPropertyNameThatWrapsInClangFormat READ getToggle WRITE setToggle NOTIFY toggleChanged)

        public:
          virtual ~MyObject() = default;

        public:
          int count() const;
          bool toggle() const;
          Q_INVOKABLE void invokable();
          void cppMethod();
          Q_SLOT void setCount(int count);
          Q_SLOT void setToggle(bool toggle);
          Q_SIGNAL void countChanged();
          Q_SIGNAL void toggleChanged();

        private:
          void privateMethod() const;
          void privateMethod();

        };

        static_assert(::std::is_base_of<QObject, MyObject>::value, "MyObject must inherit from QObject");
        } // namespace cxx_qt::my_object

        Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)

        "#}
    }

    /// Helper for the expected header with multiple QObjects
    pub fn expected_header_multi_qobjects() -> &'static str {
        indoc! {r#"
        #pragma once

        #include <test>

        namespace cxx_qt {
        class FirstObject;

        } // namespace cxx_qt

        namespace cxx_qt {
        class SecondObject;

        } // namespace cxx_qt

        #include "cxx-qt-gen/cxx_file_stem.cxx.h"

        namespace cxx_qt {
        class FirstObject : public QStringListModel
        {
          Q_OBJECT
          Q_PROPERTY(int longPropertyNameThatWrapsInClangFormat READ count WRITE setCount NOTIFY countChanged)

        public:
          virtual ~FirstObject() = default;

        public:
          int count() const;
          Q_SLOT void setCount(int count);
          Q_SIGNAL void countChanged();


        };

        static_assert(::std::is_base_of<QObject, FirstObject>::value, "FirstObject must inherit from QObject");
        } // namespace cxx_qt

        Q_DECLARE_METATYPE(cxx_qt::FirstObject*)

        namespace cxx_qt {
        class SecondObject : public QStringListModel
        {
          Q_OBJECT
          Q_PROPERTY(int count READ count WRITE setCount NOTIFY countChanged)

        public:
          virtual ~SecondObject() = default;

        public:
          int count() const;
          Q_SLOT void setCount(int count);
          Q_SIGNAL void countChanged();

        private:
          void privateMethod() const;

        };

        static_assert(::std::is_base_of<QObject, SecondObject>::value, "SecondObject must inherit from QObject");
        } // namespace cxx_qt

        Q_DECLARE_METATYPE(cxx_qt::SecondObject*)

        "#}
    }

    /// Helper for the expected header with no namespace
    pub fn expected_header_no_namespace() -> &'static str {
        indoc! {r#"
        #pragma once

        #include <test>


        class MyObject;



        #include "cxx-qt-gen/cxx_file_stem.cxx.h"


        class MyObject : public QStringListModel
        {
          Q_OBJECT
          Q_PROPERTY(int count READ count WRITE setCount NOTIFY countChanged)
          Q_PROPERTY(bool longPropertyNameThatWrapsInClangFormat READ getToggle WRITE setToggle NOTIFY toggleChanged)

        public:
          virtual ~MyObject() = default;

        public:
          int count() const;
          bool toggle() const;
          Q_INVOKABLE void invokable();
          void cppMethod();
          Q_SLOT void setCount(int count);
          Q_SLOT void setToggle(bool toggle);
          Q_SIGNAL void countChanged();
          Q_SIGNAL void toggleChanged();

        private:
          void privateMethod() const;
          void privateMethod();

        };

        static_assert(::std::is_base_of<QObject, MyObject>::value, "MyObject must inherit from QObject");


        Q_DECLARE_METATYPE(MyObject*)

        "#}
    }

    /// Helper for the expected source
    pub fn expected_source() -> &'static str {
        indoc! {r#"
        #include "cxx-qt-gen/cxx_file_stem.cxxqt.h"

        namespace cxx_qt::my_object {

        int
        MyObject::count() const
        {
          // getter
        }

        bool
        MyObject::toggle() const
        {
          // getter
        }

        void
        MyObject::invokable()
        {
          // invokable method
        }

        void
        MyObject::cppMethod()
        {
          // cpp method
        }

        void
        MyObject::setCount(int count) const
        {
          // setter
        }

        void
        MyObject::setToggle(bool toggle) const
        {
          // setter
        }

        void MyObject::privateMethod() const {
            // private method
        }

        void MyObject::privateMethod()
        {
            // non-const private method
        }

        } // namespace cxx_qt::my_object

        "#}
    }

    /// Helper for the expected source with multiple QObjects
    pub fn expected_source_multi_qobjects() -> &'static str {
        indoc! {r#"
        #include "cxx-qt-gen/cxx_file_stem.cxxqt.h"

        namespace cxx_qt {

        int
        FirstObject::count() const
        {
          // getter
        }

        void
        FirstObject::setCount(int count) const
        {
          // setter
        }

        } // namespace cxx_qt

        namespace cxx_qt {

        int
        SecondObject::count() const
        {
          // getter
        }

        void
        SecondObject::setCount(int count) const
        {
          // setter
        }

        void SecondObject::privateMethod() const {
            // private method
        }

        } // namespace cxx_qt

        "#}
    }

    /// Helper for the expected source with no namespace
    pub fn expected_source_no_namespace() -> &'static str {
        indoc! {r#"
        #include "cxx-qt-gen/cxx_file_stem.cxxqt.h"



        int
        MyObject::count() const
        {
          // getter
        }

        bool
        MyObject::toggle() const
        {
          // getter
        }

        void
        MyObject::invokable()
        {
          // invokable method
        }

        void
        MyObject::cppMethod()
        {
          // cpp method
        }

        void
        MyObject::setCount(int count) const
        {
          // setter
        }

        void
        MyObject::setToggle(bool toggle) const
        {
          // setter
        }

        void MyObject::privateMethod() const {
            // private method
        }

        void MyObject::privateMethod()
        {
            // non-const private method
        }



        "#}
    }

    #[test]
    fn test_write_cpp() {
        let generated = create_generated_cpp();
        let (header, source) = if let CppFragment::Pair { header, source } = write_cpp(&generated) {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(header, format_cpp(expected_header()));
        assert_str_eq!(source, format_cpp(expected_source()));
    }

    #[test]
    fn test_write_cpp_multi_qobjects() {
        let generated = create_generated_cpp_multi_qobjects();
        let (header, source) = if let CppFragment::Pair { header, source } = write_cpp(&generated) {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(header, format_cpp(expected_header_multi_qobjects()));
        assert_str_eq!(source, format_cpp(expected_source_multi_qobjects()));
    }

    #[test]
    fn test_write_cpp_no_namespace() {
        let generated = create_generated_cpp_no_namespace();
        let (header, source) = if let CppFragment::Pair { header, source } = write_cpp(&generated) {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(header, format_cpp(expected_header_no_namespace()));
        assert_str_eq!(source, format_cpp(expected_source_no_namespace()));
    }
}
