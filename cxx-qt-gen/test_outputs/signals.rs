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
        fn invokable_wrapper(self: &MyObject, cpp: Pin<&mut MyObjectQt>);
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

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

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
        pub fn invokable_wrapper(&self, cpp: std::pin::Pin<&mut FFICppObj>) {
            let mut cpp = CppObj::new(cpp);
            self.invokable(&mut cpp);
        }

        pub fn invokable(&self, cpp: &mut CppObj) {
            unsafe {
                cpp.emit_immediate(MySignals::Ready);
            }

            cpp.emit_queued(MySignals::DataChanged {
                first: 1,
                second: QVariant::from_bool(true),
                third: QPoint::new(1, 2),
            });
        }
    }

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn emit_queued(&mut self, signal: MySignals) {
            match signal {
                MySignals::Ready {} => self.cpp.as_mut().emit_ready(),
                MySignals::DataChanged {
                    first,
                    second,
                    third,
                } => self.cpp.as_mut().emit_data_changed(first, second, third),
            }
        }

        pub unsafe fn emit_immediate(&mut self, signal: MySignals) {
            match signal {
                MySignals::Ready {} => self.cpp.as_mut().ready(),
                MySignals::DataChanged {
                    first,
                    second,
                    third,
                } => self.cpp.as_mut().data_changed(first, &second, &third),
            }
        }

        pub fn grab_values_from_data(&mut self, mut data: Data) {}
    }

    #[derive(Default)]
    pub struct Data;

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(_value: &CppObj<'a>) -> Self {
            Self {}
        }
    }

    impl<'a> From<&mut CppObj<'a>> for Data {
        fn from(_value: &mut CppObj<'a>) -> Self {
            Self::from(&*_value)
        }
    }

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(Data::default());
    }
}
