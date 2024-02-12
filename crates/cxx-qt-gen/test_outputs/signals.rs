#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }
    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-common/cxxqt_connection.h");
        #[doc(hidden)]
        #[namespace = "Qt"]
        #[rust_name = "CxxQtConnectionType"]
        type ConnectionType = cxx_qt::ConnectionType;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtcommon1"]
        #[rust_name = "CxxQtQMetaObjectConnection"]
        type QMetaObjectConnection = cxx_qt::QMetaObjectConnection;
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
        #[doc(hidden)]
        #[cxx_name = "invokableWrapper"]
        fn invokable(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[cxx_name = "ready"]
        fn ready(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1::cxx_qt::my_object"]
        type MyObjectCxxQtSignalHandlerready =
            cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosureready>;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1::cxx_qt::my_object"]
        #[must_use]
        #[rust_name = "MyObject_connect_ready"]
        fn MyObject_readyConnect(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlerready,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "rust::cxxqtgen1::cxx_qt::my_object"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_ready(handler: MyObjectCxxQtSignalHandlerready);
        #[doc(hidden)]
        fn call_MyObject_signal_handler_ready(
            handler: &mut MyObjectCxxQtSignalHandlerready,
            self_value: Pin<&mut MyObject>,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "dataChanged"]
        fn data_changed(
            self: Pin<&mut MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &QPoint,
        );
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1::cxx_qt::my_object"]
        type MyObjectCxxQtSignalHandlerdataChanged =
            cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosuredataChanged>;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1::cxx_qt::my_object"]
        #[must_use]
        #[rust_name = "MyObject_connect_data_changed"]
        fn MyObject_dataChangedConnect(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlerdataChanged,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "rust::cxxqtgen1::cxx_qt::my_object"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_dataChanged(handler: MyObjectCxxQtSignalHandlerdataChanged);
        #[doc(hidden)]
        fn call_MyObject_signal_handler_dataChanged(
            handler: &mut MyObjectCxxQtSignalHandlerdataChanged,
            self_value: Pin<&mut MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &QPoint,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "newData"]
        fn base_class_new_data(
            self: Pin<&mut MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        );
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1::cxx_qt::my_object"]
        type MyObjectCxxQtSignalHandlernewData =
            cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosurenewData>;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1::cxx_qt::my_object"]
        #[must_use]
        #[rust_name = "MyObject_connect_base_class_new_data"]
        fn MyObject_newDataConnect(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlernewData,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "rust::cxxqtgen1::cxx_qt::my_object"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_newData(handler: MyObjectCxxQtSignalHandlernewData);
        #[doc(hidden)]
        fn call_MyObject_signal_handler_newData(
            handler: &mut MyObjectCxxQtSignalHandlernewData,
            self_value: Pin<&mut MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        );
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
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
        include ! (< QtCore / QTimer >);
        #[doc = " QTimer"]
        type QTimer;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1::cxx_qt::my_object"]
        type QTimerCxxQtSignalHandlertimeout =
            cxx_qt::signalhandler::CxxQtSignalHandler<super::QTimerCxxQtSignalClosuretimeout>;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1::cxx_qt::my_object"]
        #[must_use]
        #[rust_name = "QTimer_connect_timeout"]
        fn QTimer_timeoutConnect(
            self_value: Pin<&mut QTimer>,
            signal_handler: QTimerCxxQtSignalHandlertimeout,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "rust::cxxqtgen1::cxx_qt::my_object"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_QTimer_signal_handler_timeout(handler: QTimerCxxQtSignalHandlertimeout);
        #[doc(hidden)]
        fn call_QTimer_signal_handler_timeout(
            handler: &mut QTimerCxxQtSignalHandlertimeout,
            self_value: Pin<&mut QTimer>,
        );
    }
}
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
        cxx::type_id!("::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalHandlerready");
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
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "dataChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_data_changed<
        F: FnMut(
                core::pin::Pin<&mut ffi::MyObject>,
                i32,
                cxx::UniquePtr<Opaque>,
                ffi::QPoint,
                &ffi::QPoint,
            ) + 'static,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_data_changed(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosuredataChanged>::new(
                Box::new(closure),
            ),
            conn_type,
        ))
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "dataChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_data_changed<
        F: FnMut(
                core::pin::Pin<&mut ffi::MyObject>,
                i32,
                cxx::UniquePtr<Opaque>,
                ffi::QPoint,
                &ffi::QPoint,
            ) + 'static,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_data_changed(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosuredataChanged>::new(
                Box::new(closure),
            ),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosuredataChanged {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosuredataChanged {
    type Id = cxx::type_id!(
        "::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalHandlerdataChanged"
    );
    type FnType = dyn FnMut(
        core::pin::Pin<&mut ffi::MyObject>,
        i32,
        cxx::UniquePtr<Opaque>,
        ffi::QPoint,
        &ffi::QPoint,
    );
}
use core::mem::drop as drop_MyObject_signal_handler_dataChanged;
fn call_MyObject_signal_handler_dataChanged(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuredataChanged>,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
    first: i32,
    second: cxx::UniquePtr<Opaque>,
    third: ffi::QPoint,
    fourth: &ffi::QPoint,
) {
    handler.closure()(self_value, first, second, third, fourth);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuredataChanged>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuredataChanged>,
    [usize; 2]
);
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "newData"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_base_class_new_data<
        F: FnMut(
                core::pin::Pin<&mut ffi::MyObject>,
                i32,
                cxx::UniquePtr<Opaque>,
                ffi::QPoint,
                &'a ffi::QPoint,
            ) + 'static,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_base_class_new_data(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosurenewData>::new(
                Box::new(closure),
            ),
            conn_type,
        ))
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "newData"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_base_class_new_data<
        F: FnMut(
                core::pin::Pin<&mut ffi::MyObject>,
                i32,
                cxx::UniquePtr<Opaque>,
                ffi::QPoint,
                &'a ffi::QPoint,
            ) + 'static,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_base_class_new_data(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosurenewData>::new(
                Box::new(closure),
            ),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosurenewData {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosurenewData {
    type Id =
        cxx::type_id!("::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalHandlernewData");
    type FnType = dyn FnMut(
        core::pin::Pin<&mut ffi::MyObject>,
        i32,
        cxx::UniquePtr<Opaque>,
        ffi::QPoint,
        &'a ffi::QPoint,
    );
}
use core::mem::drop as drop_MyObject_signal_handler_newData;
fn call_MyObject_signal_handler_newData(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurenewData>,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
    first: i32,
    second: cxx::UniquePtr<Opaque>,
    third: ffi::QPoint,
    fourth: &'a ffi::QPoint,
) {
    handler.closure()(self_value, first, second, third, fourth);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurenewData>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurenewData>,
    [usize; 2]
);
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
impl ffi::QTimer {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "timeout"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_timeout<F: FnMut(core::pin::Pin<&mut ffi::QTimer>) + 'static>(
        self: core::pin::Pin<&mut ffi::QTimer>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QTimer_connect_timeout(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<QTimerCxxQtSignalClosuretimeout>::new(
                Box::new(closure),
            ),
            conn_type,
        ))
    }
}
impl ffi::QTimer {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "timeout"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_timeout<F: FnMut(core::pin::Pin<&mut ffi::QTimer>) + 'static>(
        self: core::pin::Pin<&mut ffi::QTimer>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QTimer_connect_timeout(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<QTimerCxxQtSignalClosuretimeout>::new(
                Box::new(closure),
            ),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[doc(hidden)]
pub struct QTimerCxxQtSignalClosuretimeout {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for QTimerCxxQtSignalClosuretimeout {
    type Id =
        cxx::type_id!("::rust::cxxqtgen1::cxx_qt::my_object::QTimerCxxQtSignalHandlertimeout");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::QTimer>);
}
use core::mem::drop as drop_QTimer_signal_handler_timeout;
fn call_QTimer_signal_handler_timeout(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<QTimerCxxQtSignalClosuretimeout>,
    self_value: core::pin::Pin<&mut ffi::QTimer>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<QTimerCxxQtSignalClosuretimeout>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<QTimerCxxQtSignalClosuretimeout>,
    [usize; 2]
);
