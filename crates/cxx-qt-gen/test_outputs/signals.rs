#[cxx::bridge(namespace = "cxx_qt::my_object")]
#[allow(unused_unsafe)]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
        type Opaque;
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
        #[namespace = "cxx_qt::my_object"]
        type MyObject;
    }
    extern "Rust" {
        #[namespace = "cxx_qt::my_object"]
        type MyObjectRust;
    }
    extern "Rust" {
        #[cxx_name = "invokable"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        unsafe fn invokable(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[cxx_name = "ready"]
        #[namespace = "cxx_qt::my_object"]
        fn ready(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlerready =
            cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosureready>;
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_readyConnect"]
        fn MyObject_connect_ready(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlerready,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
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
        #[cxx_name = "data_changed"]
        #[namespace = "cxx_qt::my_object"]
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
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlerdata_changed = cxx_qt::signalhandler::CxxQtSignalHandler<
            super::MyObjectCxxQtSignalClosuredata_changed,
        >;
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_data_changedConnect"]
        fn MyObject_connect_data_changed(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlerdata_changed,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_data_changed(
            handler: MyObjectCxxQtSignalHandlerdata_changed,
        );
        #[doc(hidden)]
        fn call_MyObject_signal_handler_data_changed(
            handler: &mut MyObjectCxxQtSignalHandlerdata_changed,
            self_value: Pin<&mut MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &QPoint,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "newData"]
        #[namespace = "cxx_qt::my_object"]
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
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlernewData =
            cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosurenewData>;
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_newDataConnect"]
        fn MyObject_connect_base_class_new_data(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlernewData,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
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
        include ! (< QtCore / QTimer >);
        #[namespace = "cxx_qt::my_object"]
        #[doc = " QTimer"]
        type QTimer;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        type QTimerCxxQtSignalHandlertimeout =
            cxx_qt::signalhandler::CxxQtSignalHandler<super::QTimerCxxQtSignalClosuretimeout>;
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        #[cxx_name = "QTimer_timeoutConnect"]
        fn QTimer_connect_timeout(
            self_value: Pin<&mut QTimer>,
            signal_handler: QTimerCxxQtSignalHandlertimeout,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
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
    pub fn connect_ready<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send>(
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
    pub fn on_ready<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send>(
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
        cxx::type_id!("::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerready");
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
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "data_changed"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_data_changed<
        F: FnMut(
                core::pin::Pin<&mut ffi::MyObject>,
                i32,
                cxx::UniquePtr<ffi::Opaque>,
                ffi::QPoint,
                &ffi::QPoint,
            )
            + 'static
            + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt :: QMetaObjectConnectionGuard :: from (ffi :: MyObject_connect_data_changed (self , cxx_qt :: signalhandler :: CxxQtSignalHandler :: < MyObjectCxxQtSignalClosuredata_changed > :: new (Box :: new (closure)) , conn_type ,))
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "data_changed"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_data_changed<
        F: FnMut(
                core::pin::Pin<&mut ffi::MyObject>,
                i32,
                cxx::UniquePtr<ffi::Opaque>,
                ffi::QPoint,
                &ffi::QPoint,
            )
            + 'static
            + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt :: QMetaObjectConnectionGuard :: from (ffi :: MyObject_connect_data_changed (self , cxx_qt :: signalhandler :: CxxQtSignalHandler :: < MyObjectCxxQtSignalClosuredata_changed > :: new (Box :: new (closure)) , cxx_qt :: ConnectionType :: AutoConnection ,))
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosuredata_changed {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosuredata_changed {
    type Id = cxx::type_id!(
        "::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerdata_changed"
    );
    type FnType = dyn FnMut(
            core::pin::Pin<&mut ffi::MyObject>,
            i32,
            cxx::UniquePtr<ffi::Opaque>,
            ffi::QPoint,
            &ffi::QPoint,
        ) + Send;
}
use core::mem::drop as drop_MyObject_signal_handler_data_changed;
fn call_MyObject_signal_handler_data_changed(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuredata_changed>,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
    first: i32,
    second: cxx::UniquePtr<ffi::Opaque>,
    third: ffi::QPoint,
    fourth: &ffi::QPoint,
) {
    handler.closure()(self_value, first, second, third, fourth);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuredata_changed>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuredata_changed>,
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
                cxx::UniquePtr<ffi::Opaque>,
                ffi::QPoint,
                &'a ffi::QPoint,
            )
            + 'static
            + Send,
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
                cxx::UniquePtr<ffi::Opaque>,
                ffi::QPoint,
                &'a ffi::QPoint,
            )
            + 'static
            + Send,
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
        cxx::type_id!("::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlernewData");
    type FnType = dyn FnMut(
            core::pin::Pin<&mut ffi::MyObject>,
            i32,
            cxx::UniquePtr<ffi::Opaque>,
            ffi::QPoint,
            &'a ffi::QPoint,
        ) + Send;
}
use core::mem::drop as drop_MyObject_signal_handler_newData;
fn call_MyObject_signal_handler_newData(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurenewData>,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
    first: i32,
    second: cxx::UniquePtr<ffi::Opaque>,
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
impl ffi::QTimer {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "timeout"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_timeout<F: FnMut(core::pin::Pin<&mut ffi::QTimer>) + 'static + Send>(
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
    pub fn on_timeout<F: FnMut(core::pin::Pin<&mut ffi::QTimer>) + 'static + Send>(
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
        cxx::type_id!("::cxx_qt::my_object::rust::cxxqtgen1::QTimerCxxQtSignalHandlertimeout");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::QTimer>) + Send;
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
