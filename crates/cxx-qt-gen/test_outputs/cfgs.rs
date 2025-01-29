#[cxx::bridge(namespace = "")]
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
    #[cfg(not(enabled))]
    enum EnumDisabled {
        A,
    }
    extern "C++" {
        #[cfg(not(enabled))]
        type EnumDisabled;
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
        #[cxx_name = "invokable_disabled"]
        #[cfg(not(enabled))]
        #[doc(hidden)]
        fn invokable_disabled(self: &MyObject);
    }
    unsafe extern "C++" {
        #[cxx_name = "inherit_disabledCxxQtInherit"]
        #[cfg(not(enabled))]
        fn inherit_disabled(self: &MyObject);
    }
    unsafe extern "C++" {
        #[cxx_name = "signal_disabled"]
        #[cfg(not(enabled))]
        fn signal_disabled(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[cfg(not(enabled))]
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlersignal_disabled = cxx_qt::signalhandler::CxxQtSignalHandler<
            super::MyObjectCxxQtSignalClosuresignal_disabled,
        >;
        #[cfg(not(enabled))]
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        #[cxx_name = "MyObject_signal_disabledConnect"]
        fn MyObject_connect_signal_disabled(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlersignal_disabled,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "rust::cxxqtgen1"]
    extern "Rust" {
        #[cfg(not(enabled))]
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_signal_disabled(
            handler: MyObjectCxxQtSignalHandlersignal_disabled,
        );
        #[cfg(not(enabled))]
        #[doc(hidden)]
        fn call_MyObject_signal_handler_signal_disabled(
            handler: &mut MyObjectCxxQtSignalHandlersignal_disabled,
            self_value: Pin<&mut MyObject>,
        );
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt_MyObject"]
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
    unsafe extern "C++" {}
}
#[cfg(not(enabled))]
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_disabled"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_signal_disabled<
        F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::MyObject_connect_signal_disabled(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    MyObjectCxxQtSignalClosuresignal_disabled,
                >::new(Box::new(closure)),
                conn_type,
            ),
        )
    }
}
#[cfg(not(enabled))]
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_disabled"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_signal_disabled<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::MyObject_connect_signal_disabled(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    MyObjectCxxQtSignalClosuresignal_disabled,
                >::new(Box::new(closure)),
                cxx_qt::ConnectionType::AutoConnection,
            ),
        )
    }
}
#[cfg(not(enabled))]
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosuresignal_disabled {}
#[cfg(not(enabled))]
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for MyObjectCxxQtSignalClosuresignal_disabled
{
    type Id = cxx::type_id!("::rust::cxxqtgen1::MyObjectCxxQtSignalHandlersignal_disabled");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>) + Send;
}
#[cfg(not(enabled))]
use core::mem::drop as drop_MyObject_signal_handler_signal_disabled;
#[cfg(not(enabled))]
fn call_MyObject_signal_handler_signal_disabled(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        MyObjectCxxQtSignalClosuresignal_disabled,
    >,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
) {
    handler.closure()(self_value);
}
#[cfg(not(enabled))]
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuresignal_disabled>,
    usize
);
#[cfg(not(enabled))]
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuresignal_disabled>,
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
