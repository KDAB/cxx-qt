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
        #[doc = "CXX-Qt generated method which calls the C++ method"]
        #[doc = "hasChildren"]
        #[doc = "on the base class"]
        #[cxx_name = "hasChildrenCxxQtInherit"]
        fn has_children_super(self: &MyObjectQt, parent: &QModelIndex) -> bool;
    }
    extern "C++" {
        #[doc = "CXX-Qt generated method which calls the C++ method"]
        #[doc = "fetchMore"]
        #[doc = "on the base class"]
        #[cxx_name = "fetchMoreCxxQtInherit"]
        unsafe fn fetch_more(self: Pin<&mut MyObjectQt>, index: &QModelIndex);
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
        #[namespace = "cxx_qt_my_object"]
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
        #[namespace = "cxx_qt_my_object"]
        type MyObjectCxxQtThreadQueuedFn;
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt_my_object"]
        fn create_rs_my_object() -> Box<MyObject>;
    }
}
use self::cxx_qt_inheritance::*;
mod cxx_qt_inheritance {
    use super::inheritance::*;
    use std::pin::Pin;
    #[doc(hidden)]
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
        #[doc(hidden)]
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
        #[doc(hidden)]
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
    #[doc = r" Generated CXX-Qt module containing type alias to the C++ type of the QObjects"]
    pub mod qobject {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObject"]
        #[doc = "\n"]
        #[doc = "Use type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        pub type MyObject = super::MyObjectQt;
    }
}
