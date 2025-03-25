#[cxx::bridge(namespace = "cxx_qt::multi_object")]
#[allow(unused_unsafe)]
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
    #[custom_attr = "test"]
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
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/concepts/generated_qobject.html>"]
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
        #[namespace = "cxx_qt::multi_object"]
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
        #[cxx_name = "invokable_name"]
        #[namespace = "cxx_qt::multi_object"]
        #[doc(hidden)]
        unsafe fn invokable_name(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[cxx_name = "ready"]
        #[namespace = "cxx_qt::multi_object"]
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
    extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "upcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_MyObject_upcastPtr(thiz: *const MyObject) -> *const QStringListModel;
        #[doc(hidden)]
        #[cxx_name = "downcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_MyObject_downcastPtr(base: *const QStringListModel)
            -> *const MyObject;
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::multi_object::cxx_qt_MyObject"]
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
        #[doc = "SecondObjectRust"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/concepts/generated_qobject.html>"]
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
        #[namespace = "second_object"]
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
        unsafe fn invokable_name(self: Pin<&mut SecondObject>);
    }
    extern "Rust" {
        #[cxx_name = "myRenamedFunction"]
        #[namespace = "second_object"]
        #[doc(hidden)]
        unsafe fn my_function(self: &SecondObject);
    }
    unsafe extern "C++" {
        #[cxx_name = "ready"]
        #[namespace = "second_object"]
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
    extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "upcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_SecondObject_upcastPtr(thiz: *const SecondObject) -> *const QObject;
        #[doc(hidden)]
        #[cxx_name = "downcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_SecondObject_downcastPtr(base: *const QObject) -> *const SecondObject;
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "second_object::cxx_qt_SecondObject"]
        fn create_rs_SecondObjectRust() -> Box<SecondObjectRust>;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRust"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_SecondObject_unsafeRust(outer: &SecondObject) -> &SecondObjectRust;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRustMut"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_SecondObject_unsafeRustMut(
            outer: Pin<&mut SecondObject>,
        ) -> Pin<&mut SecondObjectRust>;
    }
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "ThirdObjectRust"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/concepts/generated_qobject.html>"]
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
    extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "upcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_MyCxxName_upcastPtr(thiz: *const MyRustName) -> *const QObject;
        #[doc(hidden)]
        #[cxx_name = "downcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_MyCxxName_downcastPtr(base: *const QObject) -> *const MyRustName;
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "my_namespace::cxx_qt_MyRustName"]
        fn create_rs_ThirdObjectRust() -> Box<ThirdObjectRust>;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRust"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_MyCxxName_unsafeRust(outer: &MyRustName) -> &ThirdObjectRust;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRustMut"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_MyCxxName_unsafeRustMut(
            outer: Pin<&mut MyRustName>,
        ) -> Pin<&mut ThirdObjectRust>;
    }
    extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "upcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_QPushButton_upcastPtr(thiz: *const QPushButton) -> *const QObject;
        #[doc(hidden)]
        #[cxx_name = "downcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_QPushButton_downcastPtr(base: *const QObject) -> *const QPushButton;
    }
    #[namespace = ""]
    unsafe extern "C++" {
        #[namespace = "cxx_qt::multi_object"]
        type QPushButton;
    }
    extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "upcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_ExternObjectCpp_upcastPtr(thiz: *const ExternObject)
            -> *const QObject;
        #[doc(hidden)]
        #[cxx_name = "downcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn cxx_qt_ffi_ExternObjectCpp_downcastPtr(
            base: *const QObject,
        ) -> *const ExternObject;
    }
    #[namespace = ""]
    unsafe extern "C++" {
        #[namespace = "mynamespace"]
        #[cxx_name = "ExternObjectCpp"]
        type ExternObject;
    }
    unsafe extern "C++" {
        #[cxx_name = "clicked"]
        #[namespace = "cxx_qt::multi_object"]
        fn clicked(self: Pin<&mut QPushButton>, checked: bool);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::multi_object::rust::cxxqtgen1"]
        type QPushButtonCxxQtSignalHandlerclicked =
            cxx_qt::signalhandler::CxxQtSignalHandler<super::QPushButtonCxxQtSignalClosureclicked>;
        #[doc(hidden)]
        #[namespace = "cxx_qt::multi_object::rust::cxxqtgen1"]
        #[cxx_name = "QPushButton_clickedConnect"]
        fn QPushButton_connect_clicked(
            self_value: Pin<&mut QPushButton>,
            signal_handler: QPushButtonCxxQtSignalHandlerclicked,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::multi_object::rust::cxxqtgen1"]
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
        #[namespace = "mynamespace"]
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
        #[namespace = "mynamespace"]
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
    extern "C++" {
        #[doc(hidden)]
        #[namespace = ""]
        type QObject = cxx_qt::QObject;
    }
}
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
    pub fn connect_property_name_changed<
        F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        closure: F,
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
    pub fn on_property_name_changed<
        F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        closure: F,
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
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>) + Send;
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
    pub fn connect_ready<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        closure: F,
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
    pub fn on_ready<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        closure: F,
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
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>) + Send;
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
impl ::cxx_qt::Upcast<ffi::QStringListModel> for ffi::MyObject {
    unsafe fn upcast_ptr(this: *const Self) -> *const ffi::QStringListModel {
        ffi::cxx_qt_ffi_MyObject_upcastPtr(this)
    }
    unsafe fn from_base_ptr(base: *const ffi::QStringListModel) -> *const Self {
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
        F: FnMut(core::pin::Pin<&mut ffi::SecondObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::SecondObject>,
        closure: F,
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
    pub fn on_property_name_changed<
        F: FnMut(core::pin::Pin<&mut ffi::SecondObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::SecondObject>,
        closure: F,
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
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::SecondObject>) + Send;
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
    pub fn connect_ready<F: FnMut(core::pin::Pin<&mut ffi::SecondObject>) + 'static + Send>(
        self: core::pin::Pin<&mut ffi::SecondObject>,
        closure: F,
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
    pub fn on_ready<F: FnMut(core::pin::Pin<&mut ffi::SecondObject>) + 'static + Send>(
        self: core::pin::Pin<&mut ffi::SecondObject>,
        closure: F,
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
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::SecondObject>) + Send;
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
impl ::cxx_qt::Upcast<::cxx_qt::QObject> for ffi::SecondObject {
    unsafe fn upcast_ptr(this: *const Self) -> *const ::cxx_qt::QObject {
        ffi::cxx_qt_ffi_SecondObject_upcastPtr(this)
    }
    unsafe fn from_base_ptr(base: *const ::cxx_qt::QObject) -> *const Self {
        ffi::cxx_qt_ffi_SecondObject_downcastPtr(base)
    }
}
#[doc(hidden)]
#[allow(clippy::unnecessary_box_returns)]
pub fn create_rs_SecondObjectRust() -> std::boxed::Box<SecondObjectRust> {
    std::boxed::Box::new(core::default::Default::default())
}
impl ::core::ops::Deref for ffi::SecondObject {
    type Target = SecondObjectRust;
    fn deref(&self) -> &Self::Target {
        ffi::cxx_qt_ffi_SecondObject_unsafeRust(self)
    }
}
impl ::cxx_qt::CxxQtType for ffi::SecondObject {
    type Rust = SecondObjectRust;
    fn rust(&self) -> &Self::Rust {
        ffi::cxx_qt_ffi_SecondObject_unsafeRust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        ffi::cxx_qt_ffi_SecondObject_unsafeRustMut(self)
    }
}
impl ::cxx_qt::Upcast<::cxx_qt::QObject> for ffi::MyRustName {
    unsafe fn upcast_ptr(this: *const Self) -> *const ::cxx_qt::QObject {
        ffi::cxx_qt_ffi_MyCxxName_upcastPtr(this)
    }
    unsafe fn from_base_ptr(base: *const ::cxx_qt::QObject) -> *const Self {
        ffi::cxx_qt_ffi_MyCxxName_downcastPtr(base)
    }
}
#[doc(hidden)]
#[allow(clippy::unnecessary_box_returns)]
pub fn create_rs_ThirdObjectRust() -> std::boxed::Box<ThirdObjectRust> {
    std::boxed::Box::new(core::default::Default::default())
}
impl ::core::ops::Deref for ffi::MyRustName {
    type Target = ThirdObjectRust;
    fn deref(&self) -> &Self::Target {
        ffi::cxx_qt_ffi_MyCxxName_unsafeRust(self)
    }
}
impl ::cxx_qt::CxxQtType for ffi::MyRustName {
    type Rust = ThirdObjectRust;
    fn rust(&self) -> &Self::Rust {
        ffi::cxx_qt_ffi_MyCxxName_unsafeRust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        ffi::cxx_qt_ffi_MyCxxName_unsafeRustMut(self)
    }
}
impl ::cxx_qt::Upcast<::cxx_qt::QObject> for ffi::QPushButton {
    unsafe fn upcast_ptr(this: *const Self) -> *const ::cxx_qt::QObject {
        ffi::cxx_qt_ffi_QPushButton_upcastPtr(this)
    }
    unsafe fn from_base_ptr(base: *const ::cxx_qt::QObject) -> *const Self {
        ffi::cxx_qt_ffi_QPushButton_downcastPtr(base)
    }
}
impl ::cxx_qt::Upcast<::cxx_qt::QObject> for ffi::ExternObject {
    unsafe fn upcast_ptr(this: *const Self) -> *const ::cxx_qt::QObject {
        ffi::cxx_qt_ffi_ExternObjectCpp_upcastPtr(this)
    }
    unsafe fn from_base_ptr(base: *const ::cxx_qt::QObject) -> *const Self {
        ffi::cxx_qt_ffi_ExternObjectCpp_downcastPtr(base)
    }
}
impl ffi::QPushButton {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "clicked"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_clicked<
        F: FnMut(core::pin::Pin<&mut ffi::QPushButton>, bool) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QPushButton>,
        closure: F,
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
    pub fn on_clicked<F: FnMut(core::pin::Pin<&mut ffi::QPushButton>, bool) + 'static + Send>(
        self: core::pin::Pin<&mut ffi::QPushButton>,
        closure: F,
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
    type Id = cxx::type_id!(
        "::cxx_qt::multi_object::rust::cxxqtgen1::QPushButtonCxxQtSignalHandlerclicked"
    );
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::QPushButton>, bool) + Send;
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
    pub fn connect_data_ready<F: FnMut(core::pin::Pin<&mut ffi::ExternObject>) + 'static + Send>(
        self: core::pin::Pin<&mut ffi::ExternObject>,
        closure: F,
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
    pub fn on_data_ready<F: FnMut(core::pin::Pin<&mut ffi::ExternObject>) + 'static + Send>(
        self: core::pin::Pin<&mut ffi::ExternObject>,
        closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt :: QMetaObjectConnectionGuard :: from (ffi :: ExternObject_connect_data_ready (self , cxx_qt :: signalhandler :: CxxQtSignalHandler :: < ExternObjectCxxQtSignalClosuredataReady > :: new (Box :: new (closure)) , cxx_qt :: ConnectionType :: AutoConnection ,))
    }
}
#[doc(hidden)]
pub struct ExternObjectCxxQtSignalClosuredataReady {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for ExternObjectCxxQtSignalClosuredataReady {
    type Id =
        cxx::type_id!("::mynamespace::rust::cxxqtgen1::ExternObjectCxxQtSignalHandlerdataReady");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::ExternObject>) + Send;
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
    pub fn connect_error_occurred<
        F: FnMut(core::pin::Pin<&mut ffi::ExternObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::ExternObject>,
        closure: F,
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
    pub fn on_error_occurred<F: FnMut(core::pin::Pin<&mut ffi::ExternObject>) + 'static + Send>(
        self: core::pin::Pin<&mut ffi::ExternObject>,
        closure: F,
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
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::ExternObject>) + Send;
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
