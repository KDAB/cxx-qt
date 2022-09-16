#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/include/convert.h");
        include!("cxx-qt-lib/include/cxxqt_thread.h");
    }

    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "emit_public_changed"]
        fn emitPublicChanged(self: Pin<&mut MyObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;

        #[cxx_name = "getPublic"]
        unsafe fn get_public<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i32;
        #[cxx_name = "setPublic"]
        fn set_public(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i32);
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

    impl Default for MyObject {
        fn default() -> Self {
            Self {
                public: 32,
                private: 64,
            }
        }
    }

    pub struct MyObject {
        public: i32,
        private: i32,
    }

    impl MyObject {
        pub fn get_public<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
            cpp.get_public()
        }

        pub fn set_public(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
            cpp.set_public(value);
        }
    }

    impl MyObjectQt {
        pub fn get_public(&self) -> &i32 {
            &self.rust().public
        }

        pub fn set_public(mut self: Pin<&mut Self>, value: i32) {
            unsafe {
                self.as_mut().rust_mut().public = value;
            }
            self.as_mut().emit_public_changed();
        }
    }

    unsafe impl Send for MyObjectCxxQtThread {}

    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
}
