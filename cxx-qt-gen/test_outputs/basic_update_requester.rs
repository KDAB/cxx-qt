mod my_object {
    use cxx_qt_lib::QString;
    use cxx_qt_lib::UpdateRequestHandler;

    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        enum Property {}

        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            #[namespace = ""]
            type QPointF = cxx_qt_lib::QPointF;
            #[namespace = ""]
            type QString = cxx_qt_lib::QString;
            #[namespace = ""]
            type QVariant = cxx_qt_lib::QVariant;

            #[namespace = "CxxQt"]
            type Variant = cxx_qt_lib::Variant;

            #[rust_name = "new_cpp_object"]
            fn newCppObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "sayHi"]
            fn say_hi(self: &RustObj, string: &QString, number: i32);
            #[cxx_name = "sayBye"]
            fn say_bye(self: &RustObj);

            #[cxx_name = "createRs"]
            fn create_rs() -> Box<RustObj>;

            #[cxx_name = "initialiseCpp"]
            fn initialise_cpp(cpp: Pin<&mut MyObject>);

            #[cxx_name = "handleUpdateRequest"]
            fn call_handle_update_request(self: &mut RustObj, cpp: Pin<&mut MyObject>);
        }
    }

    pub type FFICppObj = ffi::MyObject;
    pub type Property = ffi::Property;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        fn say_hi(&self, string: &QString, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }

        fn say_bye(&self) {
            println!("Bye from Rust!");
        }

        fn call_handle_update_request(&mut self, cpp: std::pin::Pin<&mut FFICppObj>) {
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

        pub fn update_requester(&self) -> cxx_qt_lib::update_requester::UpdateRequester {
            use cxx_qt_lib::update_requester::{CxxQObject, UpdateRequester};

            let ptr: *const FFICppObj = unsafe { &*self.cpp.as_ref() };
            unsafe { UpdateRequester::new(ptr as *mut CxxQObject) }
        }

        pub fn grab_values_from_data(&mut self, data: &Data) {
            use cxx_qt_lib::MapQtValue;
        }
    }

    struct Data;

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(_value: &CppObj<'a>) -> Self {
            Self {}
        }
    }

    impl UpdateRequestHandler<CppObj> for RustObj {
        fn handle_update_request(&mut self, _cpp: &mut CppObj) {
            println!("update")
        }
    }

    fn create_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(&Data::default());
    }
}
