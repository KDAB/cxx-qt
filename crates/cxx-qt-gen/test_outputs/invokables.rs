#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QColor = cxx_qt_lib::QColor;
        type QPoint = cxx_qt_lib::QPoint;
        type QString = cxx_qt_lib::QString;
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
        type MyObjectQt;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;
    }

    extern "Rust" {
        #[cxx_name = "invokableWrapper"]
        fn invokable_wrapper(self: &MyObject, cpp: &MyObjectQt);
    }

    extern "Rust" {
        #[cxx_name = "invokableMutableWrapper"]
        fn invokable_mutable_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "invokableParametersWrapper"]
        fn invokable_parameters_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            opaque: &QColor,
            trivial: &QPoint,
            primitive: i32,
        );
    }

    extern "Rust" {
        #[cxx_name = "invokableReturnOpaqueWrapper"]
        fn invokable_return_opaque_wrapper(
            self: &mut MyObject,
            cpp: Pin<&mut MyObjectQt>,
        ) -> UniquePtr<QColor>;
    }

    extern "Rust" {
        #[cxx_name = "invokableReturnPrimitiveWrapper"]
        fn invokable_return_primitive_wrapper(
            self: &mut MyObject,
            cpp: Pin<&mut MyObjectQt>,
        ) -> i32;
    }

    extern "Rust" {
        #[cxx_name = "invokableReturnStaticWrapper"]
        fn invokable_return_static_wrapper(
            self: &mut MyObject,
            cpp: Pin<&mut MyObjectQt>,
        ) -> UniquePtr<QString>;
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

    impl MyObject {
        pub fn rust_only_method(&self) {
            println!("QML or C++ can't call this :)");
        }
    }

    #[derive(Default)]
    pub struct MyObject;

    impl MyObject {
        pub fn invokable_wrapper(self: &MyObject, cpp: &MyObjectQt) {
            cpp.invokable();
        }
    }

    impl MyObjectQt {
        pub fn invokable(&self) {
            println!("invokable");
        }
    }

    impl MyObject {
        pub fn invokable_mutable_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>) {
            cpp.invokable_mutable();
        }
    }

    impl MyObjectQt {
        pub fn invokable_mutable(self: Pin<&mut Self>) {
            println!("This method is mutable!");
        }
    }

    impl MyObject {
        pub fn invokable_parameters_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            opaque: &QColor,
            trivial: &QPoint,
            primitive: i32,
        ) {
            cpp.invokable_parameters(opaque, trivial, primitive);
        }
    }

    impl MyObjectQt {
        pub fn invokable_parameters(&self, opaque: &QColor, trivial: &QPoint, primitive: i32) {
            println!(
                "Red: {}, Point X: {}, Number: {}",
                opaque.red(),
                trivial.x(),
                primitive,
            );
        }
    }

    impl MyObject {
        pub fn invokable_return_opaque_wrapper(
            self: &mut MyObject,
            cpp: Pin<&mut MyObjectQt>,
        ) -> UniquePtr<QColor> {
            return cpp.invokable_return_opaque();
        }
    }

    impl MyObjectQt {
        pub fn invokable_return_opaque(self: Pin<&mut Self>) -> UniquePtr<QColor> {
            cxx_qt_lib::QColor::from_rgba(255, 0, 0, 0)
        }
    }

    impl MyObject {
        pub fn invokable_return_primitive_wrapper(
            self: &mut MyObject,
            cpp: Pin<&mut MyObjectQt>,
        ) -> i32 {
            return cpp.invokable_return_primitive();
        }
    }

    impl MyObjectQt {
        pub fn invokable_return_primitive(self: Pin<&mut Self>) -> i32 {
            2
        }
    }

    impl MyObject {
        pub fn invokable_return_static_wrapper(
            self: &mut MyObject,
            cpp: Pin<&mut MyObjectQt>,
        ) -> UniquePtr<QString> {
            return cpp.invokable_return_static();
        }
    }

    impl MyObjectQt {
        pub fn invokable_return_static(self: Pin<&mut Self>) -> UniquePtr<QString> {
            QString::from_str("static")
        }
    }

    impl MyObjectQt {
        pub fn cpp_context_method(&self) {
            println!("C++ context method");
        }
    }

    impl MyObjectQt {
        pub fn cpp_context_method_mutable(self: Pin<&mut Self>) {
            println!("mutable method");
        }
    }

    impl MyObjectQt {
        pub fn cpp_context_method_return_opaque(&self) -> UniquePtr<QColor> {
            cxx_qt_lib::QColor::from_rgba(255, 0, 0, 0)
        }
    }

    unsafe impl Send for MyObjectCxxQtThread {}

    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
}
