#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");
        include ! (< QtCore / QObject >);

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "primitive"]
        fn getPrimitive(self: &MyObjectQt) -> i32;
        #[rust_name = "set_primitive"]
        fn setPrimitive(self: Pin<&mut MyObjectQt>, value: i32);

        #[rust_name = "opaque"]
        fn getOpaque(self: &MyObjectQt) -> &QColor;
        #[rust_name = "set_opaque"]
        fn setOpaque(self: Pin<&mut MyObjectQt>, value: &QColor);
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QColor = cxx_qt_lib::QColor;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/include/convert.h");

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

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

        #[cxx_name = "initialiseCpp"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
    }
}

pub use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;

    pub type FFICppObj = super::ffi::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    #[derive(Default)]
    pub struct MyObject;

    impl MyObject {}

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn primitive(&self) -> i32 {
            self.cpp.primitive()
        }

        pub fn set_primitive(&mut self, value: i32) {
            self.cpp.as_mut().set_primitive(value);
        }

        pub fn opaque(&self) -> &cxx_qt_lib::QColor {
            self.cpp.opaque()
        }

        pub fn set_opaque(&mut self, value: &cxx_qt_lib::QColor) {
            self.cpp.as_mut().set_opaque(value);
        }

        pub fn grab_values_from_data(&mut self, mut data: Data) {
            self.set_primitive(data.primitive);
            self.set_opaque(data.opaque.as_ref().unwrap());
        }
    }

    #[derive(Default)]
    pub struct Data {
        primitive: i32,
        opaque: UniquePtr<QColor>,
    }

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(value: &CppObj<'a>) -> Self {
            Self {
                primitive: value.primitive().into(),
                opaque: value.opaque().into(),
            }
        }
    }

    impl<'a> From<&mut CppObj<'a>> for Data {
        fn from(value: &mut CppObj<'a>) -> Self {
            Self::from(&*value)
        }
    }

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(Data::default());
    }
}
