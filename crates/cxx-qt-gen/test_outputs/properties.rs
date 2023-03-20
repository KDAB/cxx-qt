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
        include!("cxx-qt-lib/qmetaobjectconnection.h");
        #[doc(hidden)]
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "CxxQtQMetaObjectConnection"]
        type QMetaObjectConnection = cxx_qt_lib::QMetaObjectConnection;
    }
    unsafe extern "C++" {
        include!("cxx-qt-gen/ffi.cxxqt.h");
    }
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObject"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
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
        #[cxx_name = "getPrimitive"]
        unsafe fn primitive<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPrimitive"]
        fn set_primitive(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i32);
    }
    unsafe extern "C++" {
        #[doc = "Notify signal for the Q_PROPERTY"]
        #[doc = "primitive"]
        #[doc = "\n"]
        #[doc = "This can be used to manually notify a change when the unsafe mutable getter,"]
        #[doc = "primitive_mut"]
        #[doc = ", is used."]
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
        #[doc = "Notify signal for the Q_PROPERTY"]
        #[doc = "trivial"]
        #[doc = "\n"]
        #[doc = "This can be used to manually notify a change when the unsafe mutable getter,"]
        #[doc = "trivial_mut"]
        #[doc = ", is used."]
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
        #[doc = "Notify signal for the Q_PROPERTY"]
        #[doc = "opaque"]
        #[doc = "\n"]
        #[doc = "This can be used to manually notify a change when the unsafe mutable getter,"]
        #[doc = "opaque_mut"]
        #[doc = ", is used."]
        #[rust_name = "opaque_changed"]
        fn opaqueChanged(self: Pin<&mut MyObjectQt>);
    }
    unsafe extern "C++" {
        #[doc = r" Specialised version of CxxQtThread, which can be moved into other threads."]
        #[doc = r""]
        #[doc = r" CXX doesn't support having generic types in the function yet"]
        #[doc = r" so we cannot have CxxQtThread<T> in cxx-qt-lib and then use that here"]
        #[doc = r" For now we use a type alias in C++ then use it like a normal type here"]
        #[doc = r" <https://github.com/dtolnay/cxx/issues/683>"]
        type MyObjectCxxQtThread;
        #[doc = r" Retrieve an immutable reference to the Rust struct backing this C++ object"]
        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;
        #[doc = r" Create an instance of a CxxQtThread"]
        #[doc = r""]
        #[doc = r" This allows for queueing closures onto the Qt event loop from a background thread."]
        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObjectQt) -> UniquePtr<MyObjectCxxQtThread>;
        #[doc(hidden)]
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
        #[doc = r" This method is unsafe because it allows a Q_PROPERTY to be modified without emitting its changed signal."]
        #[doc = r" The property changed signal must be emitted manually."]
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
use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;
    use std::pin::Pin;
    #[doc(hidden)]
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
        #[doc(hidden)]
        pub fn primitive<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
            cpp.primitive()
        }
    }
    impl MyObjectQt {
        #[doc = "Getter for the Q_PROPERTY "]
        #[doc = "primitive"]
        pub fn primitive(&self) -> &i32 {
            &self.rust().primitive
        }
    }
    impl MyObjectQt {
        #[doc = "unsafe getter for the Q_PROPERTY "]
        #[doc = "primitive"]
        #[doc = "\n"]
        #[doc = "This allows for modifying the Q_PROPERTY without calling the property changed Q_SIGNAL"]
        #[doc = "\n"]
        #[doc = "After modifying the property, make sure to call the corresponding changed signal: "]
        #[doc = "primitive_changed"]
        pub unsafe fn primitive_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut i32 {
            &mut self.rust_mut().get_unchecked_mut().primitive
        }
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn set_primitive(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
            cpp.set_primitive(value);
        }
    }
    impl MyObjectQt {
        #[doc = "Setter for the Q_PROPERTY "]
        #[doc = "primitive"]
        pub fn set_primitive(mut self: Pin<&mut Self>, value: i32) {
            if self.rust().primitive == value {
                return;
            }
            unsafe {
                self.as_mut().rust_mut().primitive = value;
            }
            self.as_mut().primitive_changed();
        }
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn trivial<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a QPoint {
            cpp.trivial()
        }
    }
    impl MyObjectQt {
        #[doc = "Getter for the Q_PROPERTY "]
        #[doc = "trivial"]
        pub fn trivial(&self) -> &QPoint {
            &self.rust().trivial
        }
    }
    impl MyObjectQt {
        #[doc = "unsafe getter for the Q_PROPERTY "]
        #[doc = "trivial"]
        #[doc = "\n"]
        #[doc = "This allows for modifying the Q_PROPERTY without calling the property changed Q_SIGNAL"]
        #[doc = "\n"]
        #[doc = "After modifying the property, make sure to call the corresponding changed signal: "]
        #[doc = "trivial_changed"]
        pub unsafe fn trivial_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut QPoint {
            &mut self.rust_mut().get_unchecked_mut().trivial
        }
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn set_trivial(&mut self, cpp: Pin<&mut MyObjectQt>, value: QPoint) {
            cpp.set_trivial(value);
        }
    }
    impl MyObjectQt {
        #[doc = "Setter for the Q_PROPERTY "]
        #[doc = "trivial"]
        pub fn set_trivial(mut self: Pin<&mut Self>, value: QPoint) {
            if self.rust().trivial == value {
                return;
            }
            unsafe {
                self.as_mut().rust_mut().trivial = value;
            }
            self.as_mut().trivial_changed();
        }
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn opaque<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a UniquePtr<Opaque> {
            cpp.opaque()
        }
    }
    impl MyObjectQt {
        #[doc = "Getter for the Q_PROPERTY "]
        #[doc = "opaque"]
        pub fn opaque(&self) -> &UniquePtr<Opaque> {
            &self.rust().opaque
        }
    }
    impl MyObjectQt {
        #[doc = "unsafe getter for the Q_PROPERTY "]
        #[doc = "opaque"]
        #[doc = "\n"]
        #[doc = "This allows for modifying the Q_PROPERTY without calling the property changed Q_SIGNAL"]
        #[doc = "\n"]
        #[doc = "After modifying the property, make sure to call the corresponding changed signal: "]
        #[doc = "opaque_changed"]
        pub unsafe fn opaque_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut UniquePtr<Opaque> {
            &mut self.rust_mut().get_unchecked_mut().opaque
        }
    }
    impl MyObject {
        #[doc(hidden)]
        pub fn set_opaque(&mut self, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<Opaque>) {
            cpp.set_opaque(value);
        }
    }
    impl MyObjectQt {
        #[doc = "Setter for the Q_PROPERTY "]
        #[doc = "opaque"]
        pub fn set_opaque(mut self: Pin<&mut Self>, value: UniquePtr<Opaque>) {
            if self.rust().opaque == value {
                return;
            }
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
        fn private_rust_field_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut i32 {
            unsafe { &mut self.rust_mut().get_unchecked_mut().private_rust_field }
        }
    }
    impl MyObjectQt {
        fn set_private_rust_field(self: Pin<&mut Self>, value: i32) {
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
        pub fn public_rust_field_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut f64 {
            unsafe { &mut self.rust_mut().get_unchecked_mut().public_rust_field }
        }
    }
    impl MyObjectQt {
        pub fn set_public_rust_field(self: Pin<&mut Self>, value: f64) {
            unsafe {
                self.rust_mut().public_rust_field = value;
            }
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
            #[doc(hidden)]
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
    #[doc(hidden)]
    pub struct MyObjectCxxQtThreadQueuedFn {
        inner: std::boxed::Box<dyn FnOnce(std::pin::Pin<&mut MyObjectQt>) + Send>,
    }
    #[doc = r" Generated CXX-Qt method which creates a boxed rust struct of a QObject"]
    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
    #[doc = r" Generated CXX-Qt module containing type alias to the C++ types of the QObjects"]
    pub mod qobject {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObject"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        pub type MyObject = super::MyObjectQt;
    }
}
