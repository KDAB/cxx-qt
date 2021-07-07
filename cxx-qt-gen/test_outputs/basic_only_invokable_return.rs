mod my_object {
    #[cxx::bridge]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;

            #[rust_name = "new_MyObject"]
            fn newMyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type MyObjectRs;

            fn double_number(self: &MyObjectRs, number: i32) -> i32;
            fn hello_message(self: &MyObjectRs, msg: &str) -> String;
            fn static_message(self: &MyObjectRs) -> &str;

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<MyObjectRs>;
        }
    }

    struct MyObjectRs {}

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

    fn create_my_object_rs() -> Box<MyObjectRs> {
        Box::new(MyObjectRs {})
    }
}
