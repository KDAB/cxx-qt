#[cxx::bridge(namespace = "")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "property_name"]
        fn getPropertyName(self: &MyObjectQt) -> i32;
        #[rust_name = "set_property_name"]
        fn setPropertyName(self: Pin<&mut MyObjectQt>, value: i32);
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;

        #[cxx_name = "invokableNameWrapper"]
        fn invokable_name_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>);
    }

    unsafe extern "C++" {
        include ! (< QtCore / QStringListModel >);
    }

    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/include/convert.h");

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

        #[rust_name = "new_cpp_object"]
        #[namespace = "cxx_qt_my_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }

    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt_my_object"]
        fn create_rs() -> Box<MyObject>;

        #[cxx_name = "initialiseCpp"]
        #[namespace = "cxx_qt_my_object"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
    }
}

pub use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;

    pub type FFICppObj = super::ffi::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    use std::pin::Pin;

    #[derive(Default)]
    pub struct MyObject;

    impl MyObject {
        pub fn invokable_name_wrapper(&mut self, cpp: std::pin::Pin<&mut FFICppObj>) {
            cpp.invokable_name();
        }
    }

    impl MyObjectQt {
        pub fn invokable_name(self: Pin<&mut Self>) {
            println!("Bye from Rust!");
            self.as_mut().set_property_name(5);
        }

        pub fn grab_values_from_data(mut self: Pin<&mut Self>, mut data: Data) {
            self.as_mut().set_property_name(data.property_name);
        }
    }

    #[derive(Default)]
    pub struct Data {
        property_name: i32,
    }

    impl From<&MyObjectQt> for Data {
        fn from(value: &MyObjectQt) -> Self {
            Self {
                property_name: value.property_name().into(),
            }
        }
    }

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut MyObjectQt>) {
        cpp.grab_values_from_data(Data::default());
    }
}
