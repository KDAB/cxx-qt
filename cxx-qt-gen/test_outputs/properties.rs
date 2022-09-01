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
        fn get_primitive(self: &MyObject, cpp: &MyObjectQt) -> i32;
        #[cxx_name = "setPrimitive"]
        fn set_primitive(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i32);

        #[cxx_name = "getTrivial"]
        fn get_trivial(self: &MyObject, cpp: &MyObjectQt) -> QPoint;
        #[cxx_name = "setTrivial"]
        fn set_trivial(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QPoint);

        #[cxx_name = "getOpaque"]
        fn get_opaque(self: &MyObject, cpp: &MyObjectQt) -> UniquePtr<QColor>;
        #[cxx_name = "setOpaque"]
        fn set_opaque(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QColor);
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

    pub type FFICppObj = super::ffi::MyObjectQt;
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
        pub fn get_primitive(&self, cpp: &MyObjectQt) -> i32 {
            cpp.get_primitive()
        }

        pub fn set_primitive(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
            cpp.set_primitive(value);
        }

        pub fn get_trivial(&self, cpp: &MyObjectQt) -> QPoint {
            cpp.get_trivial()
        }

        pub fn set_trivial(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QPoint) {
            cpp.set_trivial(value);
        }

        pub fn get_opaque(&self, cpp: &MyObjectQt) -> UniquePtr<QColor> {
            cpp.get_opaque()
        }

        pub fn set_opaque(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QColor) {
            cpp.set_opaque(value);
        }
    }

    impl MyObjectQt {
        pub fn get_primitive(&self) -> i32 {
            self.rust().primitive
        }

        pub fn set_primitive(mut self: Pin<&mut Self>, value: i32) {
            unsafe {
                self.as_mut().rust_mut().primitive = value;
            }
            self.as_mut().emit_primitive_changed();
        }

        pub fn get_trivial(&self) -> QPoint {
            self.rust().trivial.clone()
        }

        pub fn set_trivial(mut self: Pin<&mut Self>, value: &QPoint) {
            unsafe {
                self.as_mut().rust_mut().trivial = value.clone();
            }
            self.as_mut().emit_trivial_changed();
        }

        pub fn get_opaque(&self) -> UniquePtr<QColor> {
            QColor::from_ref(&self.rust().opaque)
        }

        pub fn set_opaque(mut self: Pin<&mut Self>, value: &QColor) {
            unsafe {
                self.as_mut().rust_mut().opaque = QColor::from_ref(value);
            }
            self.as_mut().emit_opaque_changed();
        }
    }

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
}
