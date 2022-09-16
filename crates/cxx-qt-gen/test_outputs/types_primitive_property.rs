#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/include/convert.h");
        include!("cxx-qt-lib/include/cxxqt_thread.h");
    }

    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");
    }

    unsafe extern "C++" {
        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "emit_boolean_changed"]
        fn emitBooleanChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_float_32_changed"]
        fn emitFloat32Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_float_64_changed"]
        fn emitFloat64Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_int_8_changed"]
        fn emitInt8Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_int_16_changed"]
        fn emitInt16Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_int_32_changed"]
        fn emitInt32Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_uint_8_changed"]
        fn emitUint8Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_uint_16_changed"]
        fn emitUint16Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_uint_32_changed"]
        fn emitUint32Changed(self: Pin<&mut MyObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;

        #[cxx_name = "getBoolean"]
        unsafe fn get_boolean<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a bool;
        #[cxx_name = "setBoolean"]
        fn set_boolean(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: bool);

        #[cxx_name = "getFloat32"]
        unsafe fn get_float_32<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a f32;
        #[cxx_name = "setFloat32"]
        fn set_float_32(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: f32);

        #[cxx_name = "getFloat64"]
        unsafe fn get_float_64<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a f64;
        #[cxx_name = "setFloat64"]
        fn set_float_64(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: f64);

        #[cxx_name = "getInt8"]
        unsafe fn get_int_8<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i8;
        #[cxx_name = "setInt8"]
        fn set_int_8(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i8);

        #[cxx_name = "getInt16"]
        unsafe fn get_int_16<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i16;
        #[cxx_name = "setInt16"]
        fn set_int_16(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i16);

        #[cxx_name = "getInt32"]
        unsafe fn get_int_32<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i32;
        #[cxx_name = "setInt32"]
        fn set_int_32(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i32);

        #[cxx_name = "getUint8"]
        unsafe fn get_uint_8<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a u8;
        #[cxx_name = "setUint8"]
        fn set_uint_8(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: u8);

        #[cxx_name = "getUint16"]
        unsafe fn get_uint_16<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a u16;
        #[cxx_name = "setUint16"]
        fn set_uint_16(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: u16);

        #[cxx_name = "getUint32"]
        unsafe fn get_uint_32<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a u32;
        #[cxx_name = "setUint32"]
        fn set_uint_32(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: u32);
    }

    unsafe extern "C++" {
        type MyObjectCxxQtThread;

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObjectQt) -> UniquePtr<MyObjectCxxQtThread>;
        fn queue(self: &MyObjectCxxQtThread, func: fn(ctx: Pin<&mut MyObjectQt>)) -> Result<()>;

        #[rust_name = "new_cpp_object_my_object_qt"]
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
        fn create_rs_my_object() -> Box<MyObject>;
    }
}

