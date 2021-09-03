mod my_object {
    use super::MyTrait;

    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            #[namespace = ""]
            type QString = cxx_qt_lib::QString;

            #[rust_name = "number"]
            fn getNumber(self: &MyObject) -> i32;
            #[rust_name = "set_number"]
            fn setNumber(self: Pin<&mut MyObject>, value: i32);

            #[rust_name = "new_MyObject"]
            fn newMyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "invokable"]
            fn invokable(self: &RustObj);

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<RustObj>;
        }
    }

    pub type CppObj = ffi::MyObject;

    struct RustObj {
        number: i32,
    }

    impl RustObj {
        fn invokable(&self) {}
    }

    struct CppObjWrapper<'a> {
        cpp: std::pin::Pin<&'a mut CppObj>,
    }

    impl<'a> CppObjWrapper<'a> {
        fn new(cpp: std::pin::Pin<&'a mut CppObj>) -> Self {
            Self { cpp }
        }

        fn number(&self) -> i32 {
            self.cpp.number()
        }

        fn set_number(&mut self, value: i32) {
            self.cpp.as_mut().set_number(value);
        }
    }

    struct Data {
        number: i32,
    }

    impl From<Data> for RustObj {
        fn from(value: Data) -> Self {
            Self {
                number: value.number,
            }
        }
    }

    impl<'a> From<&CppObjWrapper<'a>> for Data {
        fn from(value: &CppObjWrapper<'a>) -> Self {
            Self {
                number: value.number().into(),
            }
        }
    }

    impl Default for Data {
        fn default() -> Self {
            Self { number: 32 }
        }
    }

    impl MyTrait for RustObj {
        fn my_func() -> String {
            "Hello".to_owned()
        }
    }

    fn create_my_object_rs() -> Box<RustObj> {
        Box::new(Data::default().into())
    }
}
