#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
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
    }

    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");
    }

    unsafe extern "C++" {
        #[cxx_name = "MyObject"]
        type MyObject;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObjectRust;
    }

    extern "Rust" {
        #[cxx_name = "getPrimitive"]
        unsafe fn get_primitive<'a>(self: &'a MyObjectRust, cpp: &'a MyObject) -> &'a i32;
    }

    extern "Rust" {
        #[cxx_name = "setPrimitive"]
        fn set_primitive(self: &mut MyObjectRust, cpp: Pin<&mut MyObject>, value: i32);
    }

    unsafe extern "C++" {
        #[rust_name = "emit_primitive_changed"]
        fn emitPrimitiveChanged(self: Pin<&mut MyObject>);
    }

    extern "Rust" {
        #[cxx_name = "getTrivial"]
        unsafe fn get_trivial<'a>(self: &'a MyObjectRust, cpp: &'a MyObject) -> &'a QPoint;
    }

    extern "Rust" {
        #[cxx_name = "setTrivial"]
        fn set_trivial(self: &mut MyObjectRust, cpp: Pin<&mut MyObject>, value: QPoint);
    }

    unsafe extern "C++" {
        #[rust_name = "emit_trivial_changed"]
        fn emitTrivialChanged(self: Pin<&mut MyObject>);
    }

    extern "Rust" {
        #[cxx_name = "getOpaque"]
        unsafe fn get_opaque<'a>(
            self: &'a MyObjectRust,
            cpp: &'a MyObject,
        ) -> &'a UniquePtr<QColor>;
    }

    extern "Rust" {
        #[cxx_name = "setOpaque"]
        fn set_opaque(self: &mut MyObjectRust, cpp: Pin<&mut MyObject>, value: UniquePtr<QColor>);
    }

    unsafe extern "C++" {
        #[rust_name = "emit_opaque_changed"]
        fn emitOpaqueChanged(self: Pin<&mut MyObject>);
    }

    unsafe extern "C++" {
        type MyObjectCxxQtThread;

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObject) -> &MyObjectRust;

        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObject) -> UniquePtr<MyObjectCxxQtThread>;
        fn queue(self: &MyObjectCxxQtThread, func: fn(ctx: Pin<&mut MyObject>)) -> Result<()>;

        #[rust_name = "new_cpp_object_my_object"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn newCppObject() -> UniquePtr<MyObject>;
    }

    extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        unsafe fn rust_mut(self: Pin<&mut MyObject>) -> Pin<&mut MyObjectRust>;
    }

    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn create_rs_my_object_rust() -> Box<MyObjectRust>;
    }
}

pub use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;
    use std::pin::Pin;

    type UniquePtr<T> = cxx::UniquePtr<T>;

    #[derive(Default)]
    pub struct MyObjectRust {
        primitive: i32,
        trivial: QPoint,
        opaque: UniquePtr<QColor>,
    }

    impl MyObjectRust {
        pub fn get_primitive<'a>(&'a self, cpp: &'a MyObject) -> &'a i32 {
            cpp.get_primitive()
        }
    }

    impl MyObject {
        pub fn get_primitive(&self) -> &i32 {
            &self.rust().primitive
        }
    }

    impl MyObjectRust {
        pub fn set_primitive(&mut self, cpp: Pin<&mut MyObject>, value: i32) {
            cpp.set_primitive(value);
        }
    }

    impl MyObject {
        pub fn set_primitive(mut self: Pin<&mut Self>, value: i32) {
            unsafe {
                self.as_mut().rust_mut().primitive = value;
            }
            self.as_mut().emit_primitive_changed();
        }
    }

    impl MyObjectRust {
        pub fn get_trivial<'a>(&'a self, cpp: &'a MyObject) -> &'a QPoint {
            cpp.get_trivial()
        }
    }

    impl MyObject {
        pub fn get_trivial(&self) -> &QPoint {
            &self.rust().trivial
        }
    }

    impl MyObjectRust {
        pub fn set_trivial(&mut self, cpp: Pin<&mut MyObject>, value: QPoint) {
            cpp.set_trivial(value);
        }
    }

    impl MyObject {
        pub fn set_trivial(mut self: Pin<&mut Self>, value: QPoint) {
            unsafe {
                self.as_mut().rust_mut().trivial = value;
            }
            self.as_mut().emit_trivial_changed();
        }
    }

    impl MyObjectRust {
        pub fn get_opaque<'a>(&'a self, cpp: &'a MyObject) -> &'a UniquePtr<QColor> {
            cpp.get_opaque()
        }
    }

    impl MyObject {
        pub fn get_opaque(&self) -> &UniquePtr<QColor> {
            &self.rust().opaque
        }
    }

    impl MyObjectRust {
        pub fn set_opaque(&mut self, cpp: Pin<&mut MyObject>, value: UniquePtr<QColor>) {
            cpp.set_opaque(value);
        }
    }

    impl MyObject {
        pub fn set_opaque(mut self: Pin<&mut Self>, value: UniquePtr<QColor>) {
            unsafe {
                self.as_mut().rust_mut().opaque = value;
            }
            self.as_mut().emit_opaque_changed();
        }
    }

    unsafe impl Send for MyObjectCxxQtThread {}

    pub fn create_rs_my_object_rust() -> std::boxed::Box<MyObjectRust> {
        std::default::Default::default()
    }
}
