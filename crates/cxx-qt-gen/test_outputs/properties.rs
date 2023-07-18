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
        #[cxx_name = "getPrimitive"]
        unsafe fn primitive<'a>(self: &'a MyObjectRust, cpp: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPrimitive"]
        fn set_primitive(self: &mut MyObjectRust, cpp: Pin<&mut MyObject>, value: i32);
    }
    extern "Rust" {
        #[cxx_name = "getTrivial"]
        unsafe fn trivial<'a>(self: &'a MyObjectRust, cpp: &'a MyObject) -> &'a QPoint;
    }
    extern "Rust" {
        #[cxx_name = "setTrivial"]
        fn set_trivial(self: &mut MyObjectRust, cpp: Pin<&mut MyObject>, value: QPoint);
    }
    unsafe extern "C++" {
        #[cxx_name = "primitiveChanged"]
        #[doc = "Notify for the Q_PROPERTY"]
        fn primitive_changed(self: Pin<&mut MyObject>);
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
        #[cxx_name = "trivialChanged"]
        #[doc = "Notify for the Q_PROPERTY"]
        fn trivial_changed(self: Pin<&mut MyObject>);
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
use self::cxx_qt_ffi::*;
#[doc = r" Internal CXX-Qt module, made public temporarily between API changes"]
pub mod cxx_qt_ffi {
    use super::ffi::*;
    use cxx_qt::CxxQtType;
    use std::pin::Pin;
    #[doc(hidden)]
    type UniquePtr<T> = cxx::UniquePtr<T>;
    type MyObjectRust = super::MyObjectRust;
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn primitive<'a>(&'a self, cpp: &'a MyObject) -> &'a i32 {
            cpp.primitive()
        }
    }
    impl MyObject {
        #[doc = "Getter for the Q_PROPERTY "]
        #[doc = "primitive"]
        pub fn primitive(&self) -> &i32 {
            &self.rust().primitive
        }
    }
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn set_primitive(&mut self, cpp: Pin<&mut MyObject>, value: i32) {
            cpp.set_primitive(value);
        }
    }
    impl MyObject {
        #[doc = "Setter for the Q_PROPERTY "]
        #[doc = "primitive"]
        pub fn set_primitive(mut self: Pin<&mut Self>, value: i32) {
            if self.rust().primitive == value {
                return;
            }
            self.as_mut().rust_mut().primitive = value;
            self.as_mut().primitive_changed();
        }
    }
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn trivial<'a>(&'a self, cpp: &'a MyObject) -> &'a QPoint {
            cpp.trivial()
        }
    }
    impl MyObject {
        #[doc = "Getter for the Q_PROPERTY "]
        #[doc = "trivial"]
        pub fn trivial(&self) -> &QPoint {
            &self.rust().trivial
        }
    }
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn set_trivial(&mut self, cpp: Pin<&mut MyObject>, value: QPoint) {
            cpp.set_trivial(value);
        }
    }
    impl MyObject {
        #[doc = "Setter for the Q_PROPERTY "]
        #[doc = "trivial"]
        pub fn set_trivial(mut self: Pin<&mut Self>, value: QPoint) {
            if self.rust().trivial == value {
                return;
            }
            self.as_mut().rust_mut().trivial = value;
            self.as_mut().trivial_changed();
        }
    }
    impl MyObject {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "primitiveChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_primitive_changed(
            self: Pin<&mut MyObject>,
            func: fn(Pin<&mut MyObject>),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_primitive_changed(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl MyObject {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "trivialChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_trivial_changed(
            self: Pin<&mut MyObject>,
            func: fn(Pin<&mut MyObject>),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_trivial_changed(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl cxx_qt::Locking for MyObject {}
    #[doc = r" Generated CXX-Qt method which creates a boxed rust struct of a QObject"]
    pub fn create_rs_my_object_rust() -> std::boxed::Box<MyObjectRust> {
        core::default::Default::default()
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
}
