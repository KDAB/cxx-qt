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
    enum EnumDisabled1 {
        A,
    }
    extern "C++" {
        #[cfg(not(enabled))]
        type EnumDisabled1;
    }
    #[repr(i32)]
    #[cfg(enabled)]
    enum EnumEnabled1 {
        A,
    }
    extern "C++" {
        #[cfg(enabled)]
        type EnumEnabled1;
    }
    #[repr(i32)]
    #[cfg(not(enabled))]
    enum EnumDisabled2 {
        A,
    }
    extern "C++" {
        #[cfg(not(enabled))]
        type EnumDisabled2;
    }
    #[repr(i32)]
    #[cfg(enabled)]
    enum EnumEnabled2 {
        A,
    }
    extern "C++" {
        #[cfg(enabled)]
        type EnumEnabled2;
    }
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "QObjectEnabledRust"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        #[cfg(enabled)]
        type QObjectEnabled;
    }
    extern "Rust" {
        #[cfg(enabled)]
        type QObjectEnabledRust;
    }
    extern "Rust" {
        #[cxx_name = "invokable_disabled"]
        #[cfg(not(enabled))]
        #[doc(hidden)]
        fn invokable_disabled(self: &QObjectEnabled);
    }
    extern "Rust" {
        #[cxx_name = "invokable_enabled"]
        #[cfg(enabled)]
        #[doc(hidden)]
        fn invokable_enabled(self: &QObjectEnabled);
    }
    unsafe extern "C++" {
        #[cxx_name = "inherit_disabledCxxQtInherit"]
        #[cfg(not(enabled))]
        fn inherit_disabled(self: &QObjectEnabled);
    }
    unsafe extern "C++" {
        #[cxx_name = "inherit_enabledCxxQtInherit"]
        #[cfg(enabled)]
        fn inherit_enabled(self: &QObjectEnabled);
    }
    unsafe extern "C++" {
        #[cxx_name = "signal_disabled"]
        #[cfg(not(enabled))]
        fn signal_disabled(self: Pin<&mut QObjectEnabled>);
    }
    #[cfg(not(enabled))]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        type QObjectEnabledCxxQtSignalHandlersignal_disabled =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::QObjectEnabledCxxQtSignalClosuresignal_disabled,
            >;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        #[cxx_name = "QObjectEnabled_signal_disabledConnect"]
        fn QObjectEnabled_connect_signal_disabled(
            self_value: Pin<&mut QObjectEnabled>,
            signal_handler: QObjectEnabledCxxQtSignalHandlersignal_disabled,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[cfg(not(enabled))]
    #[namespace = "rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_QObjectEnabled_signal_handler_signal_disabled(
            handler: QObjectEnabledCxxQtSignalHandlersignal_disabled,
        );
        #[doc(hidden)]
        fn call_QObjectEnabled_signal_handler_signal_disabled(
            handler: &mut QObjectEnabledCxxQtSignalHandlersignal_disabled,
            self_value: Pin<&mut QObjectEnabled>,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "signal_enabled"]
        #[cfg(enabled)]
        fn signal_enabled(self: Pin<&mut QObjectEnabled>);
    }
    #[cfg(enabled)]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        type QObjectEnabledCxxQtSignalHandlersignal_enabled =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::QObjectEnabledCxxQtSignalClosuresignal_enabled,
            >;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        #[cxx_name = "QObjectEnabled_signal_enabledConnect"]
        fn QObjectEnabled_connect_signal_enabled(
            self_value: Pin<&mut QObjectEnabled>,
            signal_handler: QObjectEnabledCxxQtSignalHandlersignal_enabled,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[cfg(enabled)]
    #[namespace = "rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_QObjectEnabled_signal_handler_signal_enabled(
            handler: QObjectEnabledCxxQtSignalHandlersignal_enabled,
        );
        #[doc(hidden)]
        fn call_QObjectEnabled_signal_handler_signal_enabled(
            handler: &mut QObjectEnabledCxxQtSignalHandlersignal_enabled,
            self_value: Pin<&mut QObjectEnabled>,
        );
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt_QObjectEnabled"]
        #[cfg(enabled)]
        fn create_rs_QObjectEnabledRust() -> Box<QObjectEnabledRust>;
    }
    #[cfg(enabled)]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRust"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_QObjectEnabled_unsafeRust(outer: &QObjectEnabled) -> &QObjectEnabledRust;
    }
    #[cfg(enabled)]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRustMut"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_QObjectEnabled_unsafeRustMut(
            outer: Pin<&mut QObjectEnabled>,
        ) -> Pin<&mut QObjectEnabledRust>;
    }
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "QObjectDisabledRust"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        #[cfg(not(enabled))]
        type QObjectDisabled;
    }
    extern "Rust" {
        #[cfg(not(enabled))]
        type QObjectDisabledRust;
    }
    extern "Rust" {
        #[cxx_name = "invokable_disabled"]
        #[cfg(not(enabled))]
        #[doc(hidden)]
        fn invokable_disabled(self: &QObjectDisabled);
    }
    extern "Rust" {
        #[cxx_name = "invokable_enabled"]
        #[cfg(enabled)]
        #[doc(hidden)]
        fn invokable_enabled(self: &QObjectDisabled);
    }
    unsafe extern "C++" {
        #[cxx_name = "inherit_disabledCxxQtInherit"]
        #[cfg(not(enabled))]
        fn inherit_disabled(self: &QObjectDisabled);
    }
    unsafe extern "C++" {
        #[cxx_name = "inherit_enabledCxxQtInherit"]
        #[cfg(enabled)]
        fn inherit_enabled(self: &QObjectDisabled);
    }
    unsafe extern "C++" {
        #[cxx_name = "signal_disabled"]
        #[cfg(not(enabled))]
        fn signal_disabled(self: Pin<&mut QObjectDisabled>);
    }
    #[cfg(not(enabled))]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        type QObjectDisabledCxxQtSignalHandlersignal_disabled =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::QObjectDisabledCxxQtSignalClosuresignal_disabled,
            >;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        #[cxx_name = "QObjectDisabled_signal_disabledConnect"]
        fn QObjectDisabled_connect_signal_disabled(
            self_value: Pin<&mut QObjectDisabled>,
            signal_handler: QObjectDisabledCxxQtSignalHandlersignal_disabled,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[cfg(not(enabled))]
    #[namespace = "rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_QObjectDisabled_signal_handler_signal_disabled(
            handler: QObjectDisabledCxxQtSignalHandlersignal_disabled,
        );
        #[doc(hidden)]
        fn call_QObjectDisabled_signal_handler_signal_disabled(
            handler: &mut QObjectDisabledCxxQtSignalHandlersignal_disabled,
            self_value: Pin<&mut QObjectDisabled>,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "signal_enabled"]
        #[cfg(enabled)]
        fn signal_enabled(self: Pin<&mut QObjectDisabled>);
    }
    #[cfg(enabled)]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        type QObjectDisabledCxxQtSignalHandlersignal_enabled =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::QObjectDisabledCxxQtSignalClosuresignal_enabled,
            >;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        #[cxx_name = "QObjectDisabled_signal_enabledConnect"]
        fn QObjectDisabled_connect_signal_enabled(
            self_value: Pin<&mut QObjectDisabled>,
            signal_handler: QObjectDisabledCxxQtSignalHandlersignal_enabled,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[cfg(enabled)]
    #[namespace = "rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_QObjectDisabled_signal_handler_signal_enabled(
            handler: QObjectDisabledCxxQtSignalHandlersignal_enabled,
        );
        #[doc(hidden)]
        fn call_QObjectDisabled_signal_handler_signal_enabled(
            handler: &mut QObjectDisabledCxxQtSignalHandlersignal_enabled,
            self_value: Pin<&mut QObjectDisabled>,
        );
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt_QObjectDisabled"]
        #[cfg(not(enabled))]
        fn create_rs_QObjectDisabledRust() -> Box<QObjectDisabledRust>;
    }
    #[cfg(not(enabled))]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRust"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_QObjectDisabled_unsafeRust(outer: &QObjectDisabled) -> &QObjectDisabledRust;
    }
    #[cfg(not(enabled))]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRustMut"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_QObjectDisabled_unsafeRustMut(
            outer: Pin<&mut QObjectDisabled>,
        ) -> Pin<&mut QObjectDisabledRust>;
    }
    unsafe extern "C++" {
        #[cfg(enabled)]
        type QObjectExternEnabled;
    }
    unsafe extern "C++" {
        #[cxx_name = "signal_disabled1"]
        #[cfg(not(enabled))]
        fn signal_disabled1(self: Pin<&mut QObjectExternEnabled>);
    }
    #[cfg(not(enabled))]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        type QObjectExternEnabledCxxQtSignalHandlersignal_disabled1 =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::QObjectExternEnabledCxxQtSignalClosuresignal_disabled1,
            >;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        #[cxx_name = "QObjectExternEnabled_signal_disabled1Connect"]
        fn QObjectExternEnabled_connect_signal_disabled1(
            self_value: Pin<&mut QObjectExternEnabled>,
            signal_handler: QObjectExternEnabledCxxQtSignalHandlersignal_disabled1,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[cfg(not(enabled))]
    #[namespace = "rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_QObjectExternEnabled_signal_handler_signal_disabled1(
            handler: QObjectExternEnabledCxxQtSignalHandlersignal_disabled1,
        );
        #[doc(hidden)]
        fn call_QObjectExternEnabled_signal_handler_signal_disabled1(
            handler: &mut QObjectExternEnabledCxxQtSignalHandlersignal_disabled1,
            self_value: Pin<&mut QObjectExternEnabled>,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "signal_enabled1"]
        #[cfg(enabled)]
        fn signal_enabled1(self: Pin<&mut QObjectExternEnabled>);
    }
    #[cfg(enabled)]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        type QObjectExternEnabledCxxQtSignalHandlersignal_enabled1 =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::QObjectExternEnabledCxxQtSignalClosuresignal_enabled1,
            >;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        #[cxx_name = "QObjectExternEnabled_signal_enabled1Connect"]
        fn QObjectExternEnabled_connect_signal_enabled1(
            self_value: Pin<&mut QObjectExternEnabled>,
            signal_handler: QObjectExternEnabledCxxQtSignalHandlersignal_enabled1,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[cfg(enabled)]
    #[namespace = "rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_QObjectExternEnabled_signal_handler_signal_enabled1(
            handler: QObjectExternEnabledCxxQtSignalHandlersignal_enabled1,
        );
        #[doc(hidden)]
        fn call_QObjectExternEnabled_signal_handler_signal_enabled1(
            handler: &mut QObjectExternEnabledCxxQtSignalHandlersignal_enabled1,
            self_value: Pin<&mut QObjectExternEnabled>,
        );
    }
    unsafe extern "C++" {
        #[cfg(not(enabled))]
        type QObjectExternDisabled;
    }
    unsafe extern "C++" {
        #[cxx_name = "signal_disabled2"]
        #[cfg(not(enabled))]
        fn signal_disabled2(self: Pin<&mut QObjectExternDisabled>);
    }
    #[cfg(not(enabled))]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        type QObjectExternDisabledCxxQtSignalHandlersignal_disabled2 =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::QObjectExternDisabledCxxQtSignalClosuresignal_disabled2,
            >;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        #[cxx_name = "QObjectExternDisabled_signal_disabled2Connect"]
        fn QObjectExternDisabled_connect_signal_disabled2(
            self_value: Pin<&mut QObjectExternDisabled>,
            signal_handler: QObjectExternDisabledCxxQtSignalHandlersignal_disabled2,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[cfg(not(enabled))]
    #[namespace = "rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_QObjectExternDisabled_signal_handler_signal_disabled2(
            handler: QObjectExternDisabledCxxQtSignalHandlersignal_disabled2,
        );
        #[doc(hidden)]
        fn call_QObjectExternDisabled_signal_handler_signal_disabled2(
            handler: &mut QObjectExternDisabledCxxQtSignalHandlersignal_disabled2,
            self_value: Pin<&mut QObjectExternDisabled>,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "signal_enabled2"]
        #[cfg(enabled)]
        fn signal_enabled2(self: Pin<&mut QObjectExternDisabled>);
    }
    #[cfg(enabled)]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        type QObjectExternDisabledCxxQtSignalHandlersignal_enabled2 =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::QObjectExternDisabledCxxQtSignalClosuresignal_enabled2,
            >;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtgen1"]
        #[cxx_name = "QObjectExternDisabled_signal_enabled2Connect"]
        fn QObjectExternDisabled_connect_signal_enabled2(
            self_value: Pin<&mut QObjectExternDisabled>,
            signal_handler: QObjectExternDisabledCxxQtSignalHandlersignal_enabled2,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[cfg(enabled)]
    #[namespace = "rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_QObjectExternDisabled_signal_handler_signal_enabled2(
            handler: QObjectExternDisabledCxxQtSignalHandlersignal_enabled2,
        );
        #[doc(hidden)]
        fn call_QObjectExternDisabled_signal_handler_signal_enabled2(
            handler: &mut QObjectExternDisabledCxxQtSignalHandlersignal_enabled2,
            self_value: Pin<&mut QObjectExternDisabled>,
        );
    }
}
#[cfg(not(enabled))]
impl ffi::QObjectEnabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_disabled"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_signal_disabled<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectEnabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectEnabled>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QObjectEnabled_connect_signal_disabled(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                QObjectEnabledCxxQtSignalClosuresignal_disabled,
            >::new(Box::new(closure)),
            conn_type,
        ))
    }
}
#[cfg(not(enabled))]
impl ffi::QObjectEnabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_disabled"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_signal_disabled<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectEnabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectEnabled>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QObjectEnabled_connect_signal_disabled(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                QObjectEnabledCxxQtSignalClosuresignal_disabled,
            >::new(Box::new(closure)),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[cfg(not(enabled))]
