mod my_object {
    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        enum Property {
            Obj,
        }

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
            #[namespace = "cxx_qt::sub_object"]
            type SubObject = crate::sub_object::FFICppObj;

            #[rust_name = "take_obj"]
            fn takeObj(self: Pin<&mut MyObject>) -> UniquePtr<SubObject>;
            #[rust_name = "give_obj"]
            fn giveObj(self: Pin<&mut MyObject>, value: UniquePtr<SubObject>);

            #[rust_name = "new_cpp_object"]
            fn newCppObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "createRs"]
            fn create_rs() -> Box<RustObj>;

            #[cxx_name = "initialiseCpp"]
            fn initialise_cpp(cpp: Pin<&mut MyObject>);
        }
    }

    pub type FFICppObj = ffi::MyObject;
    pub type Property = ffi::Property;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {}

    pub struct CppObjWrapper<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObjWrapper<'a> {
        fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn take_obj(&mut self) -> cxx::UniquePtr<ffi::SubObject> {
            self.cpp.as_mut().take_obj()
        }

        pub fn give_obj(&mut self, value: cxx::UniquePtr<ffi::SubObject>) {
            self.cpp.as_mut().give_obj(value);
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

    #[derive(Default)]
    struct Data;

    impl<'a> From<&CppObjWrapper<'a>> for Data {
        fn from(_value: &CppObjWrapper<'a>) -> Self {
            Self {}
        }
    }

    fn create_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObjWrapper::new(cpp);
        wrapper.grab_values_from_data(&Data::default());
    }
}
