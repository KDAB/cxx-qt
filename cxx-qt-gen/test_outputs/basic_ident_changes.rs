mod my_object {
    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            #[namespace = ""]
            type QString = cxx_qt_lib::QString;

            #[rust_name = "my_number"]
            fn getMyNumber(self: &MyObject) -> i32;
            #[rust_name = "set_my_number"]
            fn setMyNumber(self: Pin<&mut MyObject>, value: i32);

            #[rust_name = "new_MyObject"]
            fn newMyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "sayBye"]
            fn say_bye(self: &RustObj);

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<RustObj>;
        }
    }

    pub type CppObj = ffi::MyObject;

    struct RustObj {
        my_number: i32,
    }

    impl RustObj {
        fn say_bye(&self) {
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

        fn my_number(&self) -> i32 {
            self.cpp.my_number()
        }

        fn set_my_number(&mut self, value: i32) {
            self.cpp.as_mut().set_my_number(value);
        }

        fn grab_values_from_data(&mut self, data: &Data) {
            use cxx_qt_lib::MapQtValue;

            data.my_number
                .map_qt_value(|context, converted| context.set_my_number(converted), self);
        }
    }

    #[derive(Default)]
    struct Data {
        my_number: i32,
    }

    impl From<Data> for RustObj {
        fn from(value: Data) -> Self {
            Self {
                my_number: value.my_number,
            }
        }
    }

    impl<'a> From<&CppObjWrapper<'a>> for Data {
        fn from(value: &CppObjWrapper<'a>) -> Self {
            Self {
                my_number: value.my_number().into(),
            }
        }
    }

    fn create_my_object_rs() -> Box<RustObj> {
        Box::new(Data::default().into())
    }
}
