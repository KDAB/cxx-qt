#[cxx::bridge(namespace = "cxx_qt::multi_object")]
pub mod ffi {
    const MAX: u16 = 65535;
    enum Event {
        MyEvent,
    }
    unsafe extern "C++" {
        type Event;
    }
    struct MyStruct {
        a: i32,
    }
    unsafe extern "C++" {
        type MyStruct;
    }
    extern crate serde;
    fn do_something() {
        println!("I am a free function");
    }
    extern "C" {}
    #[namespace = "namespace"]
    extern "C" {}
    #[namespace = "namespace"]
    extern "C" {}
    unsafe extern "C++" {}
    #[namespace = "namespace"]
    unsafe extern "C++" {}
    #[namespace = "namespace"]
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
    use super::MyTrait;
    #[namespace = ""]
    unsafe extern "C++" {
        include ! (< QtCore / QStringListModel >);
        type QStringListModel;
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
        #[namespace = "cxx_qt::multi_object"]
        type MyObject;
    }
    extern "Rust" {
        #[namespace = "cxx_qt::multi_object"]
        type MyObjectRust;
    }
    extern "Rust" {
        #[cxx_name = "getPropertyName"]
        #[namespace = "cxx_qt::multi_object"]
        unsafe fn property_name<'a>(self: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPropertyName"]
        #[namespace = "cxx_qt::multi_object"]
        fn set_property_name(self: Pin<&mut MyObject>, value: i32);
    }
    unsafe extern "C++" {
        #[cxx_name = "propertyNameChanged"]
        #[doc = "Notify for the Q_PROPERTY"]
        fn property_name_changed(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::multi_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlerpropertyNameChanged =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::MyObjectCxxQtSignalClosurepropertyNameChanged,
            >;
        #[doc(hidden)]
        #[namespace = "cxx_qt::multi_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_propertyNameChangedConnect"]
        fn MyObject_connect_property_name_changed(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlerpropertyNameChanged,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::multi_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_propertyNameChanged(
            handler: MyObjectCxxQtSignalHandlerpropertyNameChanged,
        );
        #[doc(hidden)]
        fn call_MyObject_signal_handler_propertyNameChanged(
            handler: &mut MyObjectCxxQtSignalHandlerpropertyNameChanged,
            self_value: Pin<&mut MyObject>,
        );
    }
    extern "Rust" {
        #[cxx_name = "invokableName"]
        #[namespace = "cxx_qt::multi_object"]
        #[doc(hidden)]
        fn invokable_name(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[cxx_name = "ready"]
        fn ready(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::multi_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlerready =
            cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosureready>;
        #[doc(hidden)]
        #[namespace = "cxx_qt::multi_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_readyConnect"]
        fn MyObject_connect_ready(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlerready,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::multi_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_ready(handler: MyObjectCxxQtSignalHandlerready);
        #[doc(hidden)]
        fn call_MyObject_signal_handler_ready(
            handler: &mut MyObjectCxxQtSignalHandlerready,
            self_value: Pin<&mut MyObject>,
        );
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::multi_object::cxx_qt_my_object"]
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
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "SecondObjectRust"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        #[namespace = "second_object"]
        type SecondObject;
    }
    extern "Rust" {
        #[namespace = "second_object"]
        type SecondObjectRust;
    }
    extern "Rust" {
        #[cxx_name = "getPropertyName"]
        #[namespace = "second_object"]
        unsafe fn property_name<'a>(self: &'a SecondObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPropertyName"]
        #[namespace = "second_object"]
        fn set_property_name(self: Pin<&mut SecondObject>, value: i32);
    }
    unsafe extern "C++" {
        #[cxx_name = "propertyNameChanged"]
        #[doc = "Notify for the Q_PROPERTY"]
        fn property_name_changed(self: Pin<&mut SecondObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "second_object::rust::cxxqtgen1"]
        type SecondObjectCxxQtSignalHandlerpropertyNameChanged =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::SecondObjectCxxQtSignalClosurepropertyNameChanged,
            >;
        #[doc(hidden)]
        #[namespace = "second_object::rust::cxxqtgen1"]
        #[cxx_name = "SecondObject_propertyNameChangedConnect"]
        fn SecondObject_connect_property_name_changed(
            self_value: Pin<&mut SecondObject>,
            signal_handler: SecondObjectCxxQtSignalHandlerpropertyNameChanged,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "second_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_SecondObject_signal_handler_propertyNameChanged(
            handler: SecondObjectCxxQtSignalHandlerpropertyNameChanged,
        );
        #[doc(hidden)]
        fn call_SecondObject_signal_handler_propertyNameChanged(
            handler: &mut SecondObjectCxxQtSignalHandlerpropertyNameChanged,
            self_value: Pin<&mut SecondObject>,
        );
    }
    extern "Rust" {
        #[cxx_name = "invokableName"]
        #[namespace = "second_object"]
        #[doc(hidden)]
        fn invokable_name(self: Pin<&mut SecondObject>);
    }
    unsafe extern "C++" {
        #[cxx_name = "ready"]
        fn ready(self: Pin<&mut SecondObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "second_object::rust::cxxqtgen1"]
        type SecondObjectCxxQtSignalHandlerready =
            cxx_qt::signalhandler::CxxQtSignalHandler<super::SecondObjectCxxQtSignalClosureready>;
        #[doc(hidden)]
        #[namespace = "second_object::rust::cxxqtgen1"]
        #[cxx_name = "SecondObject_readyConnect"]
        fn SecondObject_connect_ready(
            self_value: Pin<&mut SecondObject>,
            signal_handler: SecondObjectCxxQtSignalHandlerready,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "second_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_SecondObject_signal_handler_ready(handler: SecondObjectCxxQtSignalHandlerready);
        #[doc(hidden)]
        fn call_SecondObject_signal_handler_ready(
            handler: &mut SecondObjectCxxQtSignalHandlerready,
            self_value: Pin<&mut SecondObject>,
        );
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "second_object::cxx_qt_second_object"]
        fn create_rs_second_object_rust() -> Box<SecondObjectRust>;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRust"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_second_object_unsafe_rust(outer: &SecondObject) -> &SecondObjectRust;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRustMut"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_second_object_unsafe_rust_mut(
            outer: Pin<&mut SecondObject>,
        ) -> Pin<&mut SecondObjectRust>;
    }
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "ThirdObjectRust"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        #[namespace = "my_namespace"]
        #[doc = "\n\nNote: The C++ name of this QObject is: "]
        #[doc = "MyCxxName"]
        #[cxx_name = "MyCxxName"]
        type MyRustName;
    }
    extern "Rust" {
        #[namespace = "my_namespace"]
        type ThirdObjectRust;
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "my_namespace::cxx_qt_my_rust_name"]
        fn create_rs_third_object_rust() -> Box<ThirdObjectRust>;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRust"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_my_cxx_name_unsafe_rust(outer: &MyRustName) -> &ThirdObjectRust;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRustMut"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_my_cxx_name_unsafe_rust_mut(
            outer: Pin<&mut MyRustName>,
        ) -> Pin<&mut ThirdObjectRust>;
    }
    #[namespace = ""]
    unsafe extern "C++" {
        type QPushButton;
        #[namespace = "mynamespace"]
        #[cxx_name = "ExternObjectCpp"]
        type ExternObject;
    }
    unsafe extern "C++" {
        #[cxx_name = "clicked"]
        fn clicked(self: Pin<&mut QPushButton>, checked: bool);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        type QPushButtonCxxQtSignalHandlerclicked =
            cxx_qt::signalhandler::CxxQtSignalHandler<super::QPushButtonCxxQtSignalClosureclicked>;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        #[cxx_name = "QPushButton_clickedConnect"]
        fn QPushButton_connect_clicked(
            self_value: Pin<&mut QPushButton>,
            signal_handler: QPushButtonCxxQtSignalHandlerclicked,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_QPushButton_signal_handler_clicked(handler: QPushButtonCxxQtSignalHandlerclicked);
        #[doc(hidden)]
        fn call_QPushButton_signal_handler_clicked(
            handler: &mut QPushButtonCxxQtSignalHandlerclicked,
            self_value: Pin<&mut QPushButton>,
            checked: bool,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "dataReady"]
        fn data_ready(self: Pin<&mut ExternObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "mynamespace::rust::cxxqtgen1"]
        type ExternObjectCxxQtSignalHandlerdataReady = cxx_qt::signalhandler::CxxQtSignalHandler<
            super::ExternObjectCxxQtSignalClosuredataReady,
        >;
        #[doc(hidden)]
        #[namespace = "mynamespace::rust::cxxqtgen1"]
        #[cxx_name = "ExternObjectCpp_dataReadyConnect"]
        fn ExternObject_connect_data_ready(
            self_value: Pin<&mut ExternObject>,
            signal_handler: ExternObjectCxxQtSignalHandlerdataReady,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "mynamespace::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_ExternObject_signal_handler_dataReady(
            handler: ExternObjectCxxQtSignalHandlerdataReady,
        );
        #[doc(hidden)]
        fn call_ExternObject_signal_handler_dataReady(
            handler: &mut ExternObjectCxxQtSignalHandlerdataReady,
            self_value: Pin<&mut ExternObject>,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "errorOccurred"]
        fn error_occurred(self: Pin<&mut ExternObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "mynamespace::rust::cxxqtgen1"]
        type ExternObjectCxxQtSignalHandlererrorOccurred =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::ExternObjectCxxQtSignalClosureerrorOccurred,
            >;
        #[doc(hidden)]
        #[namespace = "mynamespace::rust::cxxqtgen1"]
        #[cxx_name = "ExternObjectCpp_errorOccurredConnect"]
        fn ExternObject_connect_error_occurred(
            self_value: Pin<&mut ExternObject>,
            signal_handler: ExternObjectCxxQtSignalHandlererrorOccurred,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "mynamespace::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_ExternObject_signal_handler_errorOccurred(
            handler: ExternObjectCxxQtSignalHandlererrorOccurred,
        );
        #[doc(hidden)]
        fn call_ExternObject_signal_handler_errorOccurred(
            handler: &mut ExternObjectCxxQtSignalHandlererrorOccurred,
            self_value: Pin<&mut ExternObject>,
        );
    }
}
impl cxx_qt::Upcast<ffi::QStringListModel> for ffi::MyObject {}
#[allow(unused_imports)]
#[allow(dead_code)]
use ffi::QStringListModel as _;
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
    pub fn connect_property_name_changed<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_property_name_changed(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                MyObjectCxxQtSignalClosurepropertyNameChanged,
            >::new(Box::new(closure)),
            conn_type,
        ))
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "propertyNameChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_property_name_changed<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_property_name_changed(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                MyObjectCxxQtSignalClosurepropertyNameChanged,
            >::new(Box::new(closure)),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosurepropertyNameChanged {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for MyObjectCxxQtSignalClosurepropertyNameChanged
{
    type Id = cxx::type_id!(
        "::cxx_qt::multi_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerpropertyNameChanged"
    );
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>);
}
use core::mem::drop as drop_MyObject_signal_handler_propertyNameChanged;
fn call_MyObject_signal_handler_propertyNameChanged(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        MyObjectCxxQtSignalClosurepropertyNameChanged,
    >,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurepropertyNameChanged>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurepropertyNameChanged>,
    [usize; 2]
);
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "ready"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_ready<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_ready(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureready>::new(
                Box::new(closure),
            ),
            conn_type,
        ))
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "ready"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_ready<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_ready(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureready>::new(
                Box::new(closure),
            ),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosureready {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosureready {
    type Id =
        cxx::type_id!("::cxx_qt::multi_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerready");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>);
}
use core::mem::drop as drop_MyObject_signal_handler_ready;
fn call_MyObject_signal_handler_ready(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>,
    [usize; 2]
);
#[doc(hidden)]
pub fn create_rs_my_object_rust() -> std::boxed::Box<MyObjectRust> {
    std::boxed::Box::new(core::default::Default::default())
}
impl ::core::ops::Deref for ffi::MyObject {
    type Target = MyObjectRust;
    fn deref(&self) -> &Self::Target {
        ffi::cxx_qt_ffi_my_object_unsafe_rust(self)
    }
}
impl ::cxx_qt::CxxQtType for ffi::MyObject {
    type Rust = MyObjectRust;
    fn rust(&self) -> &Self::Rust {
        ffi::cxx_qt_ffi_my_object_unsafe_rust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        ffi::cxx_qt_ffi_my_object_unsafe_rust_mut(self)
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
    pub fn connect_property_name_changed<
        F: FnMut(core::pin::Pin<&mut ffi::SecondObject>) + 'static,
    >(
        self: core::pin::Pin<&mut ffi::SecondObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::SecondObject_connect_property_name_changed(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                SecondObjectCxxQtSignalClosurepropertyNameChanged,
            >::new(Box::new(closure)),
            conn_type,
        ))
    }
}
impl ffi::SecondObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "propertyNameChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_property_name_changed<F: FnMut(core::pin::Pin<&mut ffi::SecondObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::SecondObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::SecondObject_connect_property_name_changed(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                SecondObjectCxxQtSignalClosurepropertyNameChanged,
            >::new(Box::new(closure)),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[doc(hidden)]
pub struct SecondObjectCxxQtSignalClosurepropertyNameChanged {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for SecondObjectCxxQtSignalClosurepropertyNameChanged
{
    type Id = cxx::type_id!(
        "::second_object::rust::cxxqtgen1::SecondObjectCxxQtSignalHandlerpropertyNameChanged"
    );
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::SecondObject>);
}
use core::mem::drop as drop_SecondObject_signal_handler_propertyNameChanged;
fn call_SecondObject_signal_handler_propertyNameChanged(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        SecondObjectCxxQtSignalClosurepropertyNameChanged,
    >,
    self_value: core::pin::Pin<&mut ffi::SecondObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<SecondObjectCxxQtSignalClosurepropertyNameChanged>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<SecondObjectCxxQtSignalClosurepropertyNameChanged>,
    [usize; 2]
);
impl ffi::SecondObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "ready"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_ready<F: FnMut(core::pin::Pin<&mut ffi::SecondObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::SecondObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::SecondObject_connect_ready(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<SecondObjectCxxQtSignalClosureready>::new(
                Box::new(closure),
            ),
            conn_type,
        ))
    }
}
impl ffi::SecondObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "ready"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_ready<F: FnMut(core::pin::Pin<&mut ffi::SecondObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::SecondObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::SecondObject_connect_ready(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<SecondObjectCxxQtSignalClosureready>::new(
                Box::new(closure),
            ),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[doc(hidden)]
pub struct SecondObjectCxxQtSignalClosureready {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for SecondObjectCxxQtSignalClosureready {
    type Id =
        cxx::type_id!("::second_object::rust::cxxqtgen1::SecondObjectCxxQtSignalHandlerready");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::SecondObject>);
}
use core::mem::drop as drop_SecondObject_signal_handler_ready;
fn call_SecondObject_signal_handler_ready(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<SecondObjectCxxQtSignalClosureready>,
    self_value: core::pin::Pin<&mut ffi::SecondObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<SecondObjectCxxQtSignalClosureready>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<SecondObjectCxxQtSignalClosureready>,
    [usize; 2]
);
#[doc(hidden)]
pub fn create_rs_second_object_rust() -> std::boxed::Box<SecondObjectRust> {
    std::boxed::Box::new(core::default::Default::default())
}
impl ::core::ops::Deref for ffi::SecondObject {
    type Target = SecondObjectRust;
    fn deref(&self) -> &Self::Target {
        ffi::cxx_qt_ffi_second_object_unsafe_rust(self)
    }
}
impl ::cxx_qt::CxxQtType for ffi::SecondObject {
    type Rust = SecondObjectRust;
    fn rust(&self) -> &Self::Rust {
        ffi::cxx_qt_ffi_second_object_unsafe_rust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        ffi::cxx_qt_ffi_second_object_unsafe_rust_mut(self)
    }
}
#[doc(hidden)]
pub fn create_rs_third_object_rust() -> std::boxed::Box<ThirdObjectRust> {
    std::boxed::Box::new(core::default::Default::default())
}
impl ::core::ops::Deref for ffi::MyRustName {
    type Target = ThirdObjectRust;
    fn deref(&self) -> &Self::Target {
        ffi::cxx_qt_ffi_my_cxx_name_unsafe_rust(self)
    }
}
impl ::cxx_qt::CxxQtType for ffi::MyRustName {
    type Rust = ThirdObjectRust;
    fn rust(&self) -> &Self::Rust {
        ffi::cxx_qt_ffi_my_cxx_name_unsafe_rust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        ffi::cxx_qt_ffi_my_cxx_name_unsafe_rust_mut(self)
    }
}
impl ffi::QPushButton {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "clicked"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_clicked<F: FnMut(core::pin::Pin<&mut ffi::QPushButton>, bool) + 'static>(
        self: core::pin::Pin<&mut ffi::QPushButton>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QPushButton_connect_clicked(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<QPushButtonCxxQtSignalClosureclicked>::new(
                Box::new(closure),
            ),
            conn_type,
        ))
    }
}
impl ffi::QPushButton {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "clicked"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_clicked<F: FnMut(core::pin::Pin<&mut ffi::QPushButton>, bool) + 'static>(
        self: core::pin::Pin<&mut ffi::QPushButton>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QPushButton_connect_clicked(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<QPushButtonCxxQtSignalClosureclicked>::new(
                Box::new(closure),
            ),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[doc(hidden)]
pub struct QPushButtonCxxQtSignalClosureclicked {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for QPushButtonCxxQtSignalClosureclicked {
    type Id = cxx::type_id!("::rust::cxxqtgen1::QPushButtonCxxQtSignalHandlerclicked");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::QPushButton>, bool);
}
use core::mem::drop as drop_QPushButton_signal_handler_clicked;
fn call_QPushButton_signal_handler_clicked(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<QPushButtonCxxQtSignalClosureclicked>,
    self_value: core::pin::Pin<&mut ffi::QPushButton>,
    checked: bool,
) {
    handler.closure()(self_value, checked);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<QPushButtonCxxQtSignalClosureclicked>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<QPushButtonCxxQtSignalClosureclicked>,
    [usize; 2]
);
impl ffi::ExternObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "dataReady"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_data_ready<F: FnMut(core::pin::Pin<&mut ffi::ExternObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::ExternObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt :: QMetaObjectConnectionGuard :: from (ffi :: ExternObject_connect_data_ready (self , cxx_qt :: signalhandler :: CxxQtSignalHandler :: < ExternObjectCxxQtSignalClosuredataReady > :: new (Box :: new (closure)) , conn_type ,))
    }
}
impl ffi::ExternObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "dataReady"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_data_ready<F: FnMut(core::pin::Pin<&mut ffi::ExternObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::ExternObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt :: QMetaObjectConnectionGuard :: from (ffi :: ExternObject_connect_data_ready (self , cxx_qt :: signalhandler :: CxxQtSignalHandler :: < ExternObjectCxxQtSignalClosuredataReady > :: new (Box :: new (closure)) , cxx_qt :: ConnectionType :: AutoConnection ,))
    }
}
#[doc(hidden)]
pub struct ExternObjectCxxQtSignalClosuredataReady {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for ExternObjectCxxQtSignalClosuredataReady {
    type Id =
        cxx::type_id!("::mynamespace::rust::cxxqtgen1::ExternObjectCxxQtSignalHandlerdataReady");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::ExternObject>);
}
use core::mem::drop as drop_ExternObject_signal_handler_dataReady;
fn call_ExternObject_signal_handler_dataReady(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        ExternObjectCxxQtSignalClosuredataReady,
    >,
    self_value: core::pin::Pin<&mut ffi::ExternObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<ExternObjectCxxQtSignalClosuredataReady>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<ExternObjectCxxQtSignalClosuredataReady>,
    [usize; 2]
);
impl ffi::ExternObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "errorOccurred"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_error_occurred<F: FnMut(core::pin::Pin<&mut ffi::ExternObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::ExternObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::ExternObject_connect_error_occurred(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    ExternObjectCxxQtSignalClosureerrorOccurred,
                >::new(Box::new(closure)),
                conn_type,
            ),
        )
    }
}
impl ffi::ExternObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "errorOccurred"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_error_occurred<F: FnMut(core::pin::Pin<&mut ffi::ExternObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::ExternObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::ExternObject_connect_error_occurred(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    ExternObjectCxxQtSignalClosureerrorOccurred,
                >::new(Box::new(closure)),
                cxx_qt::ConnectionType::AutoConnection,
            ),
        )
    }
}
#[doc(hidden)]
pub struct ExternObjectCxxQtSignalClosureerrorOccurred {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for ExternObjectCxxQtSignalClosureerrorOccurred
{
    type Id = cxx::type_id!(
        "::mynamespace::rust::cxxqtgen1::ExternObjectCxxQtSignalHandlererrorOccurred"
    );
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::ExternObject>);
}
use core::mem::drop as drop_ExternObject_signal_handler_errorOccurred;
fn call_ExternObject_signal_handler_errorOccurred(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        ExternObjectCxxQtSignalClosureerrorOccurred,
    >,
    self_value: core::pin::Pin<&mut ffi::ExternObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<ExternObjectCxxQtSignalClosureerrorOccurred>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<ExternObjectCxxQtSignalClosureerrorOccurred>,
    [usize; 2]
);
