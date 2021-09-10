mod my_object {
    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        enum Property {}

        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            #[namespace = ""]
            type QString = cxx_qt_lib::QString;
            #[namespace = "cxx_qt::sub_object"]
            type SubObject = crate::sub_object::CppObj;

            #[rust_name = "new_MyObject"]
            fn newMyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "subTest"]
            fn sub_test(self: &RustObj, _cpp: Pin<&mut MyObject>, sub: Pin<&mut SubObject>);

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<RustObj>;

            #[cxx_name = "initialiseMyObjectCpp"]
            fn initialise_my_object_cpp(cpp: Pin<&mut MyObject>);
        }
    }

    pub type CppObj = ffi::MyObject;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        fn sub_test(
            &self,
            _cpp: std::pin::Pin<&mut CppObj>,
            sub: std::pin::Pin<&mut crate::sub_object::CppObj>,
        ) {
            println!("Bye from Rust!");
        }
    }

    pub struct CppObjWrapper<'a> {
        cpp: std::pin::Pin<&'a mut CppObj>,
    }

    impl<'a> CppObjWrapper<'a> {
        fn new(cpp: std::pin::Pin<&'a mut CppObj>) -> Self {
            Self { cpp }
        }

        pub fn update_requester(&self) -> cxx_qt_lib::update_requester::UpdateRequester {
            use cxx_qt_lib::update_requester::{CxxQObject, UpdateRequester};

            let ptr: *const CppObj = unsafe { &*self.cpp.as_ref() };
            unsafe { UpdateRequester::new(ptr as *mut CxxQObject) }
        }

        pub fn grab_values_from_data(&mut self, data: &Data) {
            use cxx_qt_lib::MapQtValue;
        }
    }

    struct Data;

    impl<'a> From<&CppObjWrapper<'a>> for Data {
        fn from(_value: &CppObjWrapper<'a>) -> Self {
            Self {}
        }
    }

    fn create_my_object_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    fn initialise_my_object_cpp(cpp: std::pin::Pin<&mut CppObj>) {
        let mut wrapper = CppObjWrapper::new(cpp);
        wrapper.grab_values_from_data(&Data::default());
    }
}
