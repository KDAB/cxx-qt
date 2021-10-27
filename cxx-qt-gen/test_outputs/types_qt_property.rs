mod my_object {
    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        enum Property {
            Pointf,
            String,
        }

        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            #[namespace = ""]
            type QPointF = cxx_qt_lib::QPointF;
            #[namespace = ""]
            type QString = cxx_qt_lib::QString;

            #[rust_name = "pointf"]
            fn getPointf(self: &MyObject) -> &QPointF;
            #[rust_name = "set_pointf"]
            fn setPointf(self: Pin<&mut MyObject>, value: &QPointF);

            #[rust_name = "string"]
            fn getString(self: &MyObject) -> &QString;
            #[rust_name = "set_string"]
            fn setString(self: Pin<&mut MyObject>, value: &QString);

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

    pub type CppObj = ffi::MyObject;
    pub type Property = ffi::Property;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {}

    pub struct CppObjWrapper<'a> {
        cpp: std::pin::Pin<&'a mut CppObj>,
    }

    impl<'a> CppObjWrapper<'a> {
        fn new(cpp: std::pin::Pin<&'a mut CppObj>) -> Self {
            Self { cpp }
        }

        pub fn pointf(&self) -> &cxx_qt_lib::QPointF {
            self.cpp.pointf()
        }

        pub fn set_pointf(&mut self, value: &cxx_qt_lib::QPointF) {
            self.cpp.as_mut().set_pointf(value);
        }

        pub fn string(&self) -> &cxx_qt_lib::QString {
            self.cpp.string()
        }

        pub fn set_string(&mut self, value: &cxx_qt_lib::QString) {
            self.cpp.as_mut().set_string(value);
        }

        pub fn update_requester(&self) -> cxx_qt_lib::update_requester::UpdateRequester {
            use cxx_qt_lib::update_requester::{CxxQObject, UpdateRequester};

            let ptr: *const CppObj = unsafe { &*self.cpp.as_ref() };
            unsafe { UpdateRequester::new(ptr as *mut CxxQObject) }
        }

        pub fn grab_values_from_data(&mut self, data: &Data) {
            use cxx_qt_lib::MapQtValue;

            data.pointf
                .map_qt_value(|context, converted| context.set_pointf(converted), self);
            data.string
                .map_qt_value(|context, converted| context.set_string(converted), self);
        }
    }

    #[derive(Default)]
    struct Data {
        pointf: QPointF,
        string: String,
    }

    impl<'a> From<&CppObjWrapper<'a>> for Data {
        fn from(value: &CppObjWrapper<'a>) -> Self {
            Self {
                pointf: value.pointf().into(),
                string: value.string().into(),
            }
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
