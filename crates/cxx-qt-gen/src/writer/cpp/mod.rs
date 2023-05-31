// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod header;
pub mod source;

use crate::generator::cpp::{fragment::CppFragment, GeneratedCppBlocks};
use clang_format::clang_format;
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
        header: clang_format(&header).unwrap_or(header),
        source: clang_format(&source).unwrap_or(source),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::cpp::qobject::{GeneratedCppQObject, GeneratedCppQObjectBlocks};
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
                    base_class: "QStringListModel".to_owned(),
                    locking: true,
                    blocks: GeneratedCppQObjectBlocks {
                        deconstructors: vec![],
                        forward_declares: vec![],
                        members: vec![],
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
                    base_class: "QStringListModel".to_owned(),
                    locking: true,
                    blocks: GeneratedCppQObjectBlocks {
                        deconstructors: vec![],
                        forward_declares: vec![],
                        members: vec![],
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
                    }
                },
                GeneratedCppQObject {
                    ident: "SecondObject".to_owned(),
                    rust_ident: "SecondObjectRust".to_owned(),
                    namespace_internals: "cxx_qt::cxx_qt_second_object".to_owned(),
                    base_class: "QStringListModel".to_owned(),
                    locking: false,
                    blocks: GeneratedCppQObjectBlocks {
                        deconstructors: vec![],
                        forward_declares: vec![],
                        members: vec![],
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

        #include <memory>
        #include <mutex>

        namespace rust::cxxqtlib1 {
        template<typename T>
        class CxxQtThread;
        }

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
          explicit MyObject(QObject* parent = nullptr);
          ~MyObject();
          MyObjectRust const& unsafeRust() const;
          MyObjectRust& unsafeRustMut();

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
          ::rust::Box<MyObjectRust> m_rustObj;
          ::std::shared_ptr<::std::recursive_mutex> m_rustObjMutex;
        };

        static_assert(::std::is_base_of<QObject, MyObject>::value, "MyObject must inherit from QObject");
        } // namespace cxx_qt::my_object

        namespace cxx_qt::my_object::cxx_qt_my_object {
        ::std::unique_ptr<MyObject>
        newCppObject();
        } // namespace cxx_qt::my_object::cxx_qt_my_object

        Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)

        "#}
    }

    /// Helper for the expected header with multiple QObjects
    pub fn expected_header_multi_qobjects() -> &'static str {
        indoc! {r#"
        #pragma once

        #include <memory>
        #include <mutex>

        namespace rust::cxxqtlib1 {
        template<typename T>
        class CxxQtThread;
        }

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
          explicit FirstObject(QObject* parent = nullptr);
          ~FirstObject();
          FirstObjectRust const& unsafeRust() const;
          FirstObjectRust& unsafeRustMut();

        public:
          int count() const;
          Q_SLOT void setCount(int count);
          Q_SIGNAL void countChanged();

        private:
          ::rust::Box<FirstObjectRust> m_rustObj;
          ::std::shared_ptr<::std::recursive_mutex> m_rustObjMutex;
        };

        static_assert(::std::is_base_of<QObject, FirstObject>::value, "FirstObject must inherit from QObject");
        } // namespace cxx_qt

        namespace cxx_qt::cxx_qt_first_object {
        ::std::unique_ptr<FirstObject>
        newCppObject();
        } // namespace cxx_qt::cxx_qt_first_object

        Q_DECLARE_METATYPE(cxx_qt::FirstObject*)

        namespace cxx_qt {
        class SecondObject : public QStringListModel
        {
          Q_OBJECT
          Q_PROPERTY(int count READ count WRITE setCount NOTIFY countChanged)

        public:
          explicit SecondObject(QObject* parent = nullptr);
          ~SecondObject();
          SecondObjectRust const& unsafeRust() const;
          SecondObjectRust& unsafeRustMut();

        public:
          int count() const;
          Q_SLOT void setCount(int count);
          Q_SIGNAL void countChanged();

        private:
          ::rust::Box<SecondObjectRust> m_rustObj;
        };

        static_assert(::std::is_base_of<QObject, SecondObject>::value, "SecondObject must inherit from QObject");
        } // namespace cxx_qt

        namespace cxx_qt::cxx_qt_second_object {
        ::std::unique_ptr<SecondObject>
        newCppObject();
        } // namespace cxx_qt::cxx_qt_second_object

        Q_DECLARE_METATYPE(cxx_qt::SecondObject*)

        "#}
    }

    /// Helper for the expected header with no namespace
    pub fn expected_header_no_namespace() -> &'static str {
        indoc! {r#"
        #pragma once

        #include <memory>
        #include <mutex>

        namespace rust::cxxqtlib1 {
        template<typename T>
        class CxxQtThread;
        }


        class MyObject;



        #include "cxx-qt-gen/cxx_file_stem.cxx.h"


        class MyObject : public QStringListModel
        {
          Q_OBJECT
          Q_PROPERTY(int count READ count WRITE setCount NOTIFY countChanged)
          Q_PROPERTY(bool longPropertyNameThatWrapsInClangFormat READ getToggle WRITE setToggle NOTIFY toggleChanged)

        public:
          explicit MyObject(QObject* parent = nullptr);
          ~MyObject();
          MyObjectRust const& unsafeRust() const;
          MyObjectRust& unsafeRustMut();

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
          ::rust::Box<MyObjectRust> m_rustObj;
          ::std::shared_ptr<::std::recursive_mutex> m_rustObjMutex;
        };

        static_assert(::std::is_base_of<QObject, MyObject>::value, "MyObject must inherit from QObject");


        namespace cxx_qt_my_object {
        ::std::unique_ptr<MyObject>
        newCppObject();
        } // namespace cxx_qt_my_object

        Q_DECLARE_METATYPE(MyObject*)

        "#}
    }

    /// Helper for the expected source
    pub fn expected_source() -> &'static str {
        indoc! {r#"
        #include "cxx-qt-gen/cxx_file_stem.cxxqt.h"

        namespace cxx_qt::my_object {

        MyObject::MyObject(QObject* parent)
          : QStringListModel(parent)
          , m_rustObj(cxx_qt::my_object::cxx_qt_my_object::createRs())
          , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
        {
        }

        MyObject::~MyObject()
        {

        }

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

        } // namespace cxx_qt::my_object

        namespace cxx_qt::my_object::cxx_qt_my_object {
        ::std::unique_ptr<MyObject>
        newCppObject()
        {
          return ::std::make_unique<MyObject>();
        }
        } // namespace cxx_qt::my_object::cxx_qt_my_object

        "#}
    }

    /// Helper for the expected source with multiple QObjects
    pub fn expected_source_multi_qobjects() -> &'static str {
        indoc! {r#"
        #include "cxx-qt-gen/cxx_file_stem.cxxqt.h"

        namespace cxx_qt {

        FirstObject::FirstObject(QObject* parent)
          : QStringListModel(parent)
          , m_rustObj(cxx_qt::cxx_qt_first_object::createRs())
          , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
        {
        }

        FirstObject::~FirstObject()
        {

        }

        FirstObjectRust const&
        FirstObject::unsafeRust() const
        {
          return *m_rustObj;
        }

        FirstObjectRust&
        FirstObject::unsafeRustMut()
        {
          return *m_rustObj;
        }

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

        namespace cxx_qt::cxx_qt_first_object {
        ::std::unique_ptr<FirstObject>
        newCppObject()
        {
          return ::std::make_unique<FirstObject>();
        }
        } // namespace cxx_qt::cxx_qt_first_object

        namespace cxx_qt {

        SecondObject::SecondObject(QObject* parent)
          : QStringListModel(parent)
          , m_rustObj(cxx_qt::cxx_qt_second_object::createRs())
        {
        }

        SecondObject::~SecondObject()
        {

        }

        SecondObjectRust const&
        SecondObject::unsafeRust() const
        {
          return *m_rustObj;
        }

        SecondObjectRust&
        SecondObject::unsafeRustMut()
        {
          return *m_rustObj;
        }

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

        } // namespace cxx_qt

        namespace cxx_qt::cxx_qt_second_object {
        ::std::unique_ptr<SecondObject>
        newCppObject()
        {
          return ::std::make_unique<SecondObject>();
        }
        } // namespace cxx_qt::cxx_qt_second_object

        "#}
    }

    /// Helper for the expected header with no namespace
    pub fn expected_source_no_namespace() -> &'static str {
        indoc! {r#"
        #include "cxx-qt-gen/cxx_file_stem.cxxqt.h"



        MyObject::MyObject(QObject* parent)
          : QStringListModel(parent)
          , m_rustObj(cxx_qt_my_object::createRs())
          , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
        {
        }

        MyObject::~MyObject()
        {

        }

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



        namespace cxx_qt_my_object {
        ::std::unique_ptr<MyObject>
        newCppObject()
        {
          return ::std::make_unique<MyObject>();
        }
        } // namespace cxx_qt_my_object

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
        assert_str_eq!(header, clang_format(expected_header()).unwrap());
        assert_str_eq!(source, clang_format(expected_source()).unwrap());
    }

    #[test]
    fn test_write_cpp_multi_qobjects() {
        let generated = create_generated_cpp_multi_qobjects();
        let (header, source) = if let CppFragment::Pair { header, source } = write_cpp(&generated) {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(
            header,
            clang_format(expected_header_multi_qobjects()).unwrap()
        );
        assert_str_eq!(
            source,
            clang_format(expected_source_multi_qobjects()).unwrap()
        );
    }

    #[test]
    fn test_write_cpp_no_namespace() {
        let generated = create_generated_cpp_no_namespace();
        let (header, source) = if let CppFragment::Pair { header, source } = write_cpp(&generated) {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(
            header,
            clang_format(expected_header_no_namespace()).unwrap()
        );
        assert_str_eq!(
            source,
            clang_format(expected_source_no_namespace()).unwrap()
        );
    }
}
