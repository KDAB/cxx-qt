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
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObject"]
        #[doc = "\n"]
        #[doc = "Use type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
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
        #[doc = r" Specialised version of CxxQtThread, which can be moved into other threads."]
        #[doc = r""]
        #[doc = r" CXX doesn't support having generic types in the function yet"]
        #[doc = r" so we cannot have CxxQtThread<T> in cxx-qt-lib and then use that here"]
        #[doc = r" For now we use a type alias in C++ then use it like a normal type here"]
        #[doc = r" https://github.com/dtolnay/cxx/issues/683"]
        type MyObjectCxxQtThread;

        #[doc = r" Retrieve an immutable reference to the Rust struct backing this C++ object"]
        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

        #[doc = r" Create an instance of a CxxQtThread"]
        #[doc = r""]
        #[doc = r" This allows for queueing closures onto the Qt event loop from a background thread."]
        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObjectQt) -> UniquePtr<MyObjectCxxQtThread>;

        #[cxx_name = "queue"]
        fn queue_boxed_fn(
            self: &MyObjectCxxQtThread,
            func: fn(Pin<&mut MyObjectQt>, Box<MyObjectCxxQtThreadQueuedFn>),
            arg: Box<MyObjectCxxQtThreadQueuedFn>,
        ) -> Result<()>;

        #[doc = "Generated CXX-Qt method which creates a new"]
        #[doc = "MyObjectQt"]
        #[doc = "as a UniquePtr with no parent in Qt"]
        #[rust_name = "new_cpp_object_my_object_qt"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[doc = r" Retrieve a mutable reference to the Rust struct backing this C++ object"]
        #[doc = r""]
        #[doc = r" This method is unsafe as if a Q_PROPERTY is modified its changed signal must be triggered manually."]
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
        #[doc = "Generated CXX-Qt wrapper method for the Q_INVOKABLE"]
        #[doc = "invokable"]
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
        #[doc = "Generated CXX-Qt wrapper method for the Q_INVOKABLE"]
        #[doc = "invokable_mutable"]
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
        #[doc = "Generated CXX-Qt wrapper method for the Q_INVOKABLE"]
        #[doc = "invokable_parameters"]
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
        #[doc = "Generated CXX-Qt wrapper method for the Q_INVOKABLE"]
        #[doc = "invokable_return_opaque"]
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
        #[doc = "Generated CXX-Qt wrapper method for the Q_INVOKABLE"]
        #[doc = "invokable_return_trivial"]
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
        #[doc = "Generated CXX-Qt wrapper method for the Q_INVOKABLE"]
        #[doc = "invokable_final"]
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
        #[doc = "Generated CXX-Qt wrapper method for the Q_INVOKABLE"]
        #[doc = "invokable_override"]
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
        #[doc = "Generated CXX-Qt wrapper method for the Q_INVOKABLE"]
        #[doc = "invokable_virtual"]
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
        #[doc = r" Queue the given closure onto the Qt event loop for this QObject"]
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

    #[doc = r" Generated CXX-Qt thread helper for a QObject"]
    pub struct MyObjectCxxQtThreadQueuedFn {
        inner: std::boxed::Box<dyn FnOnce(std::pin::Pin<&mut MyObjectQt>) + Send>,
    }

    #[doc = r" Generated CXX-Qt method which creates a boxed rust struct of a QObject"]
    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }

    #[doc = r" Generated CXX-Qt module containing type alias to the C++ type of the QObjects"]
    pub mod qobject {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObject"]
        #[doc = "\n"]
        #[doc = "Use type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        pub type MyObject = super::MyObjectQt;
    }
}
