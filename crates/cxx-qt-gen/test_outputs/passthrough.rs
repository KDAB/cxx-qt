#[cxx::bridge(namespace = "cxx_qt::my_object")]
#[attrA]
#[attrB]
pub mod ffi {
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
    }

    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "emit_number_changed"]
        fn emitNumberChanged(self: Pin<&mut MyObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;
        #[cxx_name = "getNumber"]
        unsafe fn get_number<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i32;
        #[cxx_name = "setNumber"]
        fn set_number(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i32);
    }

    unsafe extern "C++" {
        type MyObjectCxxQtThread;

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObjectQt) -> UniquePtr<MyObjectCxxQtThread>;
        fn queue(self: &MyObjectCxxQtThread, func: fn(ctx: Pin<&mut MyObjectQt>)) -> Result<()>;

        #[rust_name = "new_cpp_object_my_object_qt"]
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
        fn create_rs_my_object() -> Box<MyObject>;
    }
}

pub use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;

    type UniquePtr<T> = cxx::UniquePtr<T>;

    use std::pin::Pin;

    #[derive(Default)]
    pub struct MyObject {
        number: i32,
    }

    impl MyObject {
        pub fn get_number<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
            cpp.get_number()
        }

        pub fn set_number(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
            cpp.set_number(value);
        }
    }

    impl MyObjectQt {
        pub fn get_number(&self) -> &i32 {
            &self.rust().number
        }

        pub fn set_number(mut self: Pin<&mut Self>, value: i32) {
            unsafe {
                self.as_mut().rust_mut().number = value;
            }
            self.as_mut().emit_number_changed();
        }
    }

    use super::MyTrait;

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

    unsafe impl Send for MyObjectCxxQtThread {}

    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
}
