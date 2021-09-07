mod my_object {
    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            #[namespace = ""]
            type QString = cxx_qt_lib::QString;
            #[namespace = "cxx_qt::sub_object"]
            type SubObject = crate::sub_object::CppObj;

            #[rust_name = "take_obj"]
            fn takeObj(self: Pin<&mut MyObject>) -> UniquePtr<SubObject>;
            #[rust_name = "give_obj"]
            fn giveObj(self: Pin<&mut MyObject>, value: UniquePtr<SubObject>);

            #[rust_name = "new_MyObject"]
            fn newMyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<RustObj>;

            #[cxx_name = "initialiseMyObjectCpp"]
            fn initialise_my_object_cpp(cpp: Pin<&mut MyObject>);
        }
    }

    pub type CppObj = ffi::MyObject;

    #[derive(Default)]
    struct RustObj;

    struct CppObjWrapper<'a> {
        cpp: std::pin::Pin<&'a mut CppObj>,
    }

    impl<'a> CppObjWrapper<'a> {
        fn new(cpp: std::pin::Pin<&'a mut CppObj>) -> Self {
            Self { cpp }
        }

        fn take_obj(&mut self) -> cxx::UniquePtr<ffi::SubObject> {
            self.cpp.as_mut().take_obj()
        }

        fn give_obj(&mut self, value: cxx::UniquePtr<ffi::SubObject>) {
            self.cpp.as_mut().give_obj(value);
        }

        fn update_requester(&self) -> cxx_qt_lib::update_requester::UpdateRequester {
            use cxx_qt_lib::update_requester::{CxxQObject, UpdateRequester};

            let ptr: *const CppObj = unsafe { &*self.cpp.as_ref() };
            unsafe { UpdateRequester::new(ptr as *mut CxxQObject) }
        }

        fn grab_values_from_data(&mut self, data: &Data) {
            use cxx_qt_lib::MapQtValue;
        }
    }

    #[derive(Default)]
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