#[doc(hidden)]
pub struct QObjectEnabledCxxQtSignalClosuresignal_disabled {}
#[cfg(not(enabled))]
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for QObjectEnabledCxxQtSignalClosuresignal_disabled
{
    type Id = cxx::type_id!("::rust::cxxqtgen1::QObjectEnabledCxxQtSignalHandlersignal_disabled");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::QObjectEnabled>) + Send;
}
#[cfg(not(enabled))]
use core::mem::drop as drop_QObjectEnabled_signal_handler_signal_disabled;
#[cfg(not(enabled))]
fn call_QObjectEnabled_signal_handler_signal_disabled(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectEnabledCxxQtSignalClosuresignal_disabled,
    >,
    self_value: core::pin::Pin<&mut ffi::QObjectEnabled>,
) {
    handler.closure()(self_value);
}
#[cfg(not(enabled))]
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<QObjectEnabledCxxQtSignalClosuresignal_disabled>,
    usize
);
#[cfg(not(enabled))]
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<QObjectEnabledCxxQtSignalClosuresignal_disabled>,
    [usize; 2]
);
#[cfg(enabled)]
impl ffi::QObjectEnabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_enabled"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_signal_enabled<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectEnabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectEnabled>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QObjectEnabled_connect_signal_enabled(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                QObjectEnabledCxxQtSignalClosuresignal_enabled,
            >::new(Box::new(closure)),
            conn_type,
        ))
    }
}
#[cfg(enabled)]
impl ffi::QObjectEnabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_enabled"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_signal_enabled<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectEnabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectEnabled>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QObjectEnabled_connect_signal_enabled(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                QObjectEnabledCxxQtSignalClosuresignal_enabled,
            >::new(Box::new(closure)),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[cfg(enabled)]
