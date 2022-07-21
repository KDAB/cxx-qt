#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

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

        #[rust_name = "new_cpp_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;

        #[rust_name = "update_requester"]
        fn updateRequester(self: Pin<&mut MyObjectQt>) -> UniquePtr<UpdateRequester>;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type RustObj;

        #[cxx_name = "createRs"]
        fn create_rs() -> Box<RustObj>;

        #[cxx_name = "initialiseCpp"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);

        #[cxx_name = "handleUpdateRequest"]
        fn call_handle_update_request(self: &mut RustObj, cpp: Pin<&mut MyObjectQt>);
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
    }
}

pub use self::cxx_qt_my_object::*;
mod cxx_qt_my_object {
    use super::my_object::*;

    use cxx_qt_lib::UpdateRequestHandler;

    pub type FFICppObj = super::my_object::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        pub fn call_handle_update_request(&mut self, cpp: std::pin::Pin<&mut FFICppObj>) {
            let mut cpp = CppObj::new(cpp);
            self.handle_update_request(&mut cpp);
        }
    }

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn number(&self) -> i32 {
            self.cpp.number()
        }

        pub fn set_number(&mut self, value: i32) {
            self.cpp.as_mut().set_number(value);
        }

        pub fn string(&self) -> &cxx_qt_lib::QString {
            self.cpp.string()
        }

        pub fn set_string(&mut self, value: &cxx_qt_lib::QString) {
            self.cpp.as_mut().set_string(value);
        }

        pub fn update_requester(&mut self) -> cxx_qt_lib::UpdateRequester {
            cxx_qt_lib::UpdateRequester::from_unique_ptr(self.cpp.as_mut().update_requester())
        }

        pub fn grab_values_from_data(&mut self, mut data: Data) {
            self.set_number(data.number);
            self.set_string(&data.string);
        }
    }

    #[derive(Default)]
    pub struct Data {
        number: i32,
        string: QString,
    }

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(value: &CppObj<'a>) -> Self {
            Self {
                number: value.number().into(),
                string: value.string().into(),
            }
        }
    }

    impl<'a> From<&mut CppObj<'a>> for Data {
        fn from(value: &mut CppObj<'a>) -> Self {
            Self::from(&*value)
        }
    }

    impl UpdateRequestHandler<CppObj> for RustObj {
        fn handle_update_request(&mut self, _cpp: &mut CppObj) {
            println!("update")
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
