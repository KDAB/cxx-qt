#[cxx::bridge(namespace = "")]
mod inheritance {
    extern "C++" {
        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
    }
    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/convert.h");
        include!("cxx-qt-lib/cxxqt_thread.h");
        include!("cxx-qt-lib/std_types.h");
    }
    unsafe extern "C++" {
        include!("cxx-qt-gen/inheritance.cxxqt.h");
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
        #[cxx_name = "dataWrapper"]
        fn data_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            _index: &QModelIndex,
            _role: i32,
        ) -> QVariant;
    }
    extern "Rust" {
        #[cxx_name = "hasChildrenWrapper"]
        fn has_children_wrapper(self: &MyObject, cpp: &MyObjectQt, _parent: &QModelIndex) -> bool;
    }
    unsafe extern "C++" {
        #[cxx_name = "fetchMore_cxxqt_inherit"]
        fn fetch_more(self: Pin<&mut MyObjectQt>, index: &QModelIndex);
    }
    unsafe extern "C++" {
        #[cxx_name = "hasChildren_cxxqt_inherit"]
        fn has_children_super(self: &MyObjectQt, parent: &QModelIndex) -> bool;
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
        #[namespace = "cxx_qt_my_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }
    extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }
    extern "Rust" {
        #[namespace = "cxx_qt_my_object"]
        type MyObjectCxxQtThreadQueuedFn;
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt_my_object"]
        fn create_rs_my_object() -> Box<MyObject>;
    }
}
pub use self::cxx_qt_inheritance::*;
mod cxx_qt_inheritance {
    use super::inheritance::*;
    use std::pin::Pin;
    type UniquePtr<T> = cxx::UniquePtr<T>;
    #[derive(Default)]
    pub struct MyObject {
        data: Vec<i32>,
    }
    impl MyObjectQt {
        fn data(&self) -> &Vec<i32> {
            &self.rust().data
        }
    }
    impl MyObjectQt {
        fn data_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut Vec<i32> {
            unsafe { &mut self.rust_mut().get_unchecked_mut().data }
        }
    }
    impl MyObjectQt {
        fn set_data(mut self: Pin<&mut Self>, value: Vec<i32>) {
            unsafe {
                self.rust_mut().data = value;
            }
        }
    }
    impl MyObject {
        pub fn data_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            _index: &QModelIndex,
            _role: i32,
        ) -> QVariant {
            return cpp.data(_index, _role);
        }
    }
    impl MyObjectQt {
        pub fn data(&self, _index: &QModelIndex, _role: i32) -> QVariant {
            QVariant::default()
        }
    }
    impl MyObject {
        pub fn has_children_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            _parent: &QModelIndex,
        ) -> bool {
            return cpp.has_children(_parent);
        }
    }
    impl MyObjectQt {
        pub fn has_children(&self, _parent: &QModelIndex) -> bool {
            false
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
    pub mod qobject {
        pub type MyObject = super::MyObjectQt;
    }
}
