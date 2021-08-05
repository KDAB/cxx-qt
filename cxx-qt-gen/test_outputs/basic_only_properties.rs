mod my_object {
    #[cxx::bridge]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            type QString = cxx_qt_lib::QString;

            #[rust_name = "number"]
            fn getNumber(self: &MyObject) -> i32;
            #[rust_name = "set_number"]
            fn setNumber(self: Pin<&mut MyObject>, value: i32);

            #[rust_name = "string"]
            fn getString(self: &MyObject) -> &QString;
            #[rust_name = "set_string"]
            fn setString(self: Pin<&mut MyObject>, value: &QString);

            #[rust_name = "new_MyObject"]
            fn newMyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type MyObjectRs;

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<MyObjectRs>;
        }
    }

    pub type CppObj = ffi::MyObject;

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

    #[derive(Default)]
    struct MyObjectData {
        number: i32,
        string: String,
    }

    impl From<MyObjectData> for MyObjectRs {
        fn from(value: MyObjectData) -> Self {
            Self {
                number: value.number,
                string: value.string,
            }
        }
    }

    impl From<&MyObjectRs> for MyObjectData {
        fn from(value: &MyObjectRs) -> Self {
            Self {
                number: value.number.clone(),
                string: value.string.clone(),
            }
        }
    }

    fn create_my_object_rs() -> Box<MyObjectRs> {
        Box::new(MyObjectRs::default())
    }
}
