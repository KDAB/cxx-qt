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

            #[rust_name = "new_MyObject"]
            fn newMyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "subTest"]
            fn sub_test(self: &RustObj, _cpp: Pin<&mut MyObject>, sub: Pin<&mut SubObject>);

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<RustObj>;
        }
    }

    pub type CppObj = ffi::MyObject;

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

    struct CppObjWrapper<'a> {
        cpp: std::pin::Pin<&'a mut CppObj>,
    }

    impl<'a> CppObjWrapper<'a> {
        fn new(cpp: std::pin::Pin<&'a mut CppObj>) -> Self {
            Self { cpp }
        }

        fn grab_values_from_data(&mut self, data: &Data) {
            use cxx_qt_lib::MapQtValue;
        }
    }

    struct Data;

    impl From<Data> for RustObj {
        fn from(_value: Data) -> Self {
            Self {}
        }
    }

    impl<'a> From<&CppObjWrapper<'a>> for Data {
        fn from(_value: &CppObjWrapper<'a>) -> Self {
            Self {}
        }
    }

    fn create_my_object_rs() -> Box<RustObj> {
        Box::new(Data {}.into())
    }
}
