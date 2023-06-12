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
        #[cxx_name = "getPrimitive"]
        unsafe fn primitive<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPrimitive"]
        fn set_primitive(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i32);
    }
    extern "Rust" {
        #[cxx_name = "getTrivial"]
        unsafe fn trivial<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a QPoint;
    }
    extern "Rust" {
        #[cxx_name = "setTrivial"]
        fn set_trivial(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: QPoint);
    }
    unsafe extern "C++" {
        #[doc = "Notify for the Q_PROPERTY"]
        #[rust_name = "primitive_changed"]
        fn primitiveChanged(self: Pin<&mut MyObjectQt>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "primitiveChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_primitive_changed"]
        fn primitiveChangedConnect(
            self: Pin<&mut MyObjectQt>,
            func: fn(Pin<&mut MyObjectQt>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    unsafe extern "C++" {
        #[doc = "Notify for the Q_PROPERTY"]
        #[rust_name = "trivial_changed"]
        fn trivialChanged(self: Pin<&mut MyObjectQt>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "trivialChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_trivial_changed"]
        fn trivialChangedConnect(
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
    extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        #[doc(hidden)]
        unsafe fn cxx_qt_ffi_rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn create_rs_my_object() -> Box<MyObject>;
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
    #[derive(Default)]
    pub struct MyObject {
        primitive: i32,
        trivial: QPoint,
        opaque: UniquePtr<Opaque>,
        private_rust_field: i32,
        pub public_rust_field: f64,
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn primitive<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
            cpp.primitive()
        }
    }
    impl MyObjectQt {
        #[doc = "Getter for the Q_PROPERTY "]
        #[doc = "primitive"]
        pub fn primitive(&self) -> &i32 {
            &self.rust().primitive
        }
    }
    impl MyObjectQt {
        #[doc = "unsafe getter for the Q_PROPERTY "]
        #[doc = "primitive"]
        #[doc = "\n"]
        #[doc = "This allows for modifying the Q_PROPERTY without calling the property changed Q_SIGNAL"]
        #[doc = "\n"]
        #[doc = "After modifying the property, make sure to call the corresponding changed signal: "]
        #[doc = "primitive_changed"]
        pub unsafe fn primitive_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut i32 {
            &mut self.rust_mut().get_unchecked_mut().primitive
        }
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn set_primitive(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
            cpp.set_primitive(value);
        }
    }
    impl MyObjectQt {
        #[doc = "Setter for the Q_PROPERTY "]
        #[doc = "primitive"]
        pub fn set_primitive(mut self: Pin<&mut Self>, value: i32) {
            if self.rust().primitive == value {
                return;
            }
            unsafe {
                self.as_mut().rust_mut().primitive = value;
            }
            self.as_mut().primitive_changed();
        }
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn trivial<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a QPoint {
            cpp.trivial()
        }
    }
    impl MyObjectQt {
        #[doc = "Getter for the Q_PROPERTY "]
        #[doc = "trivial"]
        pub fn trivial(&self) -> &QPoint {
            &self.rust().trivial
        }
    }
    impl MyObjectQt {
        #[doc = "unsafe getter for the Q_PROPERTY "]
        #[doc = "trivial"]
        #[doc = "\n"]
        #[doc = "This allows for modifying the Q_PROPERTY without calling the property changed Q_SIGNAL"]
        #[doc = "\n"]
        #[doc = "After modifying the property, make sure to call the corresponding changed signal: "]
        #[doc = "trivial_changed"]
        pub unsafe fn trivial_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut QPoint {
            &mut self.rust_mut().get_unchecked_mut().trivial
        }
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn set_trivial(&mut self, cpp: Pin<&mut MyObjectQt>, value: QPoint) {
            cpp.set_trivial(value);
        }
    }
    impl MyObjectQt {
        #[doc = "Setter for the Q_PROPERTY "]
        #[doc = "trivial"]
        pub fn set_trivial(mut self: Pin<&mut Self>, value: QPoint) {
            if self.rust().trivial == value {
                return;
            }
            unsafe {
                self.as_mut().rust_mut().trivial = value;
            }
            self.as_mut().trivial_changed();
        }
    }
    impl MyObjectQt {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "primitiveChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_primitive_changed(
            self: Pin<&mut MyObjectQt>,
            func: fn(Pin<&mut MyObjectQt>),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_primitive_changed(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl MyObjectQt {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "trivialChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_trivial_changed(
            self: Pin<&mut MyObjectQt>,
            func: fn(Pin<&mut MyObjectQt>),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_trivial_changed(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl cxx_qt::Locking for MyObjectQt {}
    impl cxx_qt::CxxQtType for MyObjectQt {
        type Rust = MyObject;
        fn rust(&self) -> &Self::Rust {
            self.cxx_qt_ffi_rust()
        }
        unsafe fn rust_mut(self: core::pin::Pin<&mut Self>) -> Pin<&mut Self::Rust> {
            self.cxx_qt_ffi_rust_mut()
        }
    }
    #[doc = r" Generated CXX-Qt method which creates a boxed rust struct of a QObject"]
    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
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
    }
}
