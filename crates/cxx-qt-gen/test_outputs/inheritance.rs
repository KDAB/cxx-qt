#[cxx::bridge(namespace = "")]
mod inheritance {
    extern "C++" {
        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
    }
    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/qt.h");
        #[doc(hidden)]
        #[namespace = "Qt"]
        #[rust_name = "CxxQtConnectionType"]
        type ConnectionType = cxx_qt_lib::ConnectionType;
        include!("cxx-qt-lib/qmetaobjectconnection.h");
        #[doc(hidden)]
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "CxxQtQMetaObjectConnection"]
        type QMetaObjectConnection = cxx_qt_lib::QMetaObjectConnection;
    }
    unsafe extern "C++" {
        include!("cxx-qt-gen/inheritance.cxxqt.h");
    }
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObject"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        #[cxx_name = "MyObject"]
        type MyObjectQt;
    }
    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;
    }
    extern "Rust" {
        #[cxx_name = "dataWrapper"]
        fn data_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            _index: &QModelIndex,
            _role: i32,
        ) -> QVariant;
    }
    extern "Rust" {
        #[cxx_name = "hasChildrenWrapper"]
        fn has_children_wrapper(self: &MyObject, cpp: &MyObjectQt, _parent: &QModelIndex) -> bool;
    }
    unsafe extern "C++" {
        #[doc = " Inherited hasChildren from the base class"]
        #[cxx_name = "hasChildrenCxxQtInherit"]
        fn has_children_super(self: &MyObjectQt, parent: &QModelIndex) -> bool;
    }
    extern "C++" {
        #[doc = " Inherited fetchMore from the base class"]
        #[cxx_name = "fetchMoreCxxQtInherit"]
        unsafe fn fetch_more(self: Pin<&mut MyObjectQt>, index: &QModelIndex);
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt_my_object"]
        fn create_rs_my_object() -> Box<MyObject>;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRust"]
        #[doc(hidden)]
        fn cxx_qt_ffi_rust(self: &MyObjectQt) -> &MyObject;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        #[doc(hidden)]
        fn cxx_qt_ffi_rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }
}
use self::cxx_qt_inheritance::*;
#[doc = r" Internal CXX-Qt module, made public temporarily between API changes"]
pub mod cxx_qt_inheritance {
    use super::inheritance::*;
    use cxx_qt::CxxQtType;
    use std::pin::Pin;
    #[doc(hidden)]
    type UniquePtr<T> = cxx::UniquePtr<T>;
    #[derive(Default)]
    pub struct MyObject {
        data: Vec<i32>,
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn data_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            _index: &QModelIndex,
            _role: i32,
        ) -> QVariant {
            return cpp.data(_index, _role);
        }
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn has_children_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            _parent: &QModelIndex,
        ) -> bool {
            return cpp.has_children(_parent);
        }
    }
    impl cxx_qt::Locking for MyObjectQt {}
    #[doc = r" Generated CXX-Qt method which creates a boxed rust struct of a QObject"]
    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        core::default::Default::default()
    }
    impl cxx_qt::CxxQtType for MyObjectQt {
        type Rust = MyObject;
        fn rust(&self) -> &Self::Rust {
            self.cxx_qt_ffi_rust()
        }
        fn rust_mut(self: core::pin::Pin<&mut Self>) -> Pin<&mut Self::Rust> {
            self.cxx_qt_ffi_rust_mut()
        }
    }
    #[doc = r" Generated CXX-Qt module containing type alias to the C++ types of the QObjects"]
    pub mod qobject {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObject"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        pub type MyObject = super::MyObjectQt;
    }
}
