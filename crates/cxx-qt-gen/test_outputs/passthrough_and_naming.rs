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
        include!("cxx-qt-lib/std_types.h");
    }

    unsafe extern "C++" {
        include!("cxx-qt-gen/multi_object.cxxqt.h");
    }

    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObject"]
        #[doc = "\n"]
        #[doc = "Use type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
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
        #[doc = "Notify signal for the Q_PROPERTY"]
        #[doc = "property_name"]
        #[doc = "\n"]
        #[doc = "This can be used to manually notify a change when the unsafe mutable getter,"]
        #[doc = "property_name_mut"]
        #[doc = ", is used."]
        #[rust_name = "property_name_changed"]
        fn propertyNameChanged(self: Pin<&mut MyObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "invokableNameWrapper"]
        fn invokable_name_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>);
    }

    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "emit_ready"]
        fn emitReady(self: Pin<&mut MyObjectQt>);
    }

    unsafe extern "C++" {
        #[doc = r" Specialised version of CxxQtThread, which can be moved into other threads."]
        #[doc = r""]
        #[doc = r" CXX doesn't support having generic types in the function yet"]
        #[doc = r" so we cannot have CxxQtThread<T> in cxx-qt-lib and then use that here"]
        #[doc = r" For now we use a type alias in C++ then use it like a normal type here"]
        #[doc = r" https://github.com/dtolnay/cxx/issues/683"]
        type MyObjectCxxQtThread;

        #[doc = r" Retrieve an immutable reference to the Rust struct backing this C++ object"]
        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

        #[doc = r" Create an instance of a CxxQtThread"]
        #[doc = r""]
        #[doc = r" This allows for queueing closures onto the Qt event loop from a background thread."]
        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObjectQt) -> UniquePtr<MyObjectCxxQtThread>;

        #[doc(hidden)]
        #[cxx_name = "queue"]
        fn queue_boxed_fn(
            self: &MyObjectCxxQtThread,
            func: fn(Pin<&mut MyObjectQt>, Box<MyObjectCxxQtThreadQueuedFn>),
            arg: Box<MyObjectCxxQtThreadQueuedFn>,
        ) -> Result<()>;

        #[doc = "Generated CXX-Qt method which creates a new"]
        #[doc = "MyObjectQt"]
        #[doc = "as a UniquePtr with no parent in Qt"]
        #[rust_name = "new_cpp_object_my_object_qt"]
        #[namespace = "cxx_qt::multi_object::cxx_qt_my_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[doc = r" Retrieve a mutable reference to the Rust struct backing this C++ object"]
        #[doc = r""]
        #[doc = r" This method is unsafe as if a Q_PROPERTY is modified its changed signal must be triggered manually."]
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
        #[doc = "The C++ type for the QObject "]
        #[doc = "SecondObject"]
        #[doc = "\n"]
        #[doc = "Use type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
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
        #[doc = "Notify signal for the Q_PROPERTY"]
        #[doc = "property_name"]
        #[doc = "\n"]
        #[doc = "This can be used to manually notify a change when the unsafe mutable getter,"]
        #[doc = "property_name_mut"]
        #[doc = ", is used."]
        #[rust_name = "property_name_changed"]
        fn propertyNameChanged(self: Pin<&mut SecondObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "invokableNameWrapper"]
        fn invokable_name_wrapper(self: &mut SecondObject, cpp: Pin<&mut SecondObjectQt>);
    }

    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "emit_ready"]
        fn emitReady(self: Pin<&mut SecondObjectQt>);
    }

    unsafe extern "C++" {
        #[doc = r" Specialised version of CxxQtThread, which can be moved into other threads."]
        #[doc = r""]
        #[doc = r" CXX doesn't support having generic types in the function yet"]
        #[doc = r" so we cannot have CxxQtThread<T> in cxx-qt-lib and then use that here"]
        #[doc = r" For now we use a type alias in C++ then use it like a normal type here"]
        #[doc = r" https://github.com/dtolnay/cxx/issues/683"]
        type SecondObjectCxxQtThread;
        #[doc = r" Retrieve an immutable reference to the Rust struct backing this C++ object"]
        #[cxx_name = "unsafeRust"]
        fn rust(self: &SecondObjectQt) -> &SecondObject;
        #[doc = r" Create an instance of a CxxQtThread"]
        #[doc = r""]
        #[doc = r" This allows for queueing closures onto the Qt event loop from a background thread."]
        #[cxx_name = "qtThread"]
        fn qt_thread(self: &SecondObjectQt) -> UniquePtr<SecondObjectCxxQtThread>;
        #[doc(hidden)]
        #[cxx_name = "queue"]
        fn queue_boxed_fn(
            self: &SecondObjectCxxQtThread,
            func: fn(Pin<&mut SecondObjectQt>, Box<SecondObjectCxxQtThreadQueuedFn>),
            arg: Box<SecondObjectCxxQtThreadQueuedFn>,
        ) -> Result<()>;
        #[doc = "Generated CXX-Qt method which creates a new"]
        #[doc = "SecondObjectQt"]
        #[doc = "as a UniquePtr with no parent in Qt"]
        #[rust_name = "new_cpp_object_second_object_qt"]
        #[namespace = "cxx_qt::multi_object::cxx_qt_second_object"]
        fn newCppObject() -> UniquePtr<SecondObjectQt>;
    }

    extern "C++" {
        #[doc = r" Retrieve a mutable reference to the Rust struct backing this C++ object"]
        #[doc = r""]
        #[doc = r" This method is unsafe as if a Q_PROPERTY is modified its changed signal must be triggered manually."]
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

    #[doc(hidden)]
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
        #[doc(hidden)]
        pub fn property_name<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
            cpp.property_name()
        }
    }

    impl MyObjectQt {
        #[doc = "Getter for the Q_PROPERTY "]
        #[doc = "property_name"]
        pub fn property_name(&self) -> &i32 {
            &self.rust().property_name
        }
    }

    impl MyObjectQt {
        #[doc = "unsafe getter for the Q_PROPERTY "]
        #[doc = "property_name"]
        #[doc = "\n"]
        #[doc = "This allows for modifying the Q_PROPERTY without calling the property changed Q_SIGNAL"]
        #[doc = "\n"]
        #[doc = "After modifying the property, make sure to call the corresponding changed signal: "]
        #[doc = "property_name_changed"]
        pub unsafe fn property_name_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut i32 {
            &mut self.rust_mut().get_unchecked_mut().property_name
        }
    }

    impl MyObject {
        #[doc(hidden)]
        pub fn set_property_name(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
            cpp.set_property_name(value);
        }
    }

    impl MyObjectQt {
        #[doc = "Setter for the Q_PROPERTY "]
        #[doc = "property_name"]
        pub fn set_property_name(mut self: Pin<&mut Self>, value: i32) {
            if self.rust().property_name == value {
                return;
            }
            unsafe {
                self.as_mut().rust_mut().property_name = value;
            }
            self.as_mut().property_name_changed();
        }
    }

    impl MyObject {
        #[doc(hidden)]
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

    impl MyObjectQt {
        pub const MY_CONSTANT: i32 = 42;
    }

    impl MyObjectQt {
        type MyType = i32;
    }

    impl MyObjectQt {
        my_macro!();
    }

    pub enum MySignals {
        Ready,
    }

    impl MyObjectQt {
        #[doc = "Emit the signal from the enum "]
        #[doc = "MySignals"]
        #[doc = " on the QObject "]
        #[doc = "MyObject"]
        pub fn emit(self: Pin<&mut Self>, signal: MySignals) {
            match signal {
                MySignals::Ready {} => self.emit_ready(),
            }
        }
    }

    unsafe impl Send for MyObjectCxxQtThread {}

    impl MyObjectCxxQtThread {
        #[doc = r" Queue the given closure onto the Qt event loop for this QObject"]
        pub fn queue<F>(&self, f: F) -> std::result::Result<(), cxx::Exception>
        where
            F: FnOnce(std::pin::Pin<&mut MyObjectQt>),
            F: Send + 'static,
        {
            #[allow(clippy::boxed_local)]
            #[doc(hidden)]
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

    #[doc(hidden)]
    pub struct MyObjectCxxQtThreadQueuedFn {
        inner: std::boxed::Box<dyn FnOnce(std::pin::Pin<&mut MyObjectQt>) + Send>,
    }

    #[doc = r" Generated CXX-Qt method which creates a boxed rust struct of a QObject"]
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
        #[doc(hidden)]
        pub fn property_name<'a>(&'a self, cpp: &'a SecondObjectQt) -> &'a i32 {
            cpp.property_name()
        }
    }

    impl SecondObjectQt {
        #[doc = "Getter for the Q_PROPERTY "]
        #[doc = "property_name"]
        pub fn property_name(&self) -> &i32 {
            &self.rust().property_name
        }
    }
    impl SecondObjectQt {
        #[doc = "unsafe getter for the Q_PROPERTY "]
        #[doc = "property_name"]
        #[doc = "\n"]
        #[doc = "This allows for modifying the Q_PROPERTY without calling the property changed Q_SIGNAL"]
        #[doc = "\n"]
        #[doc = "After modifying the property, make sure to call the corresponding changed signal: "]
        #[doc = "property_name_changed"]
        pub unsafe fn property_name_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut i32 {
            &mut self.rust_mut().get_unchecked_mut().property_name
        }
    }

    impl SecondObject {
        #[doc(hidden)]
        pub fn set_property_name(&mut self, cpp: Pin<&mut SecondObjectQt>, value: i32) {
            cpp.set_property_name(value);
        }
    }

    impl SecondObjectQt {
        #[doc = "Setter for the Q_PROPERTY "]
        #[doc = "property_name"]
        pub fn set_property_name(mut self: Pin<&mut Self>, value: i32) {
            if self.rust().property_name == value {
                return;
            }
            unsafe {
                self.as_mut().rust_mut().property_name = value;
            }
            self.as_mut().property_name_changed();
        }
    }

    impl SecondObject {
        #[doc(hidden)]
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
        #[doc = "Emit the signal from the enum "]
        #[doc = "SecondSignals"]
        #[doc = " on the QObject "]
        #[doc = "SecondObject"]
        pub fn emit(self: Pin<&mut Self>, signal: SecondSignals) {
            match signal {
                SecondSignals::Ready {} => self.emit_ready(),
            }
        }
    }

    unsafe impl Send for SecondObjectCxxQtThread {}

    impl SecondObjectCxxQtThread {
        #[doc = r" Queue the given closure onto the Qt event loop for this QObject"]
        pub fn queue<F>(&self, f: F) -> std::result::Result<(), cxx::Exception>
        where
            F: FnOnce(std::pin::Pin<&mut SecondObjectQt>),
            F: Send + 'static,
        {
            #[allow(clippy::boxed_local)]
            #[doc(hidden)]
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

    #[doc(hidden)]
    pub struct SecondObjectCxxQtThreadQueuedFn {
        inner: std::boxed::Box<dyn FnOnce(std::pin::Pin<&mut SecondObjectQt>) + Send>,
    }

    #[doc = r" Generated CXX-Qt method which creates a boxed rust struct of a QObject"]
    pub fn create_rs_second_object() -> std::boxed::Box<SecondObject> {
        std::default::Default::default()
    }

    #[doc = r" Generated CXX-Qt module containing type alias to the C++ type of the QObjects"]
    pub mod qobject {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObject"]
        #[doc = "\n"]
        #[doc = "Use type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        pub type MyObject = super::MyObjectQt;
        #[doc = "The C++ type for the QObject "]
        #[doc = "SecondObject"]
        #[doc = "\n"]
        #[doc = "Use type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        pub type SecondObject = super::SecondObjectQt;
    }
}
