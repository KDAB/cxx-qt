#[cxx::bridge(namespace = "cxx_qt::my_object")]
#[allow(unused_unsafe)]
mod ffi {
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
    #[repr(i32)]
    #[namespace = "cxx_qt::my_object"]
    enum MyEnum {
        A,
    }
    extern "C++" {
        #[namespace = "cxx_qt::my_object"]
        type MyEnum;
    }
    #[repr(i32)]
    #[namespace = "my_namespace"]
    enum MyOtherEnum {
        X,
        Y,
        Z,
    }
    extern "C++" {
        #[namespace = "my_namespace"]
        type MyOtherEnum;
    }
    #[repr(i32)]
    #[namespace = "cxx_qt::my_object"]
    enum MyNamespacedEnum {
        A,
        B,
        C,
    }
    extern "C++" {
        #[namespace = "cxx_qt::my_object"]
        type MyNamespacedEnum;
    }
    #[repr(i32)]
    #[namespace = "other_namespace"]
    enum MyOtherNamespacedEnum {
        Variant1,
        Variant2,
    }
    extern "C++" {
        #[namespace = "other_namespace"]
        type MyOtherNamespacedEnum;
    }
    #[repr(i32)]
    #[namespace = "cxx_qt::my_object"]
    enum MyRenamedEnum {
        A,
        B,
        C,
    }
    extern "C++" {
        #[namespace = "cxx_qt::my_object"]
        type MyRenamedEnum;
    }
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObjectRust"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        #[namespace = "cxx_qt::my_object"]
        type MyObject;
    }
    extern "Rust" {
        #[namespace = "cxx_qt::my_object"]
        type MyObjectRust;
    }
    extern "Rust" {
        #[cxx_name = "my_invokable"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        unsafe fn my_invokable(self: &MyObject, qenum: MyEnum, other_qenum: MyOtherEnum);
    }
    extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "upcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_MyObject_upcastPtr(thiz: *const MyObject) -> *const QObject;
        #[doc(hidden)]
        #[cxx_name = "downcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_MyObject_downcastPtr(base: *const QObject) -> *const MyObject;
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
        fn create_rs_MyObjectRust() -> Box<MyObjectRust>;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRust"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_MyObject_unsafeRust(outer: &MyObject) -> &MyObjectRust;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRustMut"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_MyObject_unsafeRustMut(outer: Pin<&mut MyObject>) -> Pin<&mut MyObjectRust>;
    }
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "InternalObject"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        #[namespace = "cxx_qt::my_object"]
        #[doc = "\n\nNote: The C++ name of this QObject is: "]
        #[doc = "CxxName"]
        #[cxx_name = "CxxName"]
        type MyRenamedObject;
    }
    extern "Rust" {
        #[namespace = "cxx_qt::my_object"]
        type InternalObject;
    }
    extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "upcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_CxxName_upcastPtr(thiz: *const MyRenamedObject) -> *const QObject;
        #[doc(hidden)]
        #[cxx_name = "downcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_CxxName_downcastPtr(base: *const QObject) -> *const MyRenamedObject;
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::my_object::cxx_qt_MyRenamedObject"]
        fn create_rs_InternalObject() -> Box<InternalObject>;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRust"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_CxxName_unsafeRust(outer: &MyRenamedObject) -> &InternalObject;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRustMut"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_CxxName_unsafeRustMut(
            outer: Pin<&mut MyRenamedObject>,
        ) -> Pin<&mut InternalObject>;
    }
    extern "C++" {
        #[doc(hidden)]
        #[namespace = ""]
        type QObject = cxx_qt::QObject;
    }
}
impl ::cxx_qt::Upcast<::cxx_qt::QObject> for ffi::MyObject {
    unsafe fn upcast_ptr(this: *const Self) -> *const ::cxx_qt::QObject {
        ffi::cxx_qt_ffi_MyObject_upcastPtr(this)
    }
    unsafe fn from_base_ptr(base: *const ::cxx_qt::QObject) -> *const Self {
        ffi::cxx_qt_ffi_MyObject_downcastPtr(base)
    }
}
#[doc(hidden)]
#[allow(clippy::unnecessary_box_returns)]
pub fn create_rs_MyObjectRust() -> std::boxed::Box<MyObjectRust> {
    std::boxed::Box::new(core::default::Default::default())
}
impl ::core::ops::Deref for ffi::MyObject {
    type Target = MyObjectRust;
    fn deref(&self) -> &Self::Target {
        ffi::cxx_qt_ffi_MyObject_unsafeRust(self)
    }
}
impl ::cxx_qt::CxxQtType for ffi::MyObject {
    type Rust = MyObjectRust;
    fn rust(&self) -> &Self::Rust {
        ffi::cxx_qt_ffi_MyObject_unsafeRust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        ffi::cxx_qt_ffi_MyObject_unsafeRustMut(self)
    }
}
impl ::cxx_qt::Upcast<::cxx_qt::QObject> for ffi::MyRenamedObject {
    unsafe fn upcast_ptr(this: *const Self) -> *const ::cxx_qt::QObject {
        ffi::cxx_qt_ffi_CxxName_upcastPtr(this)
    }
    unsafe fn from_base_ptr(base: *const ::cxx_qt::QObject) -> *const Self {
        ffi::cxx_qt_ffi_CxxName_downcastPtr(base)
    }
}
#[doc(hidden)]
#[allow(clippy::unnecessary_box_returns)]
pub fn create_rs_InternalObject() -> std::boxed::Box<InternalObject> {
    std::boxed::Box::new(core::default::Default::default())
}
impl ::core::ops::Deref for ffi::MyRenamedObject {
    type Target = InternalObject;
    fn deref(&self) -> &Self::Target {
        ffi::cxx_qt_ffi_CxxName_unsafeRust(self)
    }
}
impl ::cxx_qt::CxxQtType for ffi::MyRenamedObject {
    type Rust = InternalObject;
    fn rust(&self) -> &Self::Rust {
        ffi::cxx_qt_ffi_CxxName_unsafeRust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        ffi::cxx_qt_ffi_CxxName_unsafeRustMut(self)
    }
}
