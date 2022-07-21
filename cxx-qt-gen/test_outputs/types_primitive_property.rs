#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
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

        #[rust_name = "new_cpp_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type RustObj;

        #[cxx_name = "createRs"]
        fn create_rs() -> Box<RustObj>;

        #[cxx_name = "initialiseCpp"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
    }
}

pub use self::cxx_qt_my_object::*;
mod cxx_qt_my_object {
    use super::my_object::*;

    pub type FFICppObj = super::my_object::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {}

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn boolean(&self) -> bool {
            self.cpp.boolean()
        }

        pub fn set_boolean(&mut self, value: bool) {
            self.cpp.as_mut().set_boolean(value);
        }

        pub fn float_32(&self) -> f32 {
            self.cpp.float_32()
        }

        pub fn set_float_32(&mut self, value: f32) {
            self.cpp.as_mut().set_float_32(value);
        }

        pub fn float_64(&self) -> f64 {
            self.cpp.float_64()
        }

        pub fn set_float_64(&mut self, value: f64) {
            self.cpp.as_mut().set_float_64(value);
        }

        pub fn int_8(&self) -> i8 {
            self.cpp.int_8()
        }

        pub fn set_int_8(&mut self, value: i8) {
            self.cpp.as_mut().set_int_8(value);
        }

        pub fn int_16(&self) -> i16 {
            self.cpp.int_16()
        }

        pub fn set_int_16(&mut self, value: i16) {
            self.cpp.as_mut().set_int_16(value);
        }

        pub fn int_32(&self) -> i32 {
            self.cpp.int_32()
        }

        pub fn set_int_32(&mut self, value: i32) {
            self.cpp.as_mut().set_int_32(value);
        }

        pub fn uint_8(&self) -> u8 {
            self.cpp.uint_8()
        }

        pub fn set_uint_8(&mut self, value: u8) {
            self.cpp.as_mut().set_uint_8(value);
        }

        pub fn uint_16(&self) -> u16 {
            self.cpp.uint_16()
        }

        pub fn set_uint_16(&mut self, value: u16) {
            self.cpp.as_mut().set_uint_16(value);
        }

        pub fn uint_32(&self) -> u32 {
            self.cpp.uint_32()
        }

        pub fn set_uint_32(&mut self, value: u32) {
            self.cpp.as_mut().set_uint_32(value);
        }

        pub fn grab_values_from_data(&mut self, mut data: Data) {
            self.set_boolean(data.boolean);
            self.set_float_32(data.float_32);
            self.set_float_64(data.float_64);
            self.set_int_8(data.int_8);
            self.set_int_16(data.int_16);
            self.set_int_32(data.int_32);
            self.set_uint_8(data.uint_8);
            self.set_uint_16(data.uint_16);
            self.set_uint_32(data.uint_32);
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

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(value: &CppObj<'a>) -> Self {
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

    impl<'a> From<&mut CppObj<'a>> for Data {
        fn from(value: &mut CppObj<'a>) -> Self {
            Self::from(&*value)
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
