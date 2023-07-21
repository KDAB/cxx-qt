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
        #[doc(hidden)]
        #[cxx_name = "invokableWrapper"]
        fn invokable(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[rust_name = "ready"]
        fn ready(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "ready"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_ready"]
        fn readyConnect(
            self: Pin<&mut MyObject>,
            func: fn(Pin<&mut MyObject>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    unsafe extern "C++" {
        #[rust_name = "data_changed"]
        fn dataChanged(
            self: Pin<&mut MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        );
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "dataChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_data_changed"]
        fn dataChangedConnect(
            self: Pin<&mut MyObject>,
            func: fn(
                Pin<&mut MyObject>,
                first: i32,
                second: UniquePtr<Opaque>,
                third: QPoint,
                fourth: &'a QPoint,
            ),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    unsafe extern "C++" {
        #[rust_name = "base_class_new_data"]
        fn newData(
            self: Pin<&mut MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        );
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "newData"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_base_class_new_data"]
        fn newDataConnect(
            self: Pin<&mut MyObject>,
            func: fn(
                Pin<&mut MyObject>,
                first: i32,
                second: UniquePtr<Opaque>,
                third: QPoint,
                fourth: &'a QPoint,
            ),
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
    #[doc(hidden)]
    type UniquePtr<T> = cxx::UniquePtr<T>;
    type MyObjectRust = super::MyObjectRust;
    impl MyObject {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "ready"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_ready(
            self: core::pin::Pin<&mut MyObject>,
            func: fn(core::pin::Pin<&mut MyObject>),
        ) -> cxx_qt_lib::QMetaObjectConnection {
            self.connect_ready(func, cxx_qt_lib::ConnectionType::AutoConnection)
        }
    }
    impl MyObject {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "dataChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_data_changed(
            self: core::pin::Pin<&mut MyObject>,
            func: fn(
                core::pin::Pin<&mut MyObject>,
                first: i32,
                second: cxx::UniquePtr<Opaque>,
                third: QPoint,
                fourth: &'a QPoint,
            ),
        ) -> cxx_qt_lib::QMetaObjectConnection {
            self.connect_data_changed(func, cxx_qt_lib::ConnectionType::AutoConnection)
        }
    }
    impl MyObject {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "newData"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_base_class_new_data(
            self: core::pin::Pin<&mut MyObject>,
            func: fn(
                core::pin::Pin<&mut MyObject>,
                first: i32,
                second: cxx::UniquePtr<Opaque>,
                third: QPoint,
                fourth: &'a QPoint,
            ),
        ) -> cxx_qt_lib::QMetaObjectConnection {
            self.connect_base_class_new_data(func, cxx_qt_lib::ConnectionType::AutoConnection)
        }
    }
    impl cxx_qt::Locking for MyObject {}
    #[doc(hidden)]
    pub fn create_rs_my_object_rust() -> std::boxed::Box<MyObjectRust> {
        std::boxed::Box::new(core::default::Default::default())
    }
    impl core::ops::Deref for MyObject {
        type Target = MyObjectRust;
        fn deref(&self) -> &Self::Target {
            self.cxx_qt_ffi_rust()
        }
    }
    impl cxx_qt::CxxQtType for MyObject {
        type Rust = MyObjectRust;
        fn rust(&self) -> &Self::Rust {
            self.cxx_qt_ffi_rust()
        }
        fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
            self.cxx_qt_ffi_rust_mut()
        }
    }
}
