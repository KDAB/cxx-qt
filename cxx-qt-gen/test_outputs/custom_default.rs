#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");
        include!("cxx-qt-lib/include/convert.h");
        include ! (< QtCore / QObject >);

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "public"]
        fn getPublic(self: &MyObjectQt) -> i32;
        #[rust_name = "set_public"]
        fn setPublic(self: Pin<&mut MyObjectQt>, value: i32);

        #[cxx_name = "unsafe_rust"]
        fn rust(self: &MyObjectQt) -> &RustObj;
        #[rust_name = "new_cpp_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafe_rust_mut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut RustObj>;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type RustObj;

        #[cxx_name = "createRs"]
        fn create_rs() -> Box<RustObj>;

        #[cxx_name = "initialiseCpp"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
    }
}

pub use self::cxx_qt_my_object::*;
mod cxx_qt_my_object {
    use super::my_object::*;

    pub type FFICppObj = super::my_object::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    pub struct RustObj {
        private: i32,
    }

    impl RustObj {}

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn public(&self) -> i32 {
            self.cpp.public()
        }

        pub fn set_public(&mut self, value: i32) {
            self.cpp.as_mut().set_public(value);
        }

        pub fn grab_values_from_data(&mut self, mut data: Data) {
            self.set_public(data.public);
        }
    }

    pub struct Data {
        public: i32,
    }

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(value: &CppObj<'a>) -> Self {
            Self {
                public: value.public().into(),
            }
        }
    }

    impl<'a> From<&mut CppObj<'a>> for Data {
        fn from(value: &mut CppObj<'a>) -> Self {
            Self::from(&*value)
        }
    }

    impl Default for Data {
        fn default() -> Self {
            Self { public: 32 }
        }
    }

    impl Default for RustObj {
        fn default() -> Self {
            Self { private: 64 }
        }
    }

    pub fn create_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(Data::default());
    }
}
