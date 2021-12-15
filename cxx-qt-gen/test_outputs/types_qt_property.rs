mod my_object {
    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        enum Property {
            Pointf,
            String,
            Variant,
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

            #[rust_name = "pointf"]
            fn getPointf(self: &MyObject) -> &QPointF;
            #[rust_name = "set_pointf"]
            fn setPointf(self: Pin<&mut MyObject>, value: &QPointF);

            #[rust_name = "string"]
            fn getString(self: &MyObject) -> &QString;
            #[rust_name = "set_string"]
            fn setString(self: Pin<&mut MyObject>, value: &QString);

            #[rust_name = "variant"]
            fn getVariant(self: &MyObject) -> &QVariant;
            #[rust_name = "set_variant"]
            fn setVariant(self: Pin<&mut MyObject>, value: &QVariant);

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

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
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

        pub fn variant(&self) -> &cxx_qt_lib::QVariant {
            self.cpp.variant()
        }

        pub fn set_variant(&mut self, value: &cxx_qt_lib::QVariant) {
            self.cpp.as_mut().set_variant(value);
        }

        pub fn update_requester(&self) -> cxx_qt_lib::update_requester::UpdateRequester {
            use cxx_qt_lib::update_requester::{CxxQObject, UpdateRequester};

            let ptr: *const FFICppObj = unsafe { &*self.cpp.as_ref() };
            unsafe { UpdateRequester::new(ptr as *mut CxxQObject) }
        }

        pub fn grab_values_from_data(&mut self, data: &Data) {
            use cxx_qt_lib::MapQtValue;

            data.pointf
                .map_qt_value(|context, converted| context.set_pointf(converted), self);
            data.string
                .map_qt_value(|context, converted| context.set_string(converted), self);
            data.variant
                .map_qt_value(|context, converted| context.set_variant(converted), self);
        }
    }

    #[derive(Default)]
    struct Data {
        pointf: QPointF,
        string: String,
        variant: Variant,
    }

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(value: &CppObj<'a>) -> Self {
            Self {
                pointf: value.pointf().into(),
                string: value.string().into(),
                variant: value.variant().into(),
            }
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