#[doc(hidden)]
pub struct QObjectEnabledCxxQtSignalClosuresignal_enabled {}
#[cfg(enabled)]
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for QObjectEnabledCxxQtSignalClosuresignal_enabled
{
    type Id = cxx::type_id!("::rust::cxxqtgen1::QObjectEnabledCxxQtSignalHandlersignal_enabled");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::QObjectEnabled>) + Send;
}
#[cfg(enabled)]
use core::mem::drop as drop_QObjectEnabled_signal_handler_signal_enabled;
#[cfg(enabled)]
fn call_QObjectEnabled_signal_handler_signal_enabled(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectEnabledCxxQtSignalClosuresignal_enabled,
    >,
    self_value: core::pin::Pin<&mut ffi::QObjectEnabled>,
) {
    handler.closure()(self_value);
}
#[cfg(enabled)]
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<QObjectEnabledCxxQtSignalClosuresignal_enabled>,
    usize
);
#[cfg(enabled)]
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<QObjectEnabledCxxQtSignalClosuresignal_enabled>,
    [usize; 2]
);
#[doc(hidden)]
#[allow(clippy::unnecessary_box_returns)]
#[cfg(enabled)]
pub fn create_rs_QObjectEnabledRust() -> std::boxed::Box<QObjectEnabledRust> {
    std::boxed::Box::new(core::default::Default::default())
}
#[cfg(enabled)]
impl ::core::ops::Deref for ffi::QObjectEnabled {
    type Target = QObjectEnabledRust;
    fn deref(&self) -> &Self::Target {
        ffi::cxx_qt_ffi_QObjectEnabled_unsafeRust(self)
    }
}
#[cfg(enabled)]
impl ::cxx_qt::CxxQtType for ffi::QObjectEnabled {
    type Rust = QObjectEnabledRust;
    fn rust(&self) -> &Self::Rust {
        ffi::cxx_qt_ffi_QObjectEnabled_unsafeRust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        ffi::cxx_qt_ffi_QObjectEnabled_unsafeRustMut(self)
    }
}
#[cfg(not(enabled))]
impl ffi::QObjectDisabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_disabled"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_signal_disabled<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectDisabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectDisabled>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QObjectDisabled_connect_signal_disabled(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                QObjectDisabledCxxQtSignalClosuresignal_disabled,
            >::new(Box::new(closure)),
            conn_type,
        ))
    }
}
#[cfg(not(enabled))]
impl ffi::QObjectDisabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_disabled"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_signal_disabled<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectDisabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectDisabled>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QObjectDisabled_connect_signal_disabled(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                QObjectDisabledCxxQtSignalClosuresignal_disabled,
            >::new(Box::new(closure)),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[cfg(not(enabled))]
