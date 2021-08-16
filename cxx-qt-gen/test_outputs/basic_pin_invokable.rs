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

            #[cxx_name = "sayHi"]
            fn say_hi(self: &RustObj, _cpp: Pin<&mut MyObject>, string: &str, number: i32);
            #[cxx_name = "sayBye"]
            fn say_bye(self: &RustObj, _cpp: Pin<&mut MyObject>);

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<RustObj>;
        }
    }

    pub type CppObj = ffi::MyObject;

    struct RustObj;

    impl RustObj {
        fn say_hi(&self, _cpp: std::pin::Pin<&mut CppObj>, string: &str, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }

        fn say_bye(&self, _cpp: std::pin::Pin<&mut CppObj>) {
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
