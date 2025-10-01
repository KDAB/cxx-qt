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
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObjectRust"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/concepts/generated_qobject.html>"]
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
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/concepts/generated_qobject.html>"]
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
        #[allow(private_interfaces)]
        #[namespace = "cxx_qt::my_object"]
        type MyEnum = super::cxx_qt_private_qenum_MyEnum::MyEnum;
    }
    extern "C++" {
        #[allow(private_interfaces)]
        #[namespace = "my_namespace"]
        type MyOtherEnum = super::cxx_qt_private_qenum_MyOtherEnum::MyOtherEnum;
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
    extern "C++" {
        #[allow(private_interfaces)]
        #[namespace = "cxx_qt::my_object"]
        type MyRenamedEnum = super::cxx_qt_private_qenum_MyRenamedEnum::MyRenamedEnum;
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
mod cxx_qt_private_qenum_MyEnum {
    #[derive(PartialEq, Eq, Clone, Copy)]
    #[repr(transparent)]
    pub(super) struct MyEnum {
        #[allow(missing_docs)]
        pub repr: i32,
    }
    #[allow(non_upper_case_globals)]
    impl MyEnum {
        pub const A: MyEnum = MyEnum { repr: 0i32 };
    }
    #[automatically_derived]
    unsafe impl ::cxx::ExternType for MyEnum {
        type Id = ::cxx::type_id!("MyEnum");
        type Kind = ::cxx::kind::Trivial;
    }
}
mod cxx_qt_private_qenum_MyOtherEnum {
    #[derive(PartialEq, Eq, Clone, Copy)]
    #[repr(transparent)]
    pub(super) struct MyOtherEnum {
        #[allow(missing_docs)]
        pub repr: i32,
    }
    #[allow(non_upper_case_globals)]
    impl MyOtherEnum {
        pub const X: MyOtherEnum = MyOtherEnum { repr: 0i32 };
        pub const Y: MyOtherEnum = MyOtherEnum { repr: 1i32 };
        pub const Z: MyOtherEnum = MyOtherEnum { repr: 2i32 };
    }
    #[automatically_derived]
    unsafe impl ::cxx::ExternType for MyOtherEnum {
        type Id = ::cxx::type_id!("MyOtherEnum");
        type Kind = ::cxx::kind::Trivial;
    }
}
mod cxx_qt_private_qenum_MyRenamedEnum {
    #[derive(PartialEq, Eq, Clone, Copy)]
    #[repr(transparent)]
    pub(super) struct MyRenamedEnum {
        #[allow(missing_docs)]
        pub repr: i32,
    }
    #[allow(non_upper_case_globals)]
    impl MyRenamedEnum {
        pub const A: MyRenamedEnum = MyRenamedEnum { repr: 0i32 };
        pub const B: MyRenamedEnum = MyRenamedEnum { repr: 1i32 };
        pub const C: MyRenamedEnum = MyRenamedEnum { repr: 2i32 };
    }
    #[automatically_derived]
    unsafe impl ::cxx::ExternType for MyRenamedEnum {
        type Id = ::cxx::type_id!("MyRenamedEnum");
        type Kind = ::cxx::kind::Trivial;
    }
}