#[doc(hidden)]
pub struct QObjectDisabledCxxQtSignalClosuresignal_disabled {}
#[cfg(not(enabled))]
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for QObjectDisabledCxxQtSignalClosuresignal_disabled
{
    type Id = cxx::type_id!("::rust::cxxqtgen1::QObjectDisabledCxxQtSignalHandlersignal_disabled");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::QObjectDisabled>) + Send;
}
#[cfg(not(enabled))]
use core::mem::drop as drop_QObjectDisabled_signal_handler_signal_disabled;
#[cfg(not(enabled))]
fn call_QObjectDisabled_signal_handler_signal_disabled(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectDisabledCxxQtSignalClosuresignal_disabled,
    >,
    self_value: core::pin::Pin<&mut ffi::QObjectDisabled>,
) {
    handler.closure()(self_value);
}
#[cfg(not(enabled))]
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<QObjectDisabledCxxQtSignalClosuresignal_disabled>,
    usize
);
#[cfg(not(enabled))]
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<QObjectDisabledCxxQtSignalClosuresignal_disabled>,
    [usize; 2]
);
#[cfg(enabled)]
impl ffi::QObjectDisabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_enabled"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_signal_enabled<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectDisabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectDisabled>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QObjectDisabled_connect_signal_enabled(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                QObjectDisabledCxxQtSignalClosuresignal_enabled,
            >::new(Box::new(closure)),
            conn_type,
        ))
    }
}
#[cfg(enabled)]
impl ffi::QObjectDisabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_enabled"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_signal_enabled<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectDisabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectDisabled>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QObjectDisabled_connect_signal_enabled(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                QObjectDisabledCxxQtSignalClosuresignal_enabled,
            >::new(Box::new(closure)),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[cfg(enabled)]
