#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        type MyObject;

        #[rust_name = "ready"]
        fn ready(self: Pin<&mut MyObject>);
        #[rust_name = "emit_ready"]
        fn emitReady(self: Pin<&mut MyObject>);

        #[rust_name = "data_changed"]
        fn dataChanged(self: Pin<&mut MyObject>, first: i32, second: &QVariant, third: &QPoint);
        #[rust_name = "emit_data_changed"]
        fn emitDataChanged(
            self: Pin<&mut MyObject>,
            first: i32,
            second: UniquePtr<QVariant>,
            third: QPoint,
        );

        #[rust_name = "new_cpp_object"]
        fn newCppObject() -> UniquePtr<MyObject>;
    }

    extern "Rust" {
        type RustObj;

        #[cxx_name = "invokableWrapper"]
        fn invokable_wrapper(self: &RustObj, cpp: Pin<&mut MyObject>);

        #[cxx_name = "createRs"]
        fn create_rs() -> Box<RustObj>;

        #[cxx_name = "initialiseCpp"]
        fn initialise_cpp(cpp: Pin<&mut MyObject>);
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QPoint = cxx_qt_lib::QPoint;
        type QVariant = cxx_qt_lib::QVariant;
    }
}

pub use self::cxx_qt_my_object::*;
mod cxx_qt_my_object {
    use super::my_object::*;

    pub type FFICppObj = super::my_object::MyObject;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    enum Signal {
        Ready,
        DataChanged {
            first: i32,
            second: UniquePtr<QVariant>,
            third: QPoint,
        },
    }

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        pub fn invokable_wrapper(&self, cpp: std::pin::Pin<&mut FFICppObj>) {
            let mut cpp = CppObj::new(cpp);
            self.invokable(&mut cpp);
        }

        pub fn invokable(&self, cpp: &mut CppObj) {
            unsafe {
                cpp.emit_immediate(Signal::Ready);
            }

            cpp.emit_queued(Signal::DataChanged {
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

        pub fn emit_queued(&mut self, signal: Signal) {
            match signal {
                Signal::Ready {} => self.cpp.as_mut().emit_ready(),
                Signal::DataChanged {
                    first,
                    second,
                    third,
                } => self.cpp.as_mut().emit_data_changed(first, second, third),
            }
        }

        pub unsafe fn emit_immediate(&mut self, signal: Signal) {
            match signal {
                Signal::Ready {} => self.cpp.as_mut().ready(),
                Signal::DataChanged {
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

    pub fn create_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(Data::default());
    }
}