pub use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;
    use std::pin::Pin;

    type UniquePtr<T> = cxx::UniquePtr<T>;

    #[derive(Default)]
    pub struct MyObject {
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

    impl MyObject {
        pub fn get_boolean<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a bool {
            cpp.get_boolean()
        }

        pub fn set_boolean(&mut self, cpp: Pin<&mut MyObjectQt>, value: bool) {
            cpp.set_boolean(value);
        }

        pub fn get_float_32<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a f32 {
            cpp.get_float_32()
        }

        pub fn set_float_32(&mut self, cpp: Pin<&mut MyObjectQt>, value: f32) {
            cpp.set_float_32(value);
        }

        pub fn get_float_64<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a f64 {
            cpp.get_float_64()
        }

        pub fn set_float_64(&mut self, cpp: Pin<&mut MyObjectQt>, value: f64) {
            cpp.set_float_64(value);
        }

        pub fn get_int_8<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i8 {
            cpp.get_int_8()
        }

        pub fn set_int_8(&mut self, cpp: Pin<&mut MyObjectQt>, value: i8) {
            cpp.set_int_8(value);
        }

        pub fn get_int_16<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i16 {
            cpp.get_int_16()
        }

        pub fn set_int_16(&mut self, cpp: Pin<&mut MyObjectQt>, value: i16) {
            cpp.set_int_16(value);
        }

        pub fn get_int_32<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
            cpp.get_int_32()
        }

        pub fn set_int_32(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
            cpp.set_int_32(value);
        }

        pub fn get_uint_8<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a u8 {
            cpp.get_uint_8()
        }

        pub fn set_uint_8(&mut self, cpp: Pin<&mut MyObjectQt>, value: u8) {
            cpp.set_uint_8(value);
        }

        pub fn get_uint_16<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a u16 {
            cpp.get_uint_16()
        }

        pub fn set_uint_16(&mut self, cpp: Pin<&mut MyObjectQt>, value: u16) {
            cpp.set_uint_16(value);
        }

        pub fn get_uint_32<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a u32 {
            cpp.get_uint_32()
        }

        pub fn set_uint_32(&mut self, cpp: Pin<&mut MyObjectQt>, value: u32) {
            cpp.set_uint_32(value);
        }
    }

    impl MyObjectQt {
        pub fn get_boolean(&self) -> &bool {
            &self.rust().boolean
        }

        pub fn set_boolean(mut self: Pin<&mut Self>, value: bool) {
            unsafe {
                self.as_mut().rust_mut().boolean = value;
            }
            self.as_mut().emit_boolean_changed();
        }

        pub fn get_float_32(&self) -> &f32 {
            &self.rust().float_32
        }

        pub fn set_float_32(mut self: Pin<&mut Self>, value: f32) {
            unsafe {
                self.as_mut().rust_mut().float_32 = value;
            }
            self.as_mut().emit_float_32_changed();
        }

        pub fn get_float_64(&self) -> &f64 {
            &self.rust().float_64
        }

        pub fn set_float_64(mut self: Pin<&mut Self>, value: f64) {
            unsafe {
                self.as_mut().rust_mut().float_64 = value;
            }
            self.as_mut().emit_float_64_changed();
        }

        pub fn get_int_8(&self) -> &i8 {
            &self.rust().int_8
        }

        pub fn set_int_8(mut self: Pin<&mut Self>, value: i8) {
            unsafe {
                self.as_mut().rust_mut().int_8 = value;
            }
            self.as_mut().emit_int_8_changed();
        }

        pub fn get_int_16(&self) -> &i16 {
            &self.rust().int_16
        }

        pub fn set_int_16(mut self: Pin<&mut Self>, value: i16) {
            unsafe {
                self.as_mut().rust_mut().int_16 = value;
            }
            self.as_mut().emit_int_16_changed();
        }

        pub fn get_int_32(&self) -> &i32 {
            &self.rust().int_32
        }

        pub fn set_int_32(mut self: Pin<&mut Self>, value: i32) {
            unsafe {
                self.as_mut().rust_mut().int_32 = value;
            }
            self.as_mut().emit_int_32_changed();
        }

        pub fn get_uint_8(&self) -> &u8 {
            &self.rust().uint_8
        }

        pub fn set_uint_8(mut self: Pin<&mut Self>, value: u8) {
            unsafe {
                self.as_mut().rust_mut().uint_8 = value;
            }
            self.as_mut().emit_uint_8_changed();
        }

        pub fn get_uint_16(&self) -> &u16 {
            &self.rust().uint_16
        }

        pub fn set_uint_16(mut self: Pin<&mut Self>, value: u16) {
            unsafe {
                self.as_mut().rust_mut().uint_16 = value;
            }
            self.as_mut().emit_uint_16_changed();
        }

        pub fn get_uint_32(&self) -> &u32 {
            &self.rust().uint_32
        }

        pub fn set_uint_32(mut self: Pin<&mut Self>, value: u32) {
            unsafe {
                self.as_mut().rust_mut().uint_32 = value;
            }
            self.as_mut().emit_uint_32_changed();
        }
    }

    unsafe impl Send for MyObjectCxxQtThread {}

    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
}
