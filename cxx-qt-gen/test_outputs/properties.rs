#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "primitive"]
        fn getPrimitive(self: &MyObjectQt) -> i32;
        #[rust_name = "set_primitive"]
        fn setPrimitive(self: Pin<&mut MyObjectQt>, value: i32);

        #[rust_name = "opaque"]
        fn getOpaque(self: &MyObjectQt) -> &QColor;
        #[rust_name = "set_opaque"]
        fn setOpaque(self: Pin<&mut MyObjectQt>, value: &QColor);
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QColor = cxx_qt_lib::QColor;
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

    #[derive(Default)]
    pub struct MyObject;

    impl MyObject {}

    impl MyObjectQt {
        pub fn grab_values_from_data(mut self: Pin<&mut Self>, mut data: Data) {
            self.as_mut().set_primitive(data.primitive);
            self.as_mut().set_opaque(data.opaque.as_ref().unwrap());
        }
    }

    #[derive(Default)]
    pub struct Data {
        primitive: i32,
        opaque: UniquePtr<QColor>,
    }

    impl From<&MyObjectQt> for Data {
        fn from(value: &MyObjectQt) -> Self {
            Self {
                primitive: value.primitive().into(),
                opaque: value.opaque().into(),
            }
        }
    }

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut MyObjectQt>) {
        cpp.grab_values_from_data(Data::default());
    }
}
