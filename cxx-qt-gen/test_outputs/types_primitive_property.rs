#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "boolean"]
        fn getBoolean(self: &MyObjectQt) -> bool;
        #[rust_name = "set_boolean"]
        fn setBoolean(self: Pin<&mut MyObjectQt>, value: bool);

        #[rust_name = "float_32"]
        fn getFloat32(self: &MyObjectQt) -> f32;
        #[rust_name = "set_float_32"]
        fn setFloat32(self: Pin<&mut MyObjectQt>, value: f32);

        #[rust_name = "float_64"]
        fn getFloat64(self: &MyObjectQt) -> f64;
        #[rust_name = "set_float_64"]
        fn setFloat64(self: Pin<&mut MyObjectQt>, value: f64);

        #[rust_name = "int_8"]
        fn getInt8(self: &MyObjectQt) -> i8;
        #[rust_name = "set_int_8"]
        fn setInt8(self: Pin<&mut MyObjectQt>, value: i8);

        #[rust_name = "int_16"]
        fn getInt16(self: &MyObjectQt) -> i16;
        #[rust_name = "set_int_16"]
        fn setInt16(self: Pin<&mut MyObjectQt>, value: i16);

        #[rust_name = "int_32"]
        fn getInt32(self: &MyObjectQt) -> i32;
        #[rust_name = "set_int_32"]
        fn setInt32(self: Pin<&mut MyObjectQt>, value: i32);

        #[rust_name = "uint_8"]
        fn getUint8(self: &MyObjectQt) -> u8;
        #[rust_name = "set_uint_8"]
        fn setUint8(self: Pin<&mut MyObjectQt>, value: u8);

        #[rust_name = "uint_16"]
        fn getUint16(self: &MyObjectQt) -> u16;
        #[rust_name = "set_uint_16"]
        fn setUint16(self: Pin<&mut MyObjectQt>, value: u16);

        #[rust_name = "uint_32"]
        fn getUint32(self: &MyObjectQt) -> u32;
        #[rust_name = "set_uint_32"]
        fn setUint32(self: Pin<&mut MyObjectQt>, value: u32);
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;
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
            self.as_mut().set_boolean(data.boolean);
            self.as_mut().set_float_32(data.float_32);
            self.as_mut().set_float_64(data.float_64);
            self.as_mut().set_int_8(data.int_8);
            self.as_mut().set_int_16(data.int_16);
            self.as_mut().set_int_32(data.int_32);
            self.as_mut().set_uint_8(data.uint_8);
            self.as_mut().set_uint_16(data.uint_16);
            self.as_mut().set_uint_32(data.uint_32);
        }
    }

    #[derive(Default)]
    pub struct Data {
        boolean: bool,
        float_32: f32,
        float_64: f64,
        int_8: i8,
        int_16: i16,
        int_32: i32,
        uint_8: u8,
        uint_16: u16,
        uint_32: u32,
    }

    impl From<&MyObjectQt> for Data {
        fn from(value: &MyObjectQt) -> Self {
            Self {
                boolean: value.boolean().into(),
                float_32: value.float_32().into(),
                float_64: value.float_64().into(),
                int_8: value.int_8().into(),
                int_16: value.int_16().into(),
                int_32: value.int_32().into(),
                uint_8: value.uint_8().into(),
                uint_16: value.uint_16().into(),
                uint_32: value.uint_32().into(),
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
