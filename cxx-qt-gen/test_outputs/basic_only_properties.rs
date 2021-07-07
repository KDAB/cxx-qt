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

            #[cxx_name = "getNumber"]
            fn number(self: &MyObjectRs) -> &i32;
            #[cxx_name = "setNumber"]
            fn set_number(self: &mut MyObjectRs, value: i32);

            #[cxx_name = "getString"]
            fn string(self: &MyObjectRs) -> &String;
            #[cxx_name = "setString"]
            fn set_string(self: &mut MyObjectRs, value: String);

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<MyObjectRs>;
        }
    }

    #[derive(Default)]
    struct MyObjectRs {
        number: i32,
        string: String,
    }

    impl MyObjectRs {
        fn number(self: &MyObjectRs) -> &i32 {
            &self.number
        }

        fn set_number(self: &mut MyObjectRs, value: i32) {
            self.number = value;
        }

        fn string(self: &MyObjectRs) -> &String {
            &self.string
        }

        fn set_string(self: &mut MyObjectRs, value: String) {
            self.string = value;
        }
    }

    fn create_my_object_rs() -> Box<MyObjectRs> {
        Box::new(MyObjectRs::default())
    }
}
