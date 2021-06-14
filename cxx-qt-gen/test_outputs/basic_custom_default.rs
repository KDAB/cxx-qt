mod my_object {
    #[cxx::bridge]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;

            fn new_MyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type MyObjectRs;

            fn invokable(self: &MyObjectRs);

            fn number(self: &MyObjectRs) -> &i32;
            fn set_number(self: &mut MyObjectRs, value: i32);

            fn create_my_object_rs() -> Box<MyObjectRs>;
        }
    }

    struct MyObjectRs {
        number: i32,
    }

    impl MyObjectRs {
        fn invokable(&self) {}

        fn number(self: &MyObjectRs) -> &i32 {
            &self.number
        }

        fn set_number(self: &mut MyObjectRs, value: i32) {
            self.number = value;
        }
    }

    impl Default for MyObjectRs {
        fn default() -> Self {
            Self {
                number: 32,
            }
        }
    }

    fn create_my_object_rs() -> Box<MyObjectRs> {
        Box::new(MyObjectRs::default())
    }
}
