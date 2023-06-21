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
        #[cxx_name = "getPropertyName"]
        unsafe fn property_name<'a>(self: &'a MyObjectRust, cpp: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPropertyName"]
        fn set_property_name(self: &mut MyObjectRust, cpp: Pin<&mut MyObject>, value: i32);
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
        #[cxx_name = "invokableNameWrapper"]
        fn invokable_name_wrapper(self: &mut MyObjectRust, cpp: Pin<&mut MyObject>);
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
        #[cxx_name = "getPropertyName"]
        unsafe fn property_name<'a>(self: &'a SecondObjectRust, cpp: &'a SecondObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPropertyName"]
        fn set_property_name(self: &mut SecondObjectRust, cpp: Pin<&mut SecondObject>, value: i32);
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
        #[cxx_name = "invokableNameWrapper"]
        fn invokable_name_wrapper(self: &mut SecondObjectRust, cpp: Pin<&mut SecondObject>);
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
pub use self::cxx_qt_ffi::*;
#[doc = r" Internal CXX-Qt module, made public temporarily between API changes"]
pub mod cxx_qt_ffi {
    use super::ffi::*;
    use cxx_qt::CxxQtType;
    use std::pin::Pin;
    #[doc(hidden)]
    type UniquePtr<T> = cxx::UniquePtr<T>;
    use super::MyTrait;
    type MyObjectRust = super::MyObjectRust;
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn property_name<'a>(&'a self, cpp: &'a MyObject) -> &'a i32 {
            cpp.property_name()
        }
    }
    impl MyObject {
        #[doc = "Getter for the Q_PROPERTY "]
        #[doc = "property_name"]
        pub fn property_name(&self) -> &i32 {
            &self.property_name
        }
    }
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn set_property_name(&mut self, cpp: Pin<&mut MyObject>, value: i32) {
            cpp.set_property_name(value);
        }
    }
    impl MyObject {
        #[doc = "Setter for the Q_PROPERTY "]
        #[doc = "property_name"]
        pub fn set_property_name(mut self: Pin<&mut Self>, value: i32) {
            if self.property_name == value {
                return;
            }
            self.as_mut().rust_mut().property_name = value;
            self.as_mut().property_name_changed();
        }
    }
    impl MyObject {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "propertyNameChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_property_name_changed(
            self: Pin<&mut MyObject>,
            func: fn(Pin<&mut MyObject>),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_property_name_changed(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn invokable_name_wrapper(self: &mut MyObjectRust, cpp: Pin<&mut MyObject>) {
            cpp.invokable_name();
        }
    }
    impl MyObject {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "ready"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_ready(
            self: Pin<&mut MyObject>,
            func: fn(Pin<&mut MyObject>),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_ready(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl cxx_qt::Locking for MyObject {}
    #[doc = r" Generated CXX-Qt method which creates a boxed rust struct of a QObject"]
    pub fn create_rs_my_object_rust() -> std::boxed::Box<MyObjectRust> {
        core::default::Default::default()
    }
    impl core::ops::Deref for MyObject {
        type Target = MyObjectRust;
        fn deref(&self) -> &Self::Target {
            self.cxx_qt_ffi_rust()
        }
    }
    impl cxx_qt::CxxQtType for MyObject {
        type Rust = MyObjectRust;
        fn rust(&self) -> &Self::Rust {
            self.cxx_qt_ffi_rust()
        }
        fn rust_mut(self: core::pin::Pin<&mut Self>) -> Pin<&mut Self::Rust> {
            self.cxx_qt_ffi_rust_mut()
        }
    }
    type SecondObjectRust = super::SecondObjectRust;
    impl SecondObjectRust {
        #[doc(hidden)]
        pub fn property_name<'a>(&'a self, cpp: &'a SecondObject) -> &'a i32 {
            cpp.property_name()
        }
    }
    impl SecondObject {
        #[doc = "Getter for the Q_PROPERTY "]
        #[doc = "property_name"]
        pub fn property_name(&self) -> &i32 {
            &self.property_name
        }
    }
    impl SecondObjectRust {
        #[doc(hidden)]
        pub fn set_property_name(&mut self, cpp: Pin<&mut SecondObject>, value: i32) {
            cpp.set_property_name(value);
        }
    }
    impl SecondObject {
        #[doc = "Setter for the Q_PROPERTY "]
        #[doc = "property_name"]
        pub fn set_property_name(mut self: Pin<&mut Self>, value: i32) {
            if self.property_name == value {
                return;
            }
            self.as_mut().rust_mut().property_name = value;
            self.as_mut().property_name_changed();
        }
    }
    impl SecondObject {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "propertyNameChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_property_name_changed(
            self: Pin<&mut SecondObject>,
            func: fn(Pin<&mut SecondObject>),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_property_name_changed(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl SecondObjectRust {
        #[doc(hidden)]
        pub fn invokable_name_wrapper(self: &mut SecondObjectRust, cpp: Pin<&mut SecondObject>) {
            cpp.invokable_name();
        }
    }
    impl SecondObject {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "ready"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_ready(
            self: Pin<&mut SecondObject>,
            func: fn(Pin<&mut SecondObject>),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_ready(func, CxxQtConnectionType::AutoConnection)
        }
    }
    #[doc = r" Generated CXX-Qt method which creates a boxed rust struct of a QObject"]
    pub fn create_rs_second_object_rust() -> std::boxed::Box<SecondObjectRust> {
        core::default::Default::default()
    }
    impl core::ops::Deref for SecondObject {
        type Target = SecondObjectRust;
        fn deref(&self) -> &Self::Target {
            self.cxx_qt_ffi_rust()
        }
    }
    impl cxx_qt::CxxQtType for SecondObject {
        type Rust = SecondObjectRust;
        fn rust(&self) -> &Self::Rust {
            self.cxx_qt_ffi_rust()
        }
        fn rust_mut(self: core::pin::Pin<&mut Self>) -> Pin<&mut Self::Rust> {
            self.cxx_qt_ffi_rust_mut()
        }
    }
}
