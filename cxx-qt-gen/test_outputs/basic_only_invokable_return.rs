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
            type MyObjectRs;

            #[cxx_name = "doubleNumber"]
            fn double_number(self: &MyObjectRs, number: i32) -> i32;
            #[cxx_name = "helloMessage"]
            fn hello_message(self: &MyObjectRs, msg: &str) -> String;
            #[cxx_name = "staticMessage"]
            fn static_message(self: &MyObjectRs) -> &str;

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<MyObjectRs>;
        }
    }

    pub type CppObj = ffi::MyObject;

    struct MyObjectRs;

    impl MyObjectRs {
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

    struct MyObjectWrapper<'a> {
        cpp: std::pin::Pin<&'a mut CppObj>,
    }

    impl<'a> MyObjectWrapper<'a> {
        fn new(cpp: std::pin::Pin<&'a mut CppObj>) -> Self {
            Self { cpp }
        }
    }

    struct Data;

    impl From<Data> for MyObjectRs {
        fn from(_value: Data) -> Self {
            Self {}
        }
    }

    impl From<&MyObjectRs> for Data {
        fn from(_value: &MyObjectRs) -> Self {
            Self {}
        }
    }

    fn create_my_object_rs() -> Box<MyObjectRs> {
        Box::new(Data {}.into())
    }
}
