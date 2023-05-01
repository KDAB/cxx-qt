#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }
    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/convert.h");
        include!("cxx-qt-lib/cxxqt_thread.h");
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
        #[cxx_name = "invokableWrapper"]
        fn invokable_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "emit_ready"]
        fn emitReady(self: Pin<&mut MyObjectQt>);
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "ready"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "on_ready"]
        fn readyConnect(
            self: Pin<&mut MyObjectQt>,
            func: fn(Pin<&mut MyObjectQt>),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "emit_data_changed"]
        fn emitDataChanged(
            self: Pin<&mut MyObjectQt>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &QPoint,
        );
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "dataChanged"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "on_data_changed"]
        fn dataChangedConnect(
            self: Pin<&mut MyObjectQt>,
            func: fn(
                Pin<&mut MyObjectQt>,
                first: i32,
                second: UniquePtr<Opaque>,
                third: QPoint,
                fourth: &QPoint,
            ),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "emit_new_data"]
        fn emitNewData(
            self: Pin<&mut MyObjectQt>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &QPoint,
        );
    }
    unsafe extern "C++" {
        #[doc = "Connect the given function pointer to the signal "]
        #[doc = "newData"]
        #[doc = ", so that when the signal is emitted the function pointer is executed."]
        #[must_use]
        #[rust_name = "on_new_data"]
        fn newDataConnect(
            self: Pin<&mut MyObjectQt>,
            func: fn(
                Pin<&mut MyObjectQt>,
                first: i32,
                second: UniquePtr<Opaque>,
                third: QPoint,
                fourth: &QPoint,
            ),
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    unsafe extern "C++" {
        #[doc = r" Specialised version of CxxQtThread, which can be moved into other threads."]
        #[doc = r""]
        #[doc = r" CXX doesn't support having generic types in the function yet"]
        #[doc = r" so we cannot have CxxQtThread<T> in cxx-qt-lib and then use that here"]
        #[doc = r" For now we use a type alias in C++ then use it like a normal type here"]
        #[doc = r" <https://github.com/dtolnay/cxx/issues/683>"]
        type MyObjectCxxQtThread;
        #[doc = r" Retrieve an immutable reference to the Rust struct backing this C++ object"]
        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;
        #[doc = r" Create an instance of a CxxQtThread"]
        #[doc = r""]
        #[doc = r" This allows for queueing closures onto the Qt event loop from a background thread."]
        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObjectQt) -> UniquePtr<MyObjectCxxQtThread>;
        #[doc(hidden)]
        #[cxx_name = "queue"]
        fn queue_boxed_fn(
            self: &MyObjectCxxQtThread,
            func: fn(Pin<&mut MyObjectQt>, Box<MyObjectCxxQtThreadQueuedFn>),
            arg: Box<MyObjectCxxQtThreadQueuedFn>,
        ) -> Result<()>;
        #[doc = "Generated CXX-Qt method which creates a new"]
        #[doc = "MyObjectQt"]
        #[doc = "as a UniquePtr with no parent in Qt"]
        #[rust_name = "new_cpp_object_my_object_qt"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }
    extern "C++" {
        #[doc = r" Retrieve a mutable reference to the Rust struct backing this C++ object"]
        #[doc = r""]
        #[doc = r" This method is unsafe because it allows a Q_PROPERTY to be modified without emitting its changed signal."]
        #[doc = r" The property changed signal must be emitted manually."]
        #[cxx_name = "unsafeRustMut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }
    extern "Rust" {
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        type MyObjectCxxQtThreadQueuedFn;
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn create_rs_my_object() -> Box<MyObject>;
    }
}
use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;
    use std::pin::Pin;
    #[doc(hidden)]
    type UniquePtr<T> = cxx::UniquePtr<T>;
    #[derive(Default)]
    pub struct MyObject;
    impl MyObject {
        #[doc(hidden)]
        pub fn invokable_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>) {
            cpp.invokable();
        }
    }
    impl MyObjectQt {
        pub fn invokable(self: Pin<&mut Self>) {
            self.as_mut().on_data_changed(
                |_sender, _first, _second, _third, _fourth| {
                    println!("DataChanged");
                },
                cxx_qt_lib::ConnectionType::AutoConnection,
            );
            self.as_mut().emit(MySignals::DataChanged {
                first: 1,
                second: Opaque::new(),
                third: QPoint::new(1, 2),
                fourth: &QPoint::new(1, 2),
            });
        }
    }
    enum MySignals<'a> {
        Ready,
        DataChanged {
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        },
        BaseClassNewData {
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        },
    }
    impl MyObjectQt {
        #[doc = "Emit the signal from the enum "]
        #[doc = "MySignals"]
        #[doc = " on the QObject "]
        #[doc = "MyObject"]
        pub fn emit(self: Pin<&mut Self>, signal: MySignals) {
            match signal {
                MySignals::Ready {} => self.emit_ready(),
                MySignals::DataChanged {
                    first,
                    second,
                    third,
                    fourth,
                } => self.emit_data_changed(first, second, third, fourth),
                MySignals::BaseClassNewData {
                    first,
                    second,
                    third,
                    fourth,
                } => self.emit_new_data(first, second, third, fourth),
            }
        }
    }
    unsafe impl Send for MyObjectCxxQtThread {}
    impl MyObjectCxxQtThread {
        #[doc = r" Queue the given closure onto the Qt event loop for this QObject"]
        pub fn queue<F>(&self, f: F) -> std::result::Result<(), cxx::Exception>
        where
            F: FnOnce(std::pin::Pin<&mut MyObjectQt>),
            F: Send + 'static,
        {
            #[allow(clippy::boxed_local)]
            #[doc(hidden)]
            fn func(
                obj: std::pin::Pin<&mut MyObjectQt>,
                arg: std::boxed::Box<MyObjectCxxQtThreadQueuedFn>,
            ) {
                (arg.inner)(obj)
            }
            let arg = MyObjectCxxQtThreadQueuedFn {
                inner: std::boxed::Box::new(f),
            };
            self.queue_boxed_fn(func, std::boxed::Box::new(arg))
        }
    }
    #[doc(hidden)]
    pub struct MyObjectCxxQtThreadQueuedFn {
        inner: std::boxed::Box<dyn FnOnce(std::pin::Pin<&mut MyObjectQt>) + Send>,
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
