#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "emit_primitive_changed"]
        fn emitPrimitiveChanged(self: Pin<&mut MyObjectQt>);
        #[rust_name = "emit_trivial_changed"]
        fn emitTrivialChanged(self: Pin<&mut MyObjectQt>);
        #[rust_name = "emit_opaque_changed"]
        fn emitOpaqueChanged(self: Pin<&mut MyObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;

        #[cxx_name = "getPrimitive"]
        unsafe fn get_primitive<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i32;
        #[cxx_name = "setPrimitive"]
        fn set_primitive(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i32);

        #[cxx_name = "getTrivial"]
        unsafe fn get_trivial<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a QPoint;
        #[cxx_name = "setTrivial"]
        fn set_trivial(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: QPoint);

        #[cxx_name = "getOpaque"]
        unsafe fn get_opaque<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a UniquePtr<QColor>;
        #[cxx_name = "setOpaque"]
        fn set_opaque(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QColor>);
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QColor = cxx_qt_lib::QColor;
        type QPoint = cxx_qt_lib::QPoint;
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
    }
}

pub use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;

    type UniquePtr<T> = cxx::UniquePtr<T>;

    unsafe impl Send for MyObjectCxxQtThread {}

    use std::pin::Pin;

    #[derive(Default)]
    pub struct MyObject {
        primitive: i32,
        trivial: QPoint,
        opaque: UniquePtr<QColor>,
    }

    impl MyObject {
        pub fn get_primitive<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
            cpp.get_primitive()
        }

        pub fn set_primitive(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
            cpp.set_primitive(value);
        }

        pub fn get_trivial<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a QPoint {
            cpp.get_trivial()
        }

        pub fn set_trivial(&mut self, cpp: Pin<&mut MyObjectQt>, value: QPoint) {
            cpp.set_trivial(value);
        }

        pub fn get_opaque<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a UniquePtr<QColor> {
            cpp.get_opaque()
        }

        pub fn set_opaque(&mut self, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QColor>) {
            cpp.set_opaque(value);
        }
    }

    impl MyObjectQt {
        pub fn get_primitive(&self) -> &i32 {
            &self.rust().primitive
        }

        pub fn set_primitive(mut self: Pin<&mut Self>, value: i32) {
            unsafe {
                self.as_mut().rust_mut().primitive = value;
            }
            self.as_mut().emit_primitive_changed();
        }

        pub fn get_trivial(&self) -> &QPoint {
            &self.rust().trivial
        }

        pub fn set_trivial(mut self: Pin<&mut Self>, value: QPoint) {
            unsafe {
                self.as_mut().rust_mut().trivial = value;
            }
            self.as_mut().emit_trivial_changed();
        }

        pub fn get_opaque(&self) -> &UniquePtr<QColor> {
            &self.rust().opaque
        }

        pub fn set_opaque(mut self: Pin<&mut Self>, value: UniquePtr<QColor>) {
            unsafe {
                self.as_mut().rust_mut().opaque = value;
            }
            self.as_mut().emit_opaque_changed();
        }
    }

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
}
