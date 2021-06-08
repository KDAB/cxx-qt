mod my_object {
    #[cxx::bridge]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;

            fn say_hi(self: &MyObject);
            fn say_bye(self: &MyObject);

            fn new_MyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type MyObjectRs;

            fn say_hi(self: &MyObjectRs);
            fn say_bye(self: &MyObjectRs);

            fn create_my_object_rs() -> Box<MyObjectRs>;
        }
    }

    struct MyObjectRs {}

    impl MyObjectRs {
        fn say_hi(&self, string: &str, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }

        fn say_bye(&self) {
            println!("Bye from Rust!");
        }
    }

    fn create_my_object_rs() -> Box<MyObjectRs> {
        Box::new(MyObjectRs {})
    }
}
