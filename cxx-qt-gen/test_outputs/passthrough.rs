#[cxx::bridge(namespace = "cxx_qt::my_object")]
#[attrA]
#[attrB]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "number"]
        fn getNumber(self: &MyObjectQt) -> i32;
        #[rust_name = "set_number"]
        fn setNumber(self: Pin<&mut MyObjectQt>, value: i32);
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;
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

    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/include/convert.h");
        include!("cxx-qt-lib/include/cxxqt_thread.h");

        type MyObjectCxxQtThread;

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObjectQt) -> UniquePtr<MyObjectCxxQtThread>;
        fn queue(self: &MyObjectCxxQtThread, func: fn(ctx: Pin<&mut MyObjectQt>)) -> Result<()>;

        #[rust_name = "new_cpp_object"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }

    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn create_rs() -> Box<MyObject>;

        #[cxx_name = "initialiseCpp"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
    }
}

pub use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;

    pub type FFICppObj = super::ffi::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    unsafe impl Send for MyObjectCxxQtThread {}

    use std::pin::Pin;

    #[derive(Default)]
    pub struct MyObject;

    impl MyObject {}

    impl MyObjectQt {
        pub fn grab_values_from_data(mut self: Pin<&mut Self>, mut data: Data) {
            self.as_mut().set_number(data.number);
        }
    }

    #[derive(Default)]
    pub struct Data {
        number: i32,
    }

    impl From<&MyObjectQt> for Data {
        fn from(value: &MyObjectQt) -> Self {
            Self {
                number: value.number().into(),
            }
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

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut MyObjectQt>) {
        cpp.grab_values_from_data(Data::default());
    }
}