#[doc(hidden)]
pub struct QObjectDisabledCxxQtSignalClosuresignal_enabled {}
#[cfg(enabled)]
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for QObjectDisabledCxxQtSignalClosuresignal_enabled
{
    type Id = cxx::type_id!("::rust::cxxqtgen1::QObjectDisabledCxxQtSignalHandlersignal_enabled");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::QObjectDisabled>) + Send;
}
#[cfg(enabled)]
use core::mem::drop as drop_QObjectDisabled_signal_handler_signal_enabled;
#[cfg(enabled)]
fn call_QObjectDisabled_signal_handler_signal_enabled(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectDisabledCxxQtSignalClosuresignal_enabled,
    >,
    self_value: core::pin::Pin<&mut ffi::QObjectDisabled>,
) {
    handler.closure()(self_value);
}
#[cfg(enabled)]
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<QObjectDisabledCxxQtSignalClosuresignal_enabled>,
    usize
);
#[cfg(enabled)]
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<QObjectDisabledCxxQtSignalClosuresignal_enabled>,
    [usize; 2]
);
#[doc(hidden)]
#[allow(clippy::unnecessary_box_returns)]
#[cfg(not(enabled))]
pub fn create_rs_QObjectDisabledRust() -> std::boxed::Box<QObjectDisabledRust> {
    std::boxed::Box::new(core::default::Default::default())
}
#[cfg(not(enabled))]
impl ::core::ops::Deref for ffi::QObjectDisabled {
    type Target = QObjectDisabledRust;
    fn deref(&self) -> &Self::Target {
        ffi::cxx_qt_ffi_QObjectDisabled_unsafeRust(self)
    }
}
#[cfg(not(enabled))]
impl ::cxx_qt::CxxQtType for ffi::QObjectDisabled {
    type Rust = QObjectDisabledRust;
    fn rust(&self) -> &Self::Rust {
        ffi::cxx_qt_ffi_QObjectDisabled_unsafeRust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        ffi::cxx_qt_ffi_QObjectDisabled_unsafeRustMut(self)
    }
}
#[cfg(not(enabled))]
impl ffi::QObjectExternEnabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_disabled1"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_signal_disabled1<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectExternEnabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectExternEnabled>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::QObjectExternEnabled_connect_signal_disabled1(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    QObjectExternEnabledCxxQtSignalClosuresignal_disabled1,
                >::new(Box::new(closure)),
                conn_type,
            ),
        )
    }
}
#[cfg(not(enabled))]
impl ffi::QObjectExternEnabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_disabled1"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_signal_disabled1<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectExternEnabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectExternEnabled>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::QObjectExternEnabled_connect_signal_disabled1(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    QObjectExternEnabledCxxQtSignalClosuresignal_disabled1,
                >::new(Box::new(closure)),
                cxx_qt::ConnectionType::AutoConnection,
            ),
        )
    }
}
#[cfg(not(enabled))]
#[doc(hidden)]
pub struct QObjectExternEnabledCxxQtSignalClosuresignal_disabled1 {}
#[cfg(not(enabled))]
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for QObjectExternEnabledCxxQtSignalClosuresignal_disabled1
{
    type Id =
        cxx::type_id!("::rust::cxxqtgen1::QObjectExternEnabledCxxQtSignalHandlersignal_disabled1");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::QObjectExternEnabled>) + Send;
}
#[cfg(not(enabled))]
use core::mem::drop as drop_QObjectExternEnabled_signal_handler_signal_disabled1;
#[cfg(not(enabled))]
fn call_QObjectExternEnabled_signal_handler_signal_disabled1(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectExternEnabledCxxQtSignalClosuresignal_disabled1,
    >,
    self_value: core::pin::Pin<&mut ffi::QObjectExternEnabled>,
) {
    handler.closure()(self_value);
}
#[cfg(not(enabled))]
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectExternEnabledCxxQtSignalClosuresignal_disabled1,
    >,
    usize
);
#[cfg(not(enabled))]
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectExternEnabledCxxQtSignalClosuresignal_disabled1,
    >,
    [usize; 2]
);
#[cfg(enabled)]
impl ffi::QObjectExternEnabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_enabled1"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_signal_enabled1<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectExternEnabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectExternEnabled>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QObjectExternEnabled_connect_signal_enabled1(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                QObjectExternEnabledCxxQtSignalClosuresignal_enabled1,
            >::new(Box::new(closure)),
            conn_type,
        ))
    }
}
#[cfg(enabled)]
impl ffi::QObjectExternEnabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_enabled1"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_signal_enabled1<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectExternEnabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectExternEnabled>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::QObjectExternEnabled_connect_signal_enabled1(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                QObjectExternEnabledCxxQtSignalClosuresignal_enabled1,
            >::new(Box::new(closure)),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[cfg(enabled)]
