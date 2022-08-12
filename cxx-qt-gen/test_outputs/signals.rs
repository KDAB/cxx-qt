#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "ready"]
        fn ready(self: Pin<&mut MyObjectQt>);
        #[rust_name = "emit_ready"]
        fn emitReady(self: Pin<&mut MyObjectQt>);

        #[rust_name = "data_changed"]
        fn dataChanged(self: Pin<&mut MyObjectQt>, first: i32, second: &QVariant, third: &QPoint);
        #[rust_name = "emit_data_changed"]
        fn emitDataChanged(
            self: Pin<&mut MyObjectQt>,
            first: i32,
            second: UniquePtr<QVariant>,
            third: QPoint,
        );
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;

        #[cxx_name = "invokableWrapper"]
        fn invokable_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>);
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QPoint = cxx_qt_lib::QPoint;
        type QVariant = cxx_qt_lib::QVariant;
    }

    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/include/convert.h");
        include!("cxx-qt-lib/include/cxxqt_thread.h");

        type MyObjectCxxQtThread;

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObjectQt) -> UniquePtr<MyObjectCxxQtThread>;
        fn queue(self: &MyObjectCxxQtThread, func: fn(ctx: Pin<&mut MyObjectQt>)) -> Result<()>;

        #[rust_name = "new_cpp_object"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }

    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn create_rs() -> Box<MyObject>;

        #[cxx_name = "initialiseCpp"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
    }
}

pub use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;

    pub type FFICppObj = super::ffi::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    unsafe impl Send for MyObjectCxxQtThread {}

    use std::pin::Pin;

    enum MySignals {
        Ready,
        DataChanged {
            first: i32,
            second: UniquePtr<QVariant>,
            third: QPoint,
        },
    }

    #[derive(Default)]
    pub struct MyObject;

    impl MyObject {
        pub fn invokable_wrapper(&mut self, cpp: std::pin::Pin<&mut FFICppObj>) {
            cpp.invokable();
        }
    }

    impl MyObjectQt {
        pub fn invokable(self: Pin<&mut Self>) {
            unsafe {
                self.as_mut().emit_immediate(MySignals::Ready);
            }

            self.as_mut().emit_queued(MySignals::DataChanged {
                first: 1,
                second: QVariant::from_bool(true),
                third: QPoint::new(1, 2),
            });
        }

        pub fn emit_queued(self: Pin<&mut Self>, signal: MySignals) {
            match signal {
                MySignals::Ready {} => self.emit_ready(),
                MySignals::DataChanged {
                    first,
                    second,
                    third,
                } => self.emit_data_changed(first, second, third),
            }
        }

        pub unsafe fn emit_immediate(self: Pin<&mut Self>, signal: MySignals) {
            match signal {
                MySignals::Ready {} => self.ready(),
                MySignals::DataChanged {
                    first,
                    second,
                    third,
                } => self.data_changed(first, &second, &third),
            }
        }

        pub fn grab_values_from_data(mut self: Pin<&mut Self>, mut data: Data) {}
    }

    #[derive(Default)]
    pub struct Data;

    impl From<&MyObjectQt> for Data {
        fn from(_value: &MyObjectQt) -> Self {
            Self {}
        }
    }

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut MyObjectQt>) {
        cpp.grab_values_from_data(Data::default());
    }
}
