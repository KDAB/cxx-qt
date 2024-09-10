// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod header;
pub mod source;

use crate::generator::cpp::{fragment::CppFragment, GeneratedCppBlocks};
use clang_format::{clang_format_with_style, ClangFormatStyle};
use header::write_cpp_header;
use indoc::formatdoc;
use source::write_cpp_source;

/// Surround the given C++ code with the namespace if it is not empty
pub fn namespaced(namespace: &str, cpp_code: &str) -> String {
    if namespace.is_empty() {
        cpp_code.to_owned()
    } else {
        formatdoc! {r#"
            namespace {namespace} {{
            {cpp_code}
            }} // namespace {namespace}
            "# }
    }
}

/// For a given GeneratedCppBlocks write this into a C++ header and source pair
pub fn write_cpp(generated: &GeneratedCppBlocks, include_path: &str) -> CppFragment {
    let header = write_cpp_header(generated, include_path);
    let source = write_cpp_source(generated, include_path);

    CppFragment::Pair {
        header: clang_format_with_style(&header, &ClangFormatStyle::File).unwrap_or(header),
        source: clang_format_with_style(&source, &ClangFormatStyle::File).unwrap_or(source),
    }
}
/// Extract the header from a given CppFragment
pub fn pair_as_header(pair: &CppFragment) -> Option<String> {
    match pair {
        CppFragment::Pair { header, source: _ } => Some(header.clone()),
        CppFragment::Header(header) => Some(header.clone()),
        CppFragment::Source(_) => None,
    }
}

/// Extract the source from a given CppFragment
pub fn pair_as_source(pair: &CppFragment) -> Option<String> {
    match pair {
        CppFragment::Pair { header: _, source } => Some(source.clone()),
        CppFragment::Header(_) => None,
        CppFragment::Source(source) => Some(source.clone()),
    }
}

pub fn extract_extern_qt(
    generated: &GeneratedCppBlocks,
    mut filter_fn: impl FnMut(&CppFragment) -> Option<String>,
) -> String {
    generated
        .extern_cxx_qt
        .iter()
        .flat_map(|block| {
            block
                .fragments
                .iter()
                .filter_map(&mut filter_fn)
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    use crate::generator::cpp::property::tests::require_pair;
    use crate::{
        generator::cpp::qobject::{GeneratedCppQObject, GeneratedCppQObjectBlocks},
        naming::Name,
        tests::format_cpp,
    };
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;

    pub fn create_generated_cpp() -> GeneratedCppBlocks {
        create_generated_cpp_with_namespace(Some("cxx_qt::my_object"))
    }

    /// Helper to create a GeneratedCppBlocks for testing
    pub fn create_generated_cpp_with_namespace(namespace: Option<&str>) -> GeneratedCppBlocks {
        GeneratedCppBlocks {
            forward_declares: vec![],
            includes: BTreeSet::default(),
            extern_cxx_qt: vec![],
            qobjects: vec![
                GeneratedCppQObject {
                    name: if let Some(namespace) = namespace {
                        Name::mock_namespaced("MyObject", namespace)
                    } else {
                        Name::mock("MyObject")
                    },
                    rust_struct: Name::mock("MyObjectRust"),
                    namespace_internals: if let Some(namespace) = namespace {
                        format!("{namespace}::cxx_qt_my_object")
                    } else {
                        "cxx_qt_my_object".to_owned()
                    },
                    has_qobject_macro: true,
                    blocks: GeneratedCppQObjectBlocks {
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
                            }],
                            ..Default::default()
                    }
                }
            ],
        }
    }

    /// Helper to create a GeneratedCppBlocks for testing with multiple qobjects
    pub fn create_generated_cpp_multi_qobjects() -> GeneratedCppBlocks {
        GeneratedCppBlocks {
            forward_declares: vec![],
            includes: BTreeSet::default(),
            extern_cxx_qt: vec![],
            qobjects: vec![
                GeneratedCppQObject {
                    name: Name::mock_namespaced("FirstObject", "cxx_qt"),
                    rust_struct: Name::mock("FirstObjectRust"),
                    namespace_internals: "cxx_qt::cxx_qt_first_object".to_owned(),
                    has_qobject_macro: true,
                    blocks: GeneratedCppQObjectBlocks {
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
                        ..Default::default()
                    }
                },
                GeneratedCppQObject {
                    name: Name::mock_namespaced("SecondObject",  "cxx_qt"),
                    rust_struct: Name::mock("SecondObjectRust"),
                    namespace_internals: "cxx_qt::cxx_qt_second_object".to_owned(),
                    has_qobject_macro: true,
                    blocks: GeneratedCppQObjectBlocks {
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
                            }],
                        ..Default::default()
                    },
                }
            ]
        }
    }

    /// Helper to create a GeneratedCppBlocks with no namespace for testing
    pub fn create_generated_cpp_no_namespace() -> GeneratedCppBlocks {
        create_generated_cpp_with_namespace(None)
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
        public:
          Q_PROPERTY(int count READ count WRITE setCount NOTIFY countChanged)
          Q_PROPERTY(bool longPropertyNameThatWrapsInClangFormat READ getToggle WRITE setToggle NOTIFY toggleChanged)

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
        public:
          Q_PROPERTY(int longPropertyNameThatWrapsInClangFormat READ count WRITE setCount NOTIFY countChanged)

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
        public:
          Q_PROPERTY(int count READ count WRITE setCount NOTIFY countChanged)

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
        public:
          Q_PROPERTY(int count READ count WRITE setCount NOTIFY countChanged)
          Q_PROPERTY(bool longPropertyNameThatWrapsInClangFormat READ getToggle WRITE setToggle NOTIFY toggleChanged)

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
    fn namespacing() {
        let cpp_code = "// My C++ Code\n  // with multi-line";

        let namespaced_code = namespaced("my_namespace", cpp_code);

        assert_str_eq!(
            indoc! {r#"
            namespace my_namespace {
            // My C++ Code
              // with multi-line
            } // namespace my_namespace
            "#
            },
            namespaced_code
        );
    }

    #[test]
    fn namespacing_with_empty_namespace() {
        let cpp_code = indoc! {r#"
            // My C++ Code
            "#};
        let namespaced_code = namespaced("", cpp_code);
        assert_str_eq!(cpp_code, namespaced_code);
    }

    #[test]
    fn test_write_cpp() {
        let generated = create_generated_cpp();
        let (header, source) =
            require_pair(&write_cpp(&generated, "cxx-qt-gen/cxx_file_stem")).unwrap();
        assert_str_eq!(header, format_cpp(expected_header()));
        assert_str_eq!(source, format_cpp(expected_source()));
    }

    #[test]
    fn test_write_cpp_multi_qobjects() {
        let generated = create_generated_cpp_multi_qobjects();
        let (header, source) =
            require_pair(&write_cpp(&generated, "cxx-qt-gen/cxx_file_stem")).unwrap();
        assert_str_eq!(header, format_cpp(expected_header_multi_qobjects()));
        assert_str_eq!(source, format_cpp(expected_source_multi_qobjects()));
    }

    #[test]
    fn test_write_cpp_no_namespace() {
        let generated = create_generated_cpp_no_namespace();
        let (header, source) =
            require_pair(&write_cpp(&generated, "cxx-qt-gen/cxx_file_stem")).unwrap();
        assert_str_eq!(header, format_cpp(expected_header_no_namespace()));
        assert_str_eq!(source, format_cpp(expected_source_no_namespace()));
    }
}
