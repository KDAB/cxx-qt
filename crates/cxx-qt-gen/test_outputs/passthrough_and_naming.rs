#[cxx::bridge(namespace = "cxx_qt::multi_object")]
#[attrA]
#[attrB]
pub mod ffi {
    const MAX: u16 = 65535;
    enum Event {
        MyEvent,
    }
    extern crate serde;
    fn do_something() {
        println!("I am a free function");
    }
    extern "C" {}
    #[namespace = "namespace"]
    extern "C" {}
    #[namespace = "namespace"]
    #[custom_attr = "test"]
    extern "C" {}
    unsafe extern "C++" {}
    #[namespace = "namespace"]
    unsafe extern "C++" {}
    #[namespace = "namespace"]
    #[custom_attr = "test"]
    unsafe extern "C++" {}
    macro_rules! macro1 {
        () => {
            0
        };
    }
    macro macro2() {
        0
    }
    mod m {}
    static BIKE: Event = Event::MyEvent;
    pub trait CustomTrait {
        fn method();
    }
    pub trait SharableIterator = CustomTrait + Sync;
    type Result<T> = std::result::Result<T, Event>;
    union Foo<A, B> {
        x: A,
        y: B,
    }
    unsafe extern "C++" {
        include ! (< QtCore / QStringListModel >);
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
        include!("cxx-qt-gen/multi_object.cxxqt.h");
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
        #[cxx_name = "getPropertyNameWrapper"]
        unsafe fn property_name<'a>(self: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPropertyNameWrapper"]
        fn set_property_name(self: Pin<&mut MyObject>, value: i32);
    }
    unsafe extern "C++" {
        #[doc = "Notify for the Q_PROPERTY"]
        #[rust_name = "property_name_changed"]
        fn propertyNameChanged(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "propertyNameChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_property_name_changed"]
        fn propertyNameChangedConnect(
            self: Pin<&mut MyObject>,
            func: fn(Pin<&mut MyObject>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    extern "Rust" {
        #[doc(hidden)]
        #[cxx_name = "invokableNameWrapper"]
        fn invokable_name(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[rust_name = "ready"]
        fn ready(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "ready"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_ready"]
        fn readyConnect(
            self: Pin<&mut MyObject>,
            func: fn(Pin<&mut MyObject>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::multi_object::cxx_qt_my_object"]
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
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "SecondObjectRust"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        type SecondObject;
    }
    extern "Rust" {
        type SecondObjectRust;
    }
    extern "Rust" {
        #[cxx_name = "getPropertyNameWrapper"]
        unsafe fn property_name<'a>(self: &'a SecondObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPropertyNameWrapper"]
        fn set_property_name(self: Pin<&mut SecondObject>, value: i32);
    }
    unsafe extern "C++" {
        #[doc = "Notify for the Q_PROPERTY"]
        #[rust_name = "property_name_changed"]
        fn propertyNameChanged(self: Pin<&mut SecondObject>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "propertyNameChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_property_name_changed"]
        fn propertyNameChangedConnect(
            self: Pin<&mut SecondObject>,
            func: fn(Pin<&mut SecondObject>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    extern "Rust" {
        #[doc(hidden)]
        #[cxx_name = "invokableNameWrapper"]
        fn invokable_name(self: Pin<&mut SecondObject>);
    }
    unsafe extern "C++" {
        #[my_attribute]
        #[rust_name = "ready"]
        fn ready(self: Pin<&mut SecondObject>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "ready"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_ready"]
        fn readyConnect(
            self: Pin<&mut SecondObject>,
            func: fn(Pin<&mut SecondObject>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::multi_object::cxx_qt_second_object"]
        fn create_rs_second_object_rust() -> Box<SecondObjectRust>;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRust"]
        #[doc(hidden)]
        fn cxx_qt_ffi_rust(self: &SecondObject) -> &SecondObjectRust;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        #[doc(hidden)]
        fn cxx_qt_ffi_rust_mut(self: Pin<&mut SecondObject>) -> Pin<&mut SecondObjectRust>;
    }
}
use super::MyTrait;
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "property_name"]
    pub fn property_name(&self) -> &i32 {
        &self.property_name
    }
}
impl ffi::MyObject {
    #[doc = "Setter for the Q_PROPERTY "]
    #[doc = "property_name"]
    pub fn set_property_name(mut self: core::pin::Pin<&mut Self>, value: i32) {
        use cxx_qt::CxxQtType;
        if self.property_name == value {
            return;
        }
        self.as_mut().rust_mut().property_name = value;
        self.as_mut().property_name_changed();
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "propertyNameChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    #[must_use]
    pub fn on_property_name_changed(
        self: core::pin::Pin<&mut ffi::MyObject>,
        func: fn(core::pin::Pin<&mut ffi::MyObject>),
    ) -> cxx_qt_lib::QMetaObjectConnection {
        self.connect_property_name_changed(func, cxx_qt_lib::ConnectionType::AutoConnection)
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "ready"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    #[must_use]
    pub fn on_ready(
        self: core::pin::Pin<&mut ffi::MyObject>,
        func: fn(core::pin::Pin<&mut ffi::MyObject>),
    ) -> cxx_qt_lib::QMetaObjectConnection {
        self.connect_ready(func, cxx_qt_lib::ConnectionType::AutoConnection)
    }
}
impl cxx_qt::Locking for ffi::MyObject {}
#[doc(hidden)]
pub fn create_rs_my_object_rust() -> std::boxed::Box<MyObjectRust> {
    std::boxed::Box::new(core::default::Default::default())
}
impl core::ops::Deref for ffi::MyObject {
    type Target = MyObjectRust;
    fn deref(&self) -> &Self::Target {
        self.cxx_qt_ffi_rust()
    }
}
impl cxx_qt::CxxQtType for ffi::MyObject {
    type Rust = MyObjectRust;
    fn rust(&self) -> &Self::Rust {
        self.cxx_qt_ffi_rust()
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        self.cxx_qt_ffi_rust_mut()
    }
}
impl ffi::SecondObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "property_name"]
    pub fn property_name(&self) -> &i32 {
        &self.property_name
    }
}
impl ffi::SecondObject {
    #[doc = "Setter for the Q_PROPERTY "]
    #[doc = "property_name"]
    pub fn set_property_name(mut self: core::pin::Pin<&mut Self>, value: i32) {
        use cxx_qt::CxxQtType;
        if self.property_name == value {
            return;
        }
        self.as_mut().rust_mut().property_name = value;
        self.as_mut().property_name_changed();
    }
}
impl ffi::SecondObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "propertyNameChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    #[must_use]
    pub fn on_property_name_changed(
        self: core::pin::Pin<&mut ffi::SecondObject>,
        func: fn(core::pin::Pin<&mut ffi::SecondObject>),
    ) -> cxx_qt_lib::QMetaObjectConnection {
        self.connect_property_name_changed(func, cxx_qt_lib::ConnectionType::AutoConnection)
    }
}
impl ffi::SecondObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "ready"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    #[must_use]
    pub fn on_ready(
        self: core::pin::Pin<&mut ffi::SecondObject>,
        func: fn(core::pin::Pin<&mut ffi::SecondObject>),
    ) -> cxx_qt_lib::QMetaObjectConnection {
        self.connect_ready(func, cxx_qt_lib::ConnectionType::AutoConnection)
    }
}
#[doc(hidden)]
pub fn create_rs_second_object_rust() -> std::boxed::Box<SecondObjectRust> {
    std::boxed::Box::new(core::default::Default::default())
}
impl core::ops::Deref for ffi::SecondObject {
    type Target = SecondObjectRust;
    fn deref(&self) -> &Self::Target {
        self.cxx_qt_ffi_rust()
    }
}
impl cxx_qt::CxxQtType for ffi::SecondObject {
    type Rust = SecondObjectRust;
    fn rust(&self) -> &Self::Rust {
        self.cxx_qt_ffi_rust()
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        self.cxx_qt_ffi_rust_mut()
    }
}
