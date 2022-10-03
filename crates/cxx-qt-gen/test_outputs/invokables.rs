#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/convert.h");
        include!("cxx-qt-lib/cxxqt_thread.h");
        include!("cxx-qt-lib/std_types.h");
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
        ) -> UniquePtr<Opaque>;
    }

    extern "Rust" {
        #[cxx_name = "invokableReturnTrivialWrapper"]
        fn invokable_return_trivial_wrapper(
            self: &mut MyObject,
            cpp: Pin<&mut MyObjectQt>,
        ) -> QPoint;
    }

    extern "Rust" {
        #[cxx_name = "invokableFinalWrapper"]
        fn invokable_final_wrapper(self: &MyObject, cpp: &MyObjectQt);
    }

    extern "Rust" {
        #[cxx_name = "invokableOverrideWrapper"]
        fn invokable_override_wrapper(self: &MyObject, cpp: &MyObjectQt);
    }

    extern "Rust" {
        #[cxx_name = "invokableVirtualWrapper"]
        fn invokable_virtual_wrapper(self: &MyObject, cpp: &MyObjectQt);
    }

    unsafe extern "C++" {
        type MyObjectCxxQtThread;

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObjectQt) -> UniquePtr<MyObjectCxxQtThread>;

        #[cxx_name = "queue"]
        fn queue_boxed_fn(
            self: &MyObjectCxxQtThread,
            func: fn(Pin<&mut MyObjectQt>, Box<MyObjectCxxQtThreadQueuedFn>),
            arg: Box<MyObjectCxxQtThreadQueuedFn>,
        ) -> Result<()>;

        #[rust_name = "new_cpp_object_my_object_qt"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }

    extern "Rust" {
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        type MyObjectCxxQtThreadQueuedFn;

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
        ) -> UniquePtr<Opaque> {
            return cpp.invokable_return_opaque();
        }
    }

    impl MyObjectQt {
        pub fn invokable_return_opaque(self: Pin<&mut Self>) -> UniquePtr<Opaque> {
            Opaque::new()
        }
    }

    impl MyObject {
        pub fn invokable_return_trivial_wrapper(
            self: &mut MyObject,
            cpp: Pin<&mut MyObjectQt>,
        ) -> QPoint {
            return cpp.invokable_return_trivial();
        }
    }

    impl MyObjectQt {
        pub fn invokable_return_trivial(self: Pin<&mut Self>) -> QPoint {
            QPoint::new(1, 2)
        }
    }

    impl MyObject {
        pub fn invokable_final_wrapper(self: &MyObject, cpp: &MyObjectQt) {
            cpp.invokable_final();
        }
    }

    impl MyObjectQt {
        pub fn invokable_final(&self) {
            println!("Final");
        }
    }

    impl MyObject {
        pub fn invokable_override_wrapper(self: &MyObject, cpp: &MyObjectQt) {
            cpp.invokable_override();
        }
    }

    impl MyObjectQt {
        pub fn invokable_override(&self) {
            println!("Override");
        }
    }

    impl MyObject {
        pub fn invokable_virtual_wrapper(self: &MyObject, cpp: &MyObjectQt) {
            cpp.invokable_virtual();
        }
    }

    impl MyObjectQt {
        pub fn invokable_virtual(&self) {
            println!("Virtual");
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
        pub fn cpp_context_method_return_opaque(&self) -> UniquePtr<Opaque> {
            Opaque::new()
        }
    }

    unsafe impl Send for MyObjectCxxQtThread {}

    impl MyObjectCxxQtThread {
        pub fn queue<F>(&self, f: F) -> std::result::Result<(), cxx::Exception>
        where
            F: FnOnce(std::pin::Pin<&mut MyObjectQt>),
            F: Send + 'static,
        {
            #[allow(clippy::boxed_local)]
            fn func(
                obj: std::pin::Pin<&mut MyObjectQt>,
                arg: std::boxed::Box<MyObjectCxxQtThreadQueuedFn>,
            ) {
                (arg.inner)(obj)
            }
            let arg = MyObjectCxxQtThreadQueuedFn {
                inner: std::boxed::Box::new(f),
            };
            self.queue_boxed_fn(func, std::boxed::Box::new(arg))
        }
    }

    pub struct MyObjectCxxQtThreadQueuedFn {
        inner: std::boxed::Box<dyn FnOnce(std::pin::Pin<&mut MyObjectQt>) + Send>,
    }

    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }

    pub mod qobject {
        pub type MyObject = super::MyObjectQt;
    }
}
