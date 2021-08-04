mod my_object {
    #[cxx::bridge]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            type QString = cxx_qt_lib::QString;

            #[rust_name = "my_number"]
            fn getMyNumber(self: &MyObject) -> i32;
            #[rust_name = "set_my_number"]
            fn setMyNumber(self: Pin<&mut MyObject>, value: i32);

            #[rust_name = "new_MyObject"]
            fn newMyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type MyObjectRs;

            #[cxx_name = "sayBye"]
            fn say_bye(self: &MyObjectRs);

            #[cxx_name = "getMyNumber"]
            fn my_number(self: &MyObjectRs) -> &i32;
            #[cxx_name = "setMyNumber"]
            fn set_my_number(self: &mut MyObjectRs, value: i32);

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<MyObjectRs>;
        }
    }

    pub type CppObj = ffi::MyObject;

    #[derive(Default)]
    struct MyObjectRs {
        my_number: i32,
    }

    impl MyObjectRs {
        fn say_bye(&self) {
            println!("Bye from Rust!");
        }

        fn my_number(self: &MyObjectRs) -> &i32 {
            &self.my_number
        }

        fn set_my_number(self: &mut MyObjectRs, value: i32) {
            self.my_number = value;
        }
    }

    #[derive(Default)]
    struct MyObjectData {
        my_number: i32,
    }

    impl From<MyObjectData> for MyObjectRs {
        fn from(value: MyObjectData) -> Self {
            Self {
                my_number: value.my_number,
            }
        }
    }

    impl From<&MyObjectRs> for MyObjectData {
        fn from(value: &MyObjectRs) -> Self {
            Self {
                my_number: value.my_number.clone(),
            }
        }
    }

    fn create_my_object_rs() -> Box<MyObjectRs> {
        Box::new(MyObjectRs::default())
    }
}
