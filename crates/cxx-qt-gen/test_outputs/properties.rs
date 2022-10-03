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
    }

    unsafe extern "C++" {
        include!("cxx-qt-gen/ffi.cxxqt.h");
    }

    unsafe extern "C++" {
        #[cxx_name = "MyObject"]
        type MyObjectQt;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;
    }

    extern "Rust" {
        #[cxx_name = "getPrimitive"]
        unsafe fn primitive<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i32;
    }

    extern "Rust" {
        #[cxx_name = "setPrimitive"]
        fn set_primitive(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i32);
    }

    unsafe extern "C++" {
        #[rust_name = "primitive_changed"]
        fn primitiveChanged(self: Pin<&mut MyObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "getTrivial"]
        unsafe fn trivial<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a QPoint;
    }

    extern "Rust" {
        #[cxx_name = "setTrivial"]
        fn set_trivial(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: QPoint);
    }

    unsafe extern "C++" {
        #[rust_name = "trivial_changed"]
        fn trivialChanged(self: Pin<&mut MyObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "getOpaque"]
        unsafe fn opaque<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a UniquePtr<Opaque>;
    }

    extern "Rust" {
        #[cxx_name = "setOpaque"]
        fn set_opaque(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<Opaque>);
    }

    unsafe extern "C++" {
        #[rust_name = "opaque_changed"]
        fn opaqueChanged(self: Pin<&mut MyObjectQt>);
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
        primitive: i32,
        trivial: QPoint,
        opaque: UniquePtr<Opaque>,
        private_rust_field: i32,
        pub public_rust_field: f64,
    }

    impl MyObject {
        pub fn primitive<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
            cpp.primitive()
        }
    }

    impl MyObjectQt {
        pub fn primitive(&self) -> &i32 {
            &self.rust().primitive
        }
    }

    impl MyObjectQt {
        pub unsafe fn primitive_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut i32 {
            &mut self.rust_mut().get_unchecked_mut().primitive
        }
    }

    impl MyObject {
        pub fn set_primitive(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
            cpp.set_primitive(value);
        }
    }

    impl MyObjectQt {
        pub fn set_primitive(mut self: Pin<&mut Self>, value: i32) {
            unsafe {
                self.as_mut().rust_mut().primitive = value;
            }
            self.as_mut().primitive_changed();
        }
    }

    impl MyObject {
        pub fn trivial<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a QPoint {
            cpp.trivial()
        }
    }

    impl MyObjectQt {
        pub fn trivial(&self) -> &QPoint {
            &self.rust().trivial
        }
    }

    impl MyObjectQt {
        pub unsafe fn trivial_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut QPoint {
            &mut self.rust_mut().get_unchecked_mut().trivial
        }
    }

    impl MyObject {
        pub fn set_trivial(&mut self, cpp: Pin<&mut MyObjectQt>, value: QPoint) {
            cpp.set_trivial(value);
        }
    }

    impl MyObjectQt {
        pub fn set_trivial(mut self: Pin<&mut Self>, value: QPoint) {
            unsafe {
                self.as_mut().rust_mut().trivial = value;
            }
            self.as_mut().trivial_changed();
        }
    }

    impl MyObject {
        pub fn opaque<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a UniquePtr<Opaque> {
            cpp.opaque()
        }
    }

    impl MyObjectQt {
        pub fn opaque(&self) -> &UniquePtr<Opaque> {
            &self.rust().opaque
        }
    }

    impl MyObjectQt {
        pub unsafe fn opaque_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut UniquePtr<Opaque> {
            &mut self.rust_mut().get_unchecked_mut().opaque
        }
    }

    impl MyObject {
        pub fn set_opaque(&mut self, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<Opaque>) {
            cpp.set_opaque(value);
        }
    }

    impl MyObjectQt {
        pub fn set_opaque(mut self: Pin<&mut Self>, value: UniquePtr<Opaque>) {
            unsafe {
                self.as_mut().rust_mut().opaque = value;
            }
            self.as_mut().opaque_changed();
        }
    }

    impl MyObjectQt {
        fn private_rust_field(&self) -> &i32 {
            &self.rust().private_rust_field
        }
    }

    impl MyObjectQt {
        fn private_rust_field_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut i32 {
            unsafe { &mut self.rust_mut().get_unchecked_mut().private_rust_field }
        }
    }

    impl MyObjectQt {
        fn set_private_rust_field(mut self: Pin<&mut Self>, value: i32) {
            unsafe {
                self.rust_mut().private_rust_field = value;
            }
        }
    }

    impl MyObjectQt {
        pub fn public_rust_field(&self) -> &f64 {
            &self.rust().public_rust_field
        }
    }

    impl MyObjectQt {
        pub fn public_rust_field_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut f64 {
            unsafe { &mut self.rust_mut().get_unchecked_mut().public_rust_field }
        }
    }

    impl MyObjectQt {
        pub fn set_public_rust_field(mut self: Pin<&mut Self>, value: f64) {
            unsafe {
                self.rust_mut().public_rust_field = value;
            }
        }
    }

    unsafe impl Send for MyObjectCxxQtThread {}

    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
}
