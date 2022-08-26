// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod header;
pub mod source;

use crate::generator::cpp::{fragment::CppFragmentPair, GeneratedCppBlocks};
use clang_format::clang_format;
use header::write_cpp_header;
use source::write_cpp_source;

/// For a given GeneratedCppBlocks write this into a C++ header and source pair
pub fn write_cpp(generated: &GeneratedCppBlocks) -> CppFragmentPair {
    let header = write_cpp_header(generated);
    let source = write_cpp_source(generated);

    CppFragmentPair {
        header: clang_format(&header).unwrap_or(header),
        source: clang_format(&source).unwrap_or(source),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;
    use pretty_assertions::assert_str_eq;

    /// Helper to create a GeneratedCppBlocks for testing
    pub fn create_generated_cpp() -> GeneratedCppBlocks {
        GeneratedCppBlocks {
            cxx_stem: "cxx_stem".to_owned(),
            ident: "MyObject".to_owned(),
            rust_ident: "MyObjectRust".to_owned(),
            cxx_qt_thread_ident: "MyObjectCxxQtThread".to_owned(),
            namespace: "cxx_qt::my_object".to_owned(),
            namespace_internals: "cxx_qt::my_object::cxx_qt_my_object".to_owned(),
            base_class: "QStringListModel".to_owned(),
            metaobjects: vec![
                "Q_PROPERTY(int count READ count WRITE setCount NOTIFY countChanged)".to_owned(),
                "Q_PROPERTY(bool longPropertyNameThatWrapsInClangFormat READ getToggle WRITE setToggle NOTIFY toggleChanged)"
                    .to_owned(),
            ],
            methods: vec![
                CppFragmentPair {
                    header: "int count() const;".to_owned(),
                    source: indoc! {r#"
                        int
                        MyObject::count() const
                        {
                          return m_count;
                        }
                    "#}
                    .to_owned(),
                },
                CppFragmentPair {
                    header: "bool toggle() const;".to_owned(),
                    source: indoc! {r#"
                        bool
                        MyObject::toggle() const
                        {
                          return m_count;
                        }
                    "#}
                    .to_owned(),
                },
                CppFragmentPair {
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
                CppFragmentPair {
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
            ],
            slots: vec![
                CppFragmentPair {
                    header: "void setCount(int count);".to_owned(),
                    source: indoc! {r#"
                        void
                        MyObject::setCount(int count) const
                        {
                          if (m_count != count) {
                            m_count = count;

                            Q_EMIT countChanged();
                          }
                        }
                    "#}
                    .to_owned(),
                },
                CppFragmentPair {
                    header: "void setToggle(bool toggle);".to_owned(),
                    source: indoc! {r#"
                        void
                        MyObject::setToggle(bool toggle) const
                        {
                          if (m_toggle != toggle) {
                            m_toggle = toggle;

                            Q_EMIT toggleChanged();
                          }
                        }
                    "#}
                    .to_owned(),
                },
            ],
            signals: vec![
                "void countChanged();".to_owned(),
                "void toggleChanged();".to_owned(),
            ],
            members: vec!["int m_count;".to_owned(), "bool m_toggle;".to_owned()],
        }
    }

    /// Helper to create a GeneratedCppBlocks with no namespace for testing
    pub fn create_generated_cpp_no_namespace() -> GeneratedCppBlocks {
        let mut generated = create_generated_cpp();
        generated.namespace = "".to_owned();
        generated.namespace_internals = "cxx_qt_my_object".to_owned();
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
        using MyObjectCxxQtThread = rust::cxxqtlib1::CxxQtThread<MyObject>;
        } // namespace cxx_qt::my_object

        #include "cxx-qt-gen/include/cxx_stem.cxx.h"

        namespace cxx_qt::my_object {
        class MyObject : public QStringListModel
        {
          Q_OBJECT
          Q_PROPERTY(int count READ count WRITE setCount NOTIFY countChanged)
          Q_PROPERTY(bool longPropertyNameThatWrapsInClangFormat READ getToggle WRITE setToggle NOTIFY toggleChanged)

        public:
          explicit MyObject(QObject* parent = nullptr);
          ~MyObject();
          const MyObjectRust& unsafeRust() const;
          MyObjectRust& unsafeRustMut();
          std::unique_ptr<MyObjectCxxQtThread> qtThread() const;

        public:
          int count() const;
          bool toggle() const;
          Q_INVOKABLE void invokable();
          void cppMethod();

        public Q_SLOTS:
          void setCount(int count);
          void setToggle(bool toggle);

        Q_SIGNALS:
          void countChanged();
          void toggleChanged();

        private:
          rust::Box<MyObjectRust> m_rustObj;
          std::shared_ptr<std::mutex> m_rustObjMutex;
          bool m_initialised = false;
          std::shared_ptr<rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>> m_cxxQtThreadObj;

          int m_count;
          bool m_toggle;
        };

        static_assert(std::is_base_of<QObject, MyObject>::value, "MyObject must inherit from QObject");
        } // namespace cxx_qt::my_object

        namespace cxx_qt::my_object::cxx_qt_my_object {
        std::unique_ptr<MyObject>
        newCppObject();
        } // namespace cxx_qt::my_object::cxx_qt_my_object

        Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)
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
        using MyObjectCxxQtThread = rust::cxxqtlib1::CxxQtThread<MyObject>;


        #include "cxx-qt-gen/include/cxx_stem.cxx.h"


        class MyObject : public QStringListModel
        {
          Q_OBJECT
          Q_PROPERTY(int count READ count WRITE setCount NOTIFY countChanged)
          Q_PROPERTY(bool longPropertyNameThatWrapsInClangFormat READ getToggle WRITE setToggle NOTIFY toggleChanged)

        public:
          explicit MyObject(QObject* parent = nullptr);
          ~MyObject();
          const MyObjectRust& unsafeRust() const;
          MyObjectRust& unsafeRustMut();
          std::unique_ptr<MyObjectCxxQtThread> qtThread() const;

        public:
          int count() const;
          bool toggle() const;
          Q_INVOKABLE void invokable();
          void cppMethod();

        public Q_SLOTS:
          void setCount(int count);
          void setToggle(bool toggle);

        Q_SIGNALS:
          void countChanged();
          void toggleChanged();

        private:
          rust::Box<MyObjectRust> m_rustObj;
          std::shared_ptr<std::mutex> m_rustObjMutex;
          bool m_initialised = false;
          std::shared_ptr<rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>> m_cxxQtThreadObj;

          int m_count;
          bool m_toggle;
        };

        static_assert(std::is_base_of<QObject, MyObject>::value, "MyObject must inherit from QObject");


        namespace cxx_qt_my_object {
        std::unique_ptr<MyObject>
        newCppObject();
        } // namespace cxx_qt_my_object

        Q_DECLARE_METATYPE(MyObject*)
        "#}
    }

    /// Helper for the expected header
    pub fn expected_source() -> &'static str {
        indoc! {r#"
        #include "cxx-qt-gen/include/cxx_stem.cxxqt.h"

        namespace cxx_qt::my_object {

        MyObject::MyObject(QObject* parent)
          : QStringListModel(parent)
          , m_rustObj(cxx_qt::my_object::cxx_qt_my_object::createRs())
          , m_rustObjMutex(std::make_shared<std::mutex>())
          , m_cxxQtThreadObj(std::make_shared<rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>(this))
        {
          cxx_qt::my_object::cxx_qt_my_object::initialiseCpp(*this);
          m_initialised = true;
        }

        MyObject::~MyObject()
        {
          const auto guard = std::unique_lock(m_cxxQtThreadObj->mutex);
          m_cxxQtThreadObj->ptr = nullptr;
        }

        const MyObjectRust&
        MyObject::unsafeRust() const
        {
          return *m_rustObj;
        }

        MyObjectRust&
        MyObject::unsafeRustMut()
        {
          return *m_rustObj;
        }

        std::unique_ptr<MyObjectCxxQtThread>
        MyObject::qtThread() const
        {
          return std::make_unique<MyObjectCxxQtThread>(m_cxxQtThreadObj, m_rustObjMutex);
        }

        int
        MyObject::count() const
        {
          return m_count;
        }

        bool
        MyObject::toggle() const
        {
          return m_count;
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
          if (m_count != count) {
            m_count = count;

            Q_EMIT countChanged();
          }
        }

        void
        MyObject::setToggle(bool toggle) const
        {
          if (m_toggle != toggle) {
            m_toggle = toggle;

            Q_EMIT toggleChanged();
          }
        }

        } // namespace cxx_qt::my_object

        namespace cxx_qt::my_object::cxx_qt_my_object {
        std::unique_ptr<MyObject>
        newCppObject()
        {
          return std::make_unique<MyObject>();
        }
        } // namespace cxx_qt::my_object::cxx_qt_my_object
        "#}
    }

    /// Helper for the expected header with no namespace
    pub fn expected_source_no_namespace() -> &'static str {
        indoc! {r#"
        #include "cxx-qt-gen/include/cxx_stem.cxxqt.h"



        MyObject::MyObject(QObject* parent)
          : QStringListModel(parent)
          , m_rustObj(cxx_qt_my_object::createRs())
          , m_rustObjMutex(std::make_shared<std::mutex>())
          , m_cxxQtThreadObj(std::make_shared<rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>(this))
        {
          cxx_qt_my_object::initialiseCpp(*this);
          m_initialised = true;
        }

        MyObject::~MyObject()
        {
          const auto guard = std::unique_lock(m_cxxQtThreadObj->mutex);
          m_cxxQtThreadObj->ptr = nullptr;
        }

        const MyObjectRust&
        MyObject::unsafeRust() const
        {
          return *m_rustObj;
        }

        MyObjectRust&
        MyObject::unsafeRustMut()
        {
          return *m_rustObj;
        }

        std::unique_ptr<MyObjectCxxQtThread>
        MyObject::qtThread() const
        {
          return std::make_unique<MyObjectCxxQtThread>(m_cxxQtThreadObj, m_rustObjMutex);
        }

        int
        MyObject::count() const
        {
          return m_count;
        }

        bool
        MyObject::toggle() const
        {
          return m_count;
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
          if (m_count != count) {
            m_count = count;

            Q_EMIT countChanged();
          }
        }

        void
        MyObject::setToggle(bool toggle) const
        {
          if (m_toggle != toggle) {
            m_toggle = toggle;

            Q_EMIT toggleChanged();
          }
        }



        namespace cxx_qt_my_object {
        std::unique_ptr<MyObject>
        newCppObject()
        {
          return std::make_unique<MyObject>();
        }
        } // namespace cxx_qt_my_object
        "#}
    }

    #[test]
    fn test_write_cpp() {
        let generated = create_generated_cpp();
        let result = write_cpp(&generated);
        assert_str_eq!(result.header, clang_format(expected_header()).unwrap());
        assert_str_eq!(result.source, clang_format(expected_source()).unwrap());
    }

    #[test]
    fn test_write_cpp_no_namespace() {
        let generated = create_generated_cpp_no_namespace();
        let result = write_cpp(&generated);
        assert_str_eq!(
            result.header,
            clang_format(expected_header_no_namespace()).unwrap()
        );
        assert_str_eq!(
            result.source,
            clang_format(expected_source_no_namespace()).unwrap()
        );
    }
}
