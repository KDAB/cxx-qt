mod my_object {
    #[cxx::bridge]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            type QString = cxx_qt_lib::QString;
            type SubObject = crate::sub_object::CppObj;

            #[rust_name = "take_obj"]
            fn takeObj(self: Pin<&mut MyObject>) -> UniquePtr<SubObject>;
            #[rust_name = "give_obj"]
            fn giveObj(self: Pin<&mut MyObject>, value: UniquePtr<SubObject>);

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

    struct MyObjectRs;

    struct MyObjectWrapper<'a> {
        cpp: std::pin::Pin<&'a mut CppObj>,
    }

    impl<'a> MyObjectWrapper<'a> {
        fn new(cpp: std::pin::Pin<&'a mut CppObj>) -> Self {
            Self { cpp }
        }

        fn take_obj(&mut self) -> cxx::UniquePtr<ffi::SubObject> {
            self.cpp.as_mut().take_obj()
        }

        fn give_obj(&mut self, value: cxx::UniquePtr<ffi::SubObject>) {
            self.cpp.as_mut().give_obj(value);
        }
    }

    #[derive(Default)]
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
        Box::new(Data::default().into())
    }
}
