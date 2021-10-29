mod my_object {
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

            #[cxx_name = "testPointf"]
            fn test_pointf(self: &RustObj, _cpp: Pin<&mut MyObject>, pointf: &QPointF) -> QPointF;

            #[cxx_name = "testString"]
            fn test_string(self: &RustObj, _cpp: Pin<&mut MyObject>, string: &QString) -> String;

            #[cxx_name = "testVariant"]
            fn test_variant(self: &RustObj, _cpp: Pin<&mut MyObject>, variant: &QVariant) -> Variant;

            #[cxx_name = "createRs"]
            fn create_rs() -> Box<RustObj>;

            #[cxx_name = "initialiseCpp"]
            fn initialise_cpp(cpp: Pin<&mut MyObject>);
        }
    }

    pub type CppObj = ffi::MyObject;
    pub type Property = ffi::Property;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        fn test_pointf(&self, _cpp: std::pin::Pin<&mut CppObj>, pointf: &QPointF) -> QPointF {
            pointf
        }

        fn test_string(
            &self,
            _cpp: std::pin::Pin<&mut CppObj>,
            string: &cxx_qt_lib::QString,
        ) -> String {
            string.to_rust()
        }

        fn test_variant(&self, _cpp: std::pin::Pin<&mut CppObj>, variant: &QVariant) -> Variant {
            variant
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

    fn initialise_cpp(cpp: std::pin::Pin<&mut CppObj>) {
        let mut wrapper = CppObjWrapper::new(cpp);
        wrapper.grab_values_from_data(&Data::default());
    }
}
