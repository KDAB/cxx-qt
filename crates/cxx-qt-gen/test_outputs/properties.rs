#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
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
        include!("cxx-qt-gen/ffi.cxxqt.h");
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
        #[rust_name = "primitive_changed"]
        fn primitiveChanged(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "primitiveChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_primitive_changed"]
        fn primitiveChangedConnect(
            self: Pin<&mut MyObject>,
            func: fn(Pin<&mut MyObject>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    unsafe extern "C++" {
        #[doc = "Notify for the Q_PROPERTY"]
        #[rust_name = "trivial_changed"]
        fn trivialChanged(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "trivialChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_trivial_changed"]
        fn trivialChangedConnect(
            self: Pin<&mut MyObject>,
            func: fn(Pin<&mut MyObject>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
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
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    #[must_use]
    pub fn on_primitive_changed(
        self: core::pin::Pin<&mut ffi::MyObject>,
        func: fn(core::pin::Pin<&mut ffi::MyObject>),
    ) -> cxx_qt_lib::QMetaObjectConnection {
        self.connect_primitive_changed(func, cxx_qt_lib::ConnectionType::AutoConnection)
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "trivialChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    #[must_use]
    pub fn on_trivial_changed(
        self: core::pin::Pin<&mut ffi::MyObject>,
        func: fn(core::pin::Pin<&mut ffi::MyObject>),
    ) -> cxx_qt_lib::QMetaObjectConnection {
        self.connect_trivial_changed(func, cxx_qt_lib::ConnectionType::AutoConnection)
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
