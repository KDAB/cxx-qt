mod my_object {
    #[cxx::bridge]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
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

    #[derive(Default)]
    struct MyObjectRs;

    #[derive(Default)]
    struct MyObjectData;

    fn create_my_object_rs() -> Box<MyObjectRs> {
        Box::new(MyObjectRs::default())
    }
}
