#[cxx::bridge(namespace = "cxx_qt::my_object")]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "number"]
        fn getNumber(self: &MyObjectQt) -> i32;
        #[rust_name = "set_number"]
        fn setNumber(self: Pin<&mut MyObjectQt>, value: i32);

        #[cxx_name = "unsafe_rust"]
        fn rust(self: &MyObjectQt) -> &MyObject;
        #[rust_name = "new_cpp_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafe_rust_mut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;

        #[cxx_name = "createRs"]
        fn create_rs() -> Box<MyObject>;

        #[cxx_name = "initialiseCpp"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
    }

    const MAX: u16 = 65535;

    enum Event {
        MyEvent,
    }

    extern crate serde;

    fn do_something() {
        println!("I am a free function");
    }

    extern "C" {}

    #[namespace = "namespace"]
    extern "C" {}

    #[namespace = "namespace"]
    #[custom_attr = "test"]
    extern "C" {}

    unsafe extern "C++" {}

    #[namespace = "namespace"]
    unsafe extern "C++" {}

    #[namespace = "namespace"]
    #[custom_attr = "test"]
    unsafe extern "C++" {}

    macro_rules! macro1 {
        () => {
            0
        };
    }

    macro macro2() {
        0
    }

    mod m {}

    static BIKE: Event = Event::MyEvent;

    pub trait CustomTrait {
        fn method();
    }

    pub trait SharableIterator = CustomTrait + Sync;

    type Result<T> = std::result::Result<T, Event>;

    union Foo<A, B> {
        x: A,
        y: B,
    }
}

pub use self::cxx_qt_ffi::*;
#[attrA]
#[attrB]
pub mod cxx_qt_ffi {
    use super::ffi::*;

    pub type FFICppObj = super::ffi::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    #[derive(Default)]
    pub struct MyObject;

    impl MyObject {}

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn number(&self) -> i32 {
            self.cpp.number()
        }

        pub fn set_number(&mut self, value: i32) {
            self.cpp.as_mut().set_number(value);
        }

        pub fn grab_values_from_data(&mut self, mut data: Data) {
            self.set_number(data.number);
        }
    }

    #[derive(Default)]
    pub struct Data {
        number: i32,
    }

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(value: &CppObj<'a>) -> Self {
            Self {
                number: value.number().into(),
            }
        }
    }

    impl<'a> From<&mut CppObj<'a>> for Data {
        fn from(value: &mut CppObj<'a>) -> Self {
            Self::from(&*value)
        }
    }

    use super::MyTrait;

    impl MyTrait for Data {
        fn my_func() -> String {
            "Hello".to_owned()
        }
    }

    impl MyObject {
        fn test_angled(&self, optional: Option<bool>) -> Option<bool> {
            optional
        }

        fn test_unknown(&self, unknown: MyType) -> MyType {
            unknown
        }
    }

    impl MyTrait for MyObject {
        fn my_func() -> String {
            "Hello".to_owned()
        }
    }

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(Data::default());
    }
}
