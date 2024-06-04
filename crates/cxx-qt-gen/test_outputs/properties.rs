#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }
    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt/connection.h");
        #[doc(hidden)]
        #[namespace = "Qt"]
        #[rust_name = "CxxQtConnectionType"]
        type ConnectionType = cxx_qt::ConnectionType;
        #[doc(hidden)]
        #[namespace = "rust::cxxqt1"]
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
        #[cxx_name = "getPrimitiveWrapper"]
        unsafe fn primitive<'a>(self: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPrimitiveWrapper"]
        fn set_primitive(self: Pin<&mut MyObject>, value: i32);
    }
    extern "Rust" {
        #[cxx_name = "getTrivialWrapper"]
        unsafe fn trivial<'a>(self: &'a MyObject) -> &'a QPoint;
    }
    extern "Rust" {
        #[cxx_name = "setTrivialWrapper"]
        fn set_trivial(self: Pin<&mut MyObject>, value: QPoint);
    }
    unsafe extern "C++" {
        #[doc = "Notify for the Q_PROPERTY"]
        #[cxx_name = "primitiveChanged"]
        fn primitive_changed(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlerprimitiveChanged = cxx_qt::signalhandler::CxxQtSignalHandler<
            super::MyObjectCxxQtSignalClosureprimitiveChanged,
        >;
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_primitiveChangedConnect"]
        fn MyObject_connect_primitive_changed(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlerprimitiveChanged,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_primitiveChanged(
            handler: MyObjectCxxQtSignalHandlerprimitiveChanged,
        );
        #[doc(hidden)]
        fn call_MyObject_signal_handler_primitiveChanged(
            handler: &mut MyObjectCxxQtSignalHandlerprimitiveChanged,
            self_value: Pin<&mut MyObject>,
        );
    }
    unsafe extern "C++" {
        #[doc = "Notify for the Q_PROPERTY"]
        #[cxx_name = "trivialChanged"]
        fn trivial_changed(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlertrivialChanged = cxx_qt::signalhandler::CxxQtSignalHandler<
            super::MyObjectCxxQtSignalClosuretrivialChanged,
        >;
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_trivialChangedConnect"]
        fn MyObject_connect_trivial_changed(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlertrivialChanged,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_trivialChanged(
            handler: MyObjectCxxQtSignalHandlertrivialChanged,
        );
        #[doc(hidden)]
        fn call_MyObject_signal_handler_trivialChanged(
            handler: &mut MyObjectCxxQtSignalHandlertrivialChanged,
            self_value: Pin<&mut MyObject>,
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
}
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "primitive"]
    pub fn primitive(&self) -> &i32 {
        &self.primitive
    }
}
impl ffi::MyObject {
    #[doc = "Setter for the Q_PROPERTY "]
    #[doc = "primitive"]
    pub fn set_primitive(mut self: core::pin::Pin<&mut Self>, value: i32) {
        use cxx_qt::CxxQtType;
        if self.primitive == value {
            return;
        }
        self.as_mut().rust_mut().primitive = value;
        self.as_mut().primitive_changed();
    }
}
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "trivial"]
    pub fn trivial(&self) -> &ffi::QPoint {
        &self.trivial
    }
}
impl ffi::MyObject {
    #[doc = "Setter for the Q_PROPERTY "]
    #[doc = "trivial"]
    pub fn set_trivial(mut self: core::pin::Pin<&mut Self>, value: ffi::QPoint) {
        use cxx_qt::CxxQtType;
        if self.trivial == value {
            return;
        }
        self.as_mut().rust_mut().trivial = value;
        self.as_mut().trivial_changed();
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "primitiveChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_primitive_changed<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::MyObject_connect_primitive_changed(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    MyObjectCxxQtSignalClosureprimitiveChanged,
                >::new(Box::new(closure)),
                conn_type,
            ),
        )
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "primitiveChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_primitive_changed<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::MyObject_connect_primitive_changed(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    MyObjectCxxQtSignalClosureprimitiveChanged,
                >::new(Box::new(closure)),
                cxx_qt::ConnectionType::AutoConnection,
            ),
        )
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosureprimitiveChanged {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for MyObjectCxxQtSignalClosureprimitiveChanged
{
    type Id = cxx::type_id!(
        "::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerprimitiveChanged"
    );
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>);
}
use core::mem::drop as drop_MyObject_signal_handler_primitiveChanged;
fn call_MyObject_signal_handler_primitiveChanged(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        MyObjectCxxQtSignalClosureprimitiveChanged,
    >,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureprimitiveChanged>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureprimitiveChanged>,
    [usize; 2]
);
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "trivialChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_trivial_changed<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt :: QMetaObjectConnectionGuard :: from (ffi :: MyObject_connect_trivial_changed (self , cxx_qt :: signalhandler :: CxxQtSignalHandler :: < MyObjectCxxQtSignalClosuretrivialChanged > :: new (Box :: new (closure)) , conn_type ,))
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "trivialChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_trivial_changed<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt :: QMetaObjectConnectionGuard :: from (ffi :: MyObject_connect_trivial_changed (self , cxx_qt :: signalhandler :: CxxQtSignalHandler :: < MyObjectCxxQtSignalClosuretrivialChanged > :: new (Box :: new (closure)) , cxx_qt :: ConnectionType :: AutoConnection ,))
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosuretrivialChanged {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosuretrivialChanged {
    type Id = cxx::type_id!(
        "::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlertrivialChanged"
    );
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>);
}
use core::mem::drop as drop_MyObject_signal_handler_trivialChanged;
fn call_MyObject_signal_handler_trivialChanged(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        MyObjectCxxQtSignalClosuretrivialChanged,
    >,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuretrivialChanged>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuretrivialChanged>,
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