#[doc(hidden)]
pub struct QObjectExternEnabledCxxQtSignalClosuresignal_enabled1 {}
#[cfg(enabled)]
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for QObjectExternEnabledCxxQtSignalClosuresignal_enabled1
{
    type Id =
        cxx::type_id!("::rust::cxxqtgen1::QObjectExternEnabledCxxQtSignalHandlersignal_enabled1");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::QObjectExternEnabled>) + Send;
}
#[cfg(enabled)]
use core::mem::drop as drop_QObjectExternEnabled_signal_handler_signal_enabled1;
#[cfg(enabled)]
fn call_QObjectExternEnabled_signal_handler_signal_enabled1(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectExternEnabledCxxQtSignalClosuresignal_enabled1,
    >,
    self_value: core::pin::Pin<&mut ffi::QObjectExternEnabled>,
) {
    handler.closure()(self_value);
}
#[cfg(enabled)]
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectExternEnabledCxxQtSignalClosuresignal_enabled1,
    >,
    usize
);
#[cfg(enabled)]
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectExternEnabledCxxQtSignalClosuresignal_enabled1,
    >,
    [usize; 2]
);
#[cfg(not(enabled))]
impl ffi::QObjectExternDisabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_disabled2"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_signal_disabled2<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectExternDisabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectExternDisabled>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::QObjectExternDisabled_connect_signal_disabled2(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    QObjectExternDisabledCxxQtSignalClosuresignal_disabled2,
                >::new(Box::new(closure)),
                conn_type,
            ),
        )
    }
}
#[cfg(not(enabled))]
impl ffi::QObjectExternDisabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_disabled2"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_signal_disabled2<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectExternDisabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectExternDisabled>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::QObjectExternDisabled_connect_signal_disabled2(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    QObjectExternDisabledCxxQtSignalClosuresignal_disabled2,
                >::new(Box::new(closure)),
                cxx_qt::ConnectionType::AutoConnection,
            ),
        )
    }
}
#[cfg(not(enabled))]
#[doc(hidden)]
pub struct QObjectExternDisabledCxxQtSignalClosuresignal_disabled2 {}
#[cfg(not(enabled))]
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for QObjectExternDisabledCxxQtSignalClosuresignal_disabled2
{
    type Id =
        cxx::type_id!("::rust::cxxqtgen1::QObjectExternDisabledCxxQtSignalHandlersignal_disabled2");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::QObjectExternDisabled>) + Send;
}
#[cfg(not(enabled))]
use core::mem::drop as drop_QObjectExternDisabled_signal_handler_signal_disabled2;
#[cfg(not(enabled))]
fn call_QObjectExternDisabled_signal_handler_signal_disabled2(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectExternDisabledCxxQtSignalClosuresignal_disabled2,
    >,
    self_value: core::pin::Pin<&mut ffi::QObjectExternDisabled>,
) {
    handler.closure()(self_value);
}
#[cfg(not(enabled))]
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectExternDisabledCxxQtSignalClosuresignal_disabled2,
    >,
    usize
);
#[cfg(not(enabled))]
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectExternDisabledCxxQtSignalClosuresignal_disabled2,
    >,
    [usize; 2]
);
#[cfg(enabled)]
impl ffi::QObjectExternDisabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_enabled2"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_signal_enabled2<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectExternDisabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectExternDisabled>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::QObjectExternDisabled_connect_signal_enabled2(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    QObjectExternDisabledCxxQtSignalClosuresignal_enabled2,
                >::new(Box::new(closure)),
                conn_type,
            ),
        )
    }
}
#[cfg(enabled)]
impl ffi::QObjectExternDisabled {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "signal_enabled2"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_signal_enabled2<
        F: FnMut(core::pin::Pin<&mut ffi::QObjectExternDisabled>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::QObjectExternDisabled>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::QObjectExternDisabled_connect_signal_enabled2(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    QObjectExternDisabledCxxQtSignalClosuresignal_enabled2,
                >::new(Box::new(closure)),
                cxx_qt::ConnectionType::AutoConnection,
            ),
        )
    }
}
#[cfg(enabled)]
#[doc(hidden)]
pub struct QObjectExternDisabledCxxQtSignalClosuresignal_enabled2 {}
#[cfg(enabled)]
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for QObjectExternDisabledCxxQtSignalClosuresignal_enabled2
{
    type Id =
        cxx::type_id!("::rust::cxxqtgen1::QObjectExternDisabledCxxQtSignalHandlersignal_enabled2");
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::QObjectExternDisabled>) + Send;
}
#[cfg(enabled)]
use core::mem::drop as drop_QObjectExternDisabled_signal_handler_signal_enabled2;
#[cfg(enabled)]
fn call_QObjectExternDisabled_signal_handler_signal_enabled2(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectExternDisabledCxxQtSignalClosuresignal_enabled2,
    >,
    self_value: core::pin::Pin<&mut ffi::QObjectExternDisabled>,
) {
    handler.closure()(self_value);
}
#[cfg(enabled)]
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectExternDisabledCxxQtSignalClosuresignal_enabled2,
    >,
    usize
);
#[cfg(enabled)]
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<
        QObjectExternDisabledCxxQtSignalClosuresignal_enabled2,
    >,
    [usize; 2]
);
