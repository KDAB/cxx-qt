#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");
        include!("cxx-qt-lib/include/update_requester.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[namespace = "rust::cxxqtlib1"]
        type UpdateRequester = cxx_qt_lib::UpdateRequesterCpp;

        #[rust_name = "number"]
        fn getNumber(self: &MyObjectQt) -> i32;
        #[rust_name = "set_number"]
        fn setNumber(self: Pin<&mut MyObjectQt>, value: i32);

        #[rust_name = "string"]
        fn getString(self: &MyObjectQt) -> &QString;
        #[rust_name = "set_string"]
        fn setString(self: Pin<&mut MyObjectQt>, value: &QString);

        #[rust_name = "update_requester"]
        fn updateRequester(self: Pin<&mut MyObjectQt>) -> UniquePtr<UpdateRequester>;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;

        #[cxx_name = "handleUpdateRequest"]
        fn call_handle_update_request(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>);
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
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

    use cxx_qt_lib::UpdateRequestHandler;

    use std::pin::Pin;

    #[derive(Default)]
    pub struct MyObject;

    impl MyObject {
        pub fn call_handle_update_request(&mut self, cpp: std::pin::Pin<&mut FFICppObj>) {
            cpp.handle_update_request();
        }
    }

    impl MyObjectQt {
        pub fn grab_values_from_data(mut self: Pin<&mut Self>, mut data: Data) {
            self.as_mut().set_number(data.number);
            self.as_mut().set_string(&data.string);
        }
    }

    #[derive(Default)]
    pub struct Data {
        number: i32,
        string: QString,
    }

    impl From<&MyObjectQt> for Data {
        fn from(value: &MyObjectQt) -> Self {
            Self {
                number: value.number().into(),
                string: value.string().into(),
            }
        }
    }

    impl UpdateRequestHandler for MyObjectQt {
        fn handle_update_request(self: Pin<&mut Self>) {
            println!("update")
        }
    }

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut MyObjectQt>) {
        cpp.grab_values_from_data(Data::default());
    }
}
