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
        #[cxx_name = "getPropertyName"]
        unsafe fn property_name<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPropertyName"]
        fn set_property_name(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i32);
    }
    unsafe extern "C++" {
        #[doc = "Notify for the Q_PROPERTY"]
        #[rust_name = "property_name_changed"]
        fn propertyNameChanged(self: Pin<&mut MyObjectQt>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "propertyNameChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_property_name_changed"]
        fn propertyNameChangedConnect(
            self: Pin<&mut MyObjectQt>,
            func: fn(Pin<&mut MyObjectQt>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    extern "Rust" {
        #[cxx_name = "invokableNameWrapper"]
        fn invokable_name_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>);
    }
    unsafe extern "C++" {
        #[rust_name = "ready"]
        fn ready(self: Pin<&mut MyObjectQt>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "ready"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_ready"]
        fn readyConnect(
            self: Pin<&mut MyObjectQt>,
            func: fn(Pin<&mut MyObjectQt>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
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
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::multi_object::cxx_qt_my_object"]
        fn create_rs_my_object() -> Box<MyObject>;
    }
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "SecondObject"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        #[cxx_name = "SecondObject"]
        type SecondObjectQt;
    }
    extern "Rust" {
        #[cxx_name = "SecondObjectRust"]
        type SecondObject;
    }
    extern "Rust" {
        #[cxx_name = "getPropertyName"]
        unsafe fn property_name<'a>(self: &'a SecondObject, cpp: &'a SecondObjectQt) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPropertyName"]
        fn set_property_name(self: &mut SecondObject, cpp: Pin<&mut SecondObjectQt>, value: i32);
    }
    unsafe extern "C++" {
        #[doc = "Notify for the Q_PROPERTY"]
        #[rust_name = "property_name_changed"]
        fn propertyNameChanged(self: Pin<&mut SecondObjectQt>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "propertyNameChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_property_name_changed"]
        fn propertyNameChangedConnect(
            self: Pin<&mut SecondObjectQt>,
            func: fn(Pin<&mut SecondObjectQt>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    extern "Rust" {
        #[cxx_name = "invokableNameWrapper"]
        fn invokable_name_wrapper(self: &mut SecondObject, cpp: Pin<&mut SecondObjectQt>);
    }
    unsafe extern "C++" {
        #[my_attribute]
        #[rust_name = "ready"]
        fn ready(self: Pin<&mut SecondObjectQt>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "ready"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_ready"]
        fn readyConnect(
            self: Pin<&mut SecondObjectQt>,
            func: fn(Pin<&mut SecondObjectQt>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRust"]
        #[doc(hidden)]
        fn cxx_qt_ffi_rust(self: &SecondObjectQt) -> &SecondObject;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        #[doc(hidden)]
        fn cxx_qt_ffi_rust_mut(self: Pin<&mut SecondObjectQt>) -> Pin<&mut SecondObject>;
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::multi_object::cxx_qt_second_object"]
        fn create_rs_second_object() -> Box<SecondObject>;
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
    impl MyTrait for MyObject {
        fn my_func() -> String {
            "Hello".to_owned()
        }
    }
    pub struct MyObject {
        property_name: i32,
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn property_name<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
            cpp.property_name()
        }
    }
    impl MyObjectQt {
        #[doc = "Getter for the Q_PROPERTY "]
        #[doc = "property_name"]
        pub fn property_name(&self) -> &i32 {
            &self.rust().property_name
        }
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn set_property_name(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
            cpp.set_property_name(value);
        }
    }
    impl MyObjectQt {
        #[doc = "Setter for the Q_PROPERTY "]
        #[doc = "property_name"]
        pub fn set_property_name(mut self: Pin<&mut Self>, value: i32) {
            if self.rust().property_name == value {
                return;
            }
            self.as_mut().rust_mut().property_name = value;
            self.as_mut().property_name_changed();
        }
    }
    impl MyObjectQt {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "propertyNameChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_property_name_changed(
            self: Pin<&mut MyObjectQt>,
            func: fn(Pin<&mut MyObjectQt>),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_property_name_changed(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn invokable_name_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>) {
            cpp.invokable_name();
        }
    }
    impl MyObjectQt {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "ready"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_ready(
            self: Pin<&mut MyObjectQt>,
            func: fn(Pin<&mut MyObjectQt>),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_ready(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl cxx_qt::Locking for MyObjectQt {}
    impl cxx_qt::CxxQtType for MyObjectQt {
        type Rust = MyObject;
        fn rust(&self) -> &Self::Rust {
            self.cxx_qt_ffi_rust()
        }
        fn rust_mut(self: core::pin::Pin<&mut Self>) -> Pin<&mut Self::Rust> {
            self.cxx_qt_ffi_rust_mut()
        }
    }
    #[doc = r" Generated CXX-Qt method which creates a boxed rust struct of a QObject"]
    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
    pub struct SecondObject {
        property_name: i32,
    }
    impl SecondObject {
        #[doc(hidden)]
        pub fn property_name<'a>(&'a self, cpp: &'a SecondObjectQt) -> &'a i32 {
            cpp.property_name()
        }
    }
    impl SecondObjectQt {
        #[doc = "Getter for the Q_PROPERTY "]
        #[doc = "property_name"]
        pub fn property_name(&self) -> &i32 {
            &self.rust().property_name
        }
    }
    impl SecondObject {
        #[doc(hidden)]
        pub fn set_property_name(&mut self, cpp: Pin<&mut SecondObjectQt>, value: i32) {
            cpp.set_property_name(value);
        }
    }
    impl SecondObjectQt {
        #[doc = "Setter for the Q_PROPERTY "]
        #[doc = "property_name"]
        pub fn set_property_name(mut self: Pin<&mut Self>, value: i32) {
            if self.rust().property_name == value {
                return;
            }
            self.as_mut().rust_mut().property_name = value;
            self.as_mut().property_name_changed();
        }
    }
    impl SecondObjectQt {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "propertyNameChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_property_name_changed(
            self: Pin<&mut SecondObjectQt>,
            func: fn(Pin<&mut SecondObjectQt>),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_property_name_changed(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl SecondObject {
        #[doc(hidden)]
        pub fn invokable_name_wrapper(self: &mut SecondObject, cpp: Pin<&mut SecondObjectQt>) {
            cpp.invokable_name();
        }
    }
    impl SecondObjectQt {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "ready"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_ready(
            self: Pin<&mut SecondObjectQt>,
            func: fn(Pin<&mut SecondObjectQt>),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_ready(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl cxx_qt::CxxQtType for SecondObjectQt {
        type Rust = SecondObject;
        fn rust(&self) -> &Self::Rust {
            self.cxx_qt_ffi_rust()
        }
        fn rust_mut(self: core::pin::Pin<&mut Self>) -> Pin<&mut Self::Rust> {
            self.cxx_qt_ffi_rust_mut()
        }
    }
    #[doc = r" Generated CXX-Qt method which creates a boxed rust struct of a QObject"]
    pub fn create_rs_second_object() -> std::boxed::Box<SecondObject> {
        std::default::Default::default()
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
        #[doc = "The C++ type for the QObject "]
        #[doc = "SecondObject"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        pub type SecondObject = super::SecondObjectQt;
    }
}
