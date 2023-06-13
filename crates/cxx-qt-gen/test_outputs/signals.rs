#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }
    impl qobject::MyObject {
        #[qinvokable]
        pub fn invokable(self: Pin<&mut Self>) {
            self.as_mut().on_data_changed(
                |_sender, _first, _second, _third, _fourth| {
                    println!("DataChanged");
                },
                cxx_qt_lib::ConnectionType::AutoConnection,
            );
            self.as_mut()
                .data_changed(1, Opaque::new(), QPoint::new(1, 2), &QPoint::new(1, 2));
        }
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
    unsafe extern "C++" {
        #[rust_name = "ready"]
        fn ready(self: Pin<&mut MyObjectQt>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "ready"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "connect_ready"]
        fn readyConnect(
            self: Pin<&mut MyObjectQt>,
            func: fn(Pin<&mut MyObjectQt>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    unsafe extern "C++" {
        #[rust_name = "data_changed"]
        fn dataChanged(
            self: Pin<&mut MyObjectQt>,
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
            self: Pin<&mut MyObjectQt>,
            func: fn(
                Pin<&mut MyObjectQt>,
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
            self: Pin<&mut MyObjectQt>,
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
            self: Pin<&mut MyObjectQt>,
            func: fn(
                Pin<&mut MyObjectQt>,
                first: i32,
                second: UniquePtr<Opaque>,
                third: QPoint,
                fourth: &'a QPoint,
            ),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRust"]
        #[doc(hidden)]
        fn cxx_qt_ffi_rust(self: &MyObjectQt) -> &MyObject;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        #[doc(hidden)]
        fn cxx_qt_ffi_rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
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
    pub struct MyObject;
    impl MyObjectQt {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "ready"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_ready(
            self: Pin<&mut MyObjectQt>,
            func: fn(Pin<&mut MyObjectQt>),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_ready(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl MyObjectQt {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "dataChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_data_changed(
            self: Pin<&mut MyObjectQt>,
            func: fn(
                Pin<&mut MyObjectQt>,
                first: i32,
                second: UniquePtr<Opaque>,
                third: QPoint,
                fourth: &'a QPoint,
            ),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_data_changed(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl MyObjectQt {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "newData"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[doc = "\n"]
        #[doc = "Note that this method uses a AutoConnection connection type."]
        #[must_use]
        pub fn on_base_class_new_data(
            self: Pin<&mut MyObjectQt>,
            func: fn(
                Pin<&mut MyObjectQt>,
                first: i32,
                second: UniquePtr<Opaque>,
                third: QPoint,
                fourth: &'a QPoint,
            ),
        ) -> CxxQtQMetaObjectConnection {
            self.connect_base_class_new_data(func, CxxQtConnectionType::AutoConnection)
        }
    }
    impl cxx_qt::Locking for MyObjectQt {}
    impl cxx_qt::CxxQtType for MyObjectQt {
        type Rust = MyObject;
        fn rust(&self) -> &Self::Rust {
            self.cxx_qt_ffi_rust()
        }
        fn rust_mut(self: core::pin::Pin<&mut Self>) -> Pin<&mut Self::Rust> {
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
