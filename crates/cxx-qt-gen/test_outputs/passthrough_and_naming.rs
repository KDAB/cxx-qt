#[cxx::bridge(namespace = "cxx_qt::multi_object")]
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
        include!("cxx-qt-lib/convert.h");
        include!("cxx-qt-lib/cxxqt_thread.h");
    }

    unsafe extern "C++" {
        include!("cxx-qt-gen/multi_object.cxxqt.h");
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
        #[rust_name = "property_name_changed"]
        fn propertyNameChanged(self: Pin<&mut MyObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "invokableNameWrapper"]
        fn invokable_name_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>);
    }

    unsafe extern "C++" {
        #[rust_name = "emit_ready"]
        fn emitReady(self: Pin<&mut MyObjectQt>);
    }

    unsafe extern "C++" {
        type MyObjectCxxQtThread;

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObjectQt) -> UniquePtr<MyObjectCxxQtThread>;

        #[cxx_name = "queue"]
        fn queue_boxed_fn(
            self: &MyObjectCxxQtThread,
            func: fn(Pin<&mut MyObjectQt>, Box<MyObjectCxxQtThreadQueuedFn>),
            arg: Box<MyObjectCxxQtThreadQueuedFn>,
        ) -> Result<()>;

        #[rust_name = "new_cpp_object_my_object_qt"]
        #[namespace = "cxx_qt::multi_object::cxx_qt_my_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }

    extern "Rust" {
        #[namespace = "cxx_qt::multi_object::cxx_qt_my_object"]
        type MyObjectCxxQtThreadQueuedFn;

        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::multi_object::cxx_qt_my_object"]
        fn create_rs_my_object() -> Box<MyObject>;
    }

    unsafe extern "C++" {
        #[cxx_name = "SecondObject"]
        type SecondObjectQt;
    }

    extern "Rust" {
        #[cxx_name = "SecondObjectRust"]
        type SecondObject;
    }

    extern "Rust" {
        #[cxx_name = "getPropertyName"]
        unsafe fn property_name<'a>(self: &'a SecondObject, cpp: &'a SecondObjectQt) -> &'a i32;
    }

    extern "Rust" {
        #[cxx_name = "setPropertyName"]
        fn set_property_name(self: &mut SecondObject, cpp: Pin<&mut SecondObjectQt>, value: i32);
    }

    unsafe extern "C++" {
        #[rust_name = "property_name_changed"]
        fn propertyNameChanged(self: Pin<&mut SecondObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "invokableNameWrapper"]
        fn invokable_name_wrapper(self: &mut SecondObject, cpp: Pin<&mut SecondObjectQt>);
    }

    unsafe extern "C++" {
        #[rust_name = "emit_ready"]
        fn emitReady(self: Pin<&mut SecondObjectQt>);
    }

    unsafe extern "C++" {
        type SecondObjectCxxQtThread;
        #[cxx_name = "unsafeRust"]
        fn rust(self: &SecondObjectQt) -> &SecondObject;
        #[cxx_name = "qtThread"]
        fn qt_thread(self: &SecondObjectQt) -> UniquePtr<SecondObjectCxxQtThread>;
        #[cxx_name = "queue"]
        fn queue_boxed_fn(
            self: &SecondObjectCxxQtThread,
            func: fn(Pin<&mut SecondObjectQt>, Box<SecondObjectCxxQtThreadQueuedFn>),
            arg: Box<SecondObjectCxxQtThreadQueuedFn>,
        ) -> Result<()>;
        #[rust_name = "new_cpp_object_second_object_qt"]
        #[namespace = "cxx_qt::multi_object::cxx_qt_second_object"]
        fn newCppObject() -> UniquePtr<SecondObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        unsafe fn rust_mut(self: Pin<&mut SecondObjectQt>) -> Pin<&mut SecondObject>;
    }

    extern "Rust" {
        #[namespace = "cxx_qt::multi_object::cxx_qt_second_object"]
        type SecondObjectCxxQtThreadQueuedFn;
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::multi_object::cxx_qt_second_object"]
        fn create_rs_second_object() -> Box<SecondObject>;
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
            self.as_mut().property_name_changed();
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

    pub enum MySignals {
        Ready,
    }

    impl MyObjectQt {
        pub fn emit(self: Pin<&mut Self>, signal: MySignals) {
            match signal {
                MySignals::Ready {} => self.emit_ready(),
            }
        }
    }

    unsafe impl Send for MyObjectCxxQtThread {}

    impl MyObjectCxxQtThread {
        pub fn queue<F>(&self, f: F) -> std::result::Result<(), cxx::Exception>
        where
            F: FnOnce(std::pin::Pin<&mut MyObjectQt>),
            F: Send + 'static,
        {
            #[allow(clippy::boxed_local)]
            fn func(
                obj: std::pin::Pin<&mut MyObjectQt>,
                arg: std::boxed::Box<MyObjectCxxQtThreadQueuedFn>,
            ) {
                (arg.inner)(obj)
            }
            let arg = MyObjectCxxQtThreadQueuedFn {
                inner: std::boxed::Box::new(f),
            };
            self.queue_boxed_fn(func, std::boxed::Box::new(arg))
        }
    }

    pub struct MyObjectCxxQtThreadQueuedFn {
        inner: std::boxed::Box<dyn FnOnce(std::pin::Pin<&mut MyObjectQt>) + Send>,
    }

    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }

    impl Default for SecondObject {
        fn default() -> Self {
            Self { property_name: 32 }
        }
    }

    pub struct SecondObject {
        property_name: i32,
    }

    impl SecondObject {
        pub fn property_name<'a>(&'a self, cpp: &'a SecondObjectQt) -> &'a i32 {
            cpp.property_name()
        }
    }

    impl SecondObjectQt {
        pub fn property_name(&self) -> &i32 {
            &self.rust().property_name
        }
    }
    impl SecondObjectQt {
        pub unsafe fn property_name_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut i32 {
            &mut self.rust_mut().get_unchecked_mut().property_name
        }
    }

    impl SecondObject {
        pub fn set_property_name(&mut self, cpp: Pin<&mut SecondObjectQt>, value: i32) {
            cpp.set_property_name(value);
        }
    }

    impl SecondObjectQt {
        pub fn set_property_name(mut self: Pin<&mut Self>, value: i32) {
            unsafe {
                self.as_mut().rust_mut().property_name = value;
            }
            self.as_mut().property_name_changed();
        }
    }

    impl SecondObject {
        pub fn invokable_name_wrapper(self: &mut SecondObject, cpp: Pin<&mut SecondObjectQt>) {
            cpp.invokable_name();
        }
    }

    impl SecondObjectQt {
        pub fn invokable_name(self: Pin<&mut Self>) {
            println!("Bye from second Rust!");
            self.as_mut().set_property_name(5);
        }
    }

    pub enum SecondSignals {
        Ready,
    }

    impl SecondObjectQt {
        pub fn emit(self: Pin<&mut Self>, signal: SecondSignals) {
            match signal {
                SecondSignals::Ready {} => self.emit_ready(),
            }
        }
    }

    unsafe impl Send for SecondObjectCxxQtThread {}

    impl SecondObjectCxxQtThread {
        pub fn queue<F>(&self, f: F) -> std::result::Result<(), cxx::Exception>
        where
            F: FnOnce(std::pin::Pin<&mut SecondObjectQt>),
            F: Send + 'static,
        {
            #[allow(clippy::boxed_local)]
            fn func(
                obj: std::pin::Pin<&mut SecondObjectQt>,
                arg: std::boxed::Box<SecondObjectCxxQtThreadQueuedFn>,
            ) {
                (arg.inner)(obj)
            }
            let arg = SecondObjectCxxQtThreadQueuedFn {
                inner: std::boxed::Box::new(f),
            };
            self.queue_boxed_fn(func, std::boxed::Box::new(arg))
        }
    }

    pub struct SecondObjectCxxQtThreadQueuedFn {
        inner: std::boxed::Box<dyn FnOnce(std::pin::Pin<&mut SecondObjectQt>) + Send>,
    }

    pub fn create_rs_second_object() -> std::boxed::Box<SecondObject> {
        std::default::Default::default()
    }

    pub mod qobject {
        pub type MyObject = super::MyObjectQt;
        pub type SecondObject = super::SecondObjectQt;
    }
}
