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
        include ! (< QtCore / QStringListModel >);
    }

    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/include/convert.h");
        include!("cxx-qt-lib/include/cxxqt_thread.h");
    }

    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");
    }

    unsafe extern "C++" {
        #[cxx_name = "MyObject"]
        type MyObjectQt;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;
    }

    extern "Rust" {
        #[cxx_name = "getPropertyName"]
        unsafe fn property_name<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i32;
    }

    extern "Rust" {
        #[cxx_name = "setPropertyName"]
        fn set_property_name(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i32);
    }

    unsafe extern "C++" {
        #[rust_name = "emit_property_name_changed"]
        fn emitPropertyNameChanged(self: Pin<&mut MyObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "invokableNameWrapper"]
        fn invokable_name_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>);
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
    use std::pin::Pin;

    type UniquePtr<T> = cxx::UniquePtr<T>;

    use super::MyTrait;

    impl Default for MyObject {
        fn default() -> Self {
            Self { property_name: 32 }
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

    pub struct MyObject {
        property_name: i32,
    }

    impl MyObject {
        pub fn property_name<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
            cpp.property_name()
        }
    }

    impl MyObjectQt {
        pub fn property_name(&self) -> &i32 {
            &self.rust().property_name
        }
    }

    impl MyObjectQt {
        pub unsafe fn property_name_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut i32 {
            &mut self.rust_mut().get_unchecked_mut().property_name
        }
    }

    impl MyObject {
        pub fn set_property_name(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
            cpp.set_property_name(value);
        }
    }

    impl MyObjectQt {
        pub fn set_property_name(mut self: Pin<&mut Self>, value: i32) {
            unsafe {
                self.as_mut().rust_mut().property_name = value;
            }
            self.as_mut().emit_property_name_changed();
        }
    }

    impl MyObject {
        pub fn invokable_name_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>) {
            cpp.invokable_name();
        }
    }

    impl MyObjectQt {
        pub fn invokable_name(self: Pin<&mut Self>) {
            println!("Bye from Rust!");
            self.as_mut().set_property_name(5);
        }
    }

    unsafe impl Send for MyObjectCxxQtThread {}

    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
}
