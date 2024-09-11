#[cxx::bridge(namespace = "")]
mod inheritance {
    extern "C++" {
        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
        type QAbstractItemModel;
    }
    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt/connection.h");
        #[doc(hidden)]
        #[namespace = "Qt"]
        #[rust_name = "CxxQtConnectionType"]
        #[allow(dead_code)]
        type ConnectionType = cxx_qt::ConnectionType;
        #[doc(hidden)]
        #[namespace = "rust::cxxqt1"]
        #[rust_name = "CxxQtQMetaObjectConnection"]
        #[allow(dead_code)]
        type QMetaObjectConnection = cxx_qt::QMetaObjectConnection;
    }
    unsafe extern "C++" {
        include!("directory/file_ident.cxxqt.h");
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
        #[cxx_name = "data"]
        #[doc(hidden)]
        fn data(self: &MyObject, _index: &QModelIndex, _role: i32) -> QVariant;
    }
    extern "Rust" {
        #[cxx_name = "hasChildren"]
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
        #[doc(hidden)]
        #[cxx_name = "unsafeRust"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_my_object_unsafe_rust(outer: &MyObject) -> &MyObjectRust;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRustMut"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_my_object_unsafe_rust_mut(
            outer: Pin<&mut MyObject>,
        ) -> Pin<&mut MyObjectRust>;
    }
}
impl cxx_qt::Upcast<inheritance::QAbstractItemModel> for inheritance::MyObject {}
#[doc(hidden)]
pub fn create_rs_my_object_rust() -> std::boxed::Box<MyObjectRust> {
    std::boxed::Box::new(core::default::Default::default())
}
impl ::core::ops::Deref for inheritance::MyObject {
    type Target = MyObjectRust;
    fn deref(&self) -> &Self::Target {
        inheritance::cxx_qt_ffi_my_object_unsafe_rust(self)
    }
}
impl ::cxx_qt::CxxQtType for inheritance::MyObject {
    type Rust = MyObjectRust;
    fn rust(&self) -> &Self::Rust {
        inheritance::cxx_qt_ffi_my_object_unsafe_rust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        inheritance::cxx_qt_ffi_my_object_unsafe_rust_mut(self)
    }
}
