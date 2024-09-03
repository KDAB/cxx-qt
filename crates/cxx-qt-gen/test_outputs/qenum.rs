#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
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
    #[repr(i32)]
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
        include!("cxx-qt-gen/ffi.cxxqt.h");
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
        type MyObjectRust;
    }
    extern "Rust" {
        #[cxx_name = "myInvokable"]
        #[doc(hidden)]
        fn my_invokable(self: &MyObject, qenum: MyEnum, other_qenum: MyOtherEnum);
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn create_rs_my_object_rust() -> Box<MyObjectRust>;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRust"]
        #[namespace = "rust::cxxqt1"]
        #[doc(hidden)]
        fn cxx_qt_ffi_my_object_rust(outer: &MyObject) -> &MyObjectRust;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        #[namespace = "rust::cxxqt1"]
        #[doc(hidden)]
        fn cxx_qt_ffi_my_object_rust_mut(outer: Pin<&mut MyObject>) -> Pin<&mut MyObjectRust>;
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
        type InternalObject;
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_renamed_object"]
        fn create_rs_internal_object() -> Box<InternalObject>;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRust"]
        #[namespace = "rust::cxxqt1"]
        #[doc(hidden)]
        fn cxx_qt_ffi_cxx_name_rust(outer: &MyRenamedObject) -> &InternalObject;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        #[namespace = "rust::cxxqt1"]
        #[doc(hidden)]
        fn cxx_qt_ffi_cxx_name_rust_mut(
            outer: Pin<&mut MyRenamedObject>,
        ) -> Pin<&mut InternalObject>;
    }
}
#[doc(hidden)]
pub fn create_rs_my_object_rust() -> std::boxed::Box<MyObjectRust> {
    std::boxed::Box::new(core::default::Default::default())
}
impl ::core::ops::Deref for ffi::MyObject {
    type Target = MyObjectRust;
    fn deref(&self) -> &Self::Target {
        ffi::cxx_qt_ffi_my_object_rust(self)
    }
}
impl ::cxx_qt::CxxQtType for ffi::MyObject {
    type Rust = MyObjectRust;
    fn rust(&self) -> &Self::Rust {
        ffi::cxx_qt_ffi_my_object_rust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        ffi::cxx_qt_ffi_my_object_rust_mut(self)
    }
}
#[doc(hidden)]
pub fn create_rs_internal_object() -> std::boxed::Box<InternalObject> {
    std::boxed::Box::new(core::default::Default::default())
}
impl ::core::ops::Deref for ffi::MyRenamedObject {
    type Target = InternalObject;
    fn deref(&self) -> &Self::Target {
        ffi::cxx_qt_ffi_cxx_name_rust(self)
    }
}
impl ::cxx_qt::CxxQtType for ffi::MyRenamedObject {
    type Rust = InternalObject;
    fn rust(&self) -> &Self::Rust {
        ffi::cxx_qt_ffi_cxx_name_rust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        ffi::cxx_qt_ffi_cxx_name_rust_mut(self)
    }
}
