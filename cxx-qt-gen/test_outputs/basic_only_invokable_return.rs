mod my_object {
    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            #[namespace = ""]
            type QString = cxx_qt_lib::QString;

            #[rust_name = "new_MyObject"]
            fn newMyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "doubleNumber"]
            fn double_number(self: &RustObj, number: i32) -> i32;
            #[cxx_name = "helloMessage"]
            fn hello_message(self: &RustObj, msg: &str) -> String;
            #[cxx_name = "staticMessage"]
            fn static_message(self: &RustObj) -> &str;

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<RustObj>;
        }
    }

    pub type CppObj = ffi::MyObject;

    struct RustObj;

    impl RustObj {
        fn double_number(&self, number: i32) -> i32 {
            number * 2
        }

        fn hello_message(&self, msg: &str) -> String {
            format!("Hello {}", msg)
        }

        fn static_message(&self) -> &str {
            "Hello"
        }
    }

    struct CppObjWrapper<'a> {
        cpp: std::pin::Pin<&'a mut CppObj>,
    }

    impl<'a> CppObjWrapper<'a> {
        fn new(cpp: std::pin::Pin<&'a mut CppObj>) -> Self {
            Self { cpp }
        }
    }

    struct Data;

    impl From<Data> for RustObj {
        fn from(_value: Data) -> Self {
            Self {}
        }
    }

    impl From<&RustObj> for Data {
        fn from(_value: &RustObj) -> Self {
            Self {}
        }
    }

    fn create_my_object_rs() -> Box<RustObj> {
        Box::new(Data {}.into())
    }
}
