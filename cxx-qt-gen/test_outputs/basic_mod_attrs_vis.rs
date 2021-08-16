#[attrA]
#[attrB]
pub mod my_object {
    #[cxx::bridge]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            type QString = cxx_qt_lib::QString;

            #[rust_name = "new_MyObject"]
            fn newMyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObjRs;

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<RustObjRs>;
        }
    }

    pub type CppObj = ffi::MyObject;

    struct RustObjRs;

    struct RustObjWrapper<'a> {
        cpp: std::pin::Pin<&'a mut CppObj>,
    }

    impl<'a> RustObjWrapper<'a> {
        fn new(cpp: std::pin::Pin<&'a mut CppObj>) -> Self {
            Self { cpp }
        }
    }

    struct Data;

    impl From<Data> for RustObjRs {
        fn from(_value: Data) -> Self {
            Self {}
        }
    }

    impl From<&RustObjRs> for Data {
        fn from(_value: &RustObjRs) -> Self {
            Self {}
        }
    }

    fn create_my_object_rs() -> Box<RustObjRs> {
        Box::new(Data {}.into())
    }
}
