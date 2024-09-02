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
        include!("cxx-qt/connection.h");
        #[doc(hidden)]
        #[namespace = "Qt"]
        #[rust_name = "CxxQtConnectionType"]
        type ConnectionType = cxx_qt::ConnectionType;
        #[doc(hidden)]
        #[namespace = "rust::cxxqt1"]
        #[rust_name = "CxxQtQMetaObjectConnection"]
        type QMetaObjectConnection = cxx_qt::QMetaObjectConnection;
    }
    unsafe extern "C++" {
        include!("cxx-qt-gen/inheritance.cxxqt.h");
    }
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObjectRust"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        type MyObject;
    }
    extern "Rust" {
        type MyObjectRust;
    }
    extern "Rust" {
        #[cxx_name = "dataWrapper"]
        #[doc(hidden)]
        fn data(self: &MyObject, _index: &QModelIndex, _role: i32) -> QVariant;
    }
    extern "Rust" {
        #[cxx_name = "hasChildrenWrapper"]
        #[doc(hidden)]
        fn has_children(self: &MyObject, _parent: &QModelIndex) -> bool;
    }
    unsafe extern "C++" {
        #[cxx_name = "hasChildrenCxxQtInherit"]
        #[doc = " Inherited hasChildren from the base class"]
        fn has_children_super(self: &MyObject, parent: &QModelIndex) -> bool;
    }
    extern "C++" {
        #[cxx_name = "fetchMoreCxxQtInherit"]
        #[doc = " Inherited fetchMore from the base class"]
        unsafe fn fetch_more(self: Pin<&mut MyObject>, index: &QModelIndex);
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt_my_object"]
        fn create_rs_my_object_rust() -> Box<MyObjectRust>;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRust"]
        #[doc(hidden)]
        fn cxx_qt_ffi_rust(self: &MyObject) -> &MyObjectRust;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        #[doc(hidden)]
        fn cxx_qt_ffi_rust_mut(self: Pin<&mut MyObject>) -> Pin<&mut MyObjectRust>;
    }
}
impl cxx_qt::Locking for inheritance::MyObject {}
#[doc(hidden)]
pub fn create_rs_my_object_rust() -> std::boxed::Box<MyObjectRust> {
    std::boxed::Box::new(core::default::Default::default())
}
impl core::ops::Deref for inheritance::MyObject {
    type Target = MyObjectRust;
    fn deref(&self) -> &Self::Target {
        self.cxx_qt_ffi_rust()
    }
}
impl cxx_qt::CxxQtType for inheritance::MyObject {
    type Rust = MyObjectRust;
    fn rust(&self) -> &Self::Rust {
        self.cxx_qt_ffi_rust()
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        self.cxx_qt_ffi_rust_mut()
    }
}
