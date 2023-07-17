#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }
    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/qt.h");
        #[doc(hidden)]
        #[namespace = "Qt"]
        #[rust_name = "CxxQtConnectionType"]
        type ConnectionType = cxx_qt_lib::ConnectionType;
        include!("cxx-qt-lib/qmetaobjectconnection.h");
        #[doc(hidden)]
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "CxxQtQMetaObjectConnection"]
        type QMetaObjectConnection = cxx_qt_lib::QMetaObjectConnection;
    }
    unsafe extern "C++" {
        include!("cxx-qt-gen/ffi.cxxqt.h");
    }
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObjectRust"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        type MyObject;
    }
    extern "Rust" {
        type MyObjectRust;
    }
    extern "Rust" {
        #[cxx_name = "invokableWrapper"]
        fn invokable_wrapper(self: &MyObjectRust, cpp: &MyObject);
    }
    extern "Rust" {
        #[cxx_name = "invokableMutableWrapper"]
        fn invokable_mutable_wrapper(self: &mut MyObjectRust, cpp: Pin<&mut MyObject>);
    }
    extern "Rust" {
        #[cxx_name = "invokableParametersWrapper"]
        fn invokable_parameters_wrapper(
            self: &MyObjectRust,
            cpp: &MyObject,
            opaque: &QColor,
            trivial: &QPoint,
            primitive: i32,
        );
    }
    extern "Rust" {
        #[cxx_name = "invokableReturnOpaqueWrapper"]
        fn invokable_return_opaque_wrapper(
            self: &mut MyObjectRust,
            cpp: Pin<&mut MyObject>,
        ) -> UniquePtr<Opaque>;
    }
    extern "Rust" {
        #[cxx_name = "invokableReturnTrivialWrapper"]
        fn invokable_return_trivial_wrapper(
            self: &mut MyObjectRust,
            cpp: Pin<&mut MyObject>,
        ) -> QPoint;
    }
    extern "Rust" {
        #[cxx_name = "invokableFinalWrapper"]
        fn invokable_final_wrapper(self: &MyObjectRust, cpp: &MyObject);
    }
    extern "Rust" {
        #[cxx_name = "invokableOverrideWrapper"]
        fn invokable_override_wrapper(self: &MyObjectRust, cpp: &MyObject);
    }
    extern "Rust" {
        #[cxx_name = "invokableVirtualWrapper"]
        fn invokable_virtual_wrapper(self: &MyObjectRust, cpp: &MyObject);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        type MyObjectCxxQtThread = cxx_qt::CxxQtThread<MyObject>;
        include!("cxx-qt-common/cxxqt_thread.h");
        #[doc(hidden)]
        #[cxx_name = "qtThread"]
        fn cxx_qt_ffi_qt_thread(self: &MyObject) -> MyObjectCxxQtThread;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtlib1"]
        #[cxx_name = "cxxQtThreadQueue"]
        fn cxx_qt_ffi_my_object_queue_boxed_fn(
            cxx_qt_thread: &MyObjectCxxQtThread,
            func: fn(Pin<&mut MyObject>, Box<MyObjectCxxQtThreadQueuedFn>),
            arg: Box<MyObjectCxxQtThreadQueuedFn>,
        ) -> Result<()>;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtlib1"]
        #[cxx_name = "cxxQtThreadClone"]
        fn cxx_qt_ffi_my_object_threading_clone(
            cxx_qt_thread: &MyObjectCxxQtThread,
        ) -> MyObjectCxxQtThread;
        #[doc(hidden)]
        #[namespace = "rust::cxxqtlib1"]
        #[cxx_name = "cxxQtThreadDrop"]
        fn cxx_qt_ffi_my_object_threading_drop(cxx_qt_thread: &mut MyObjectCxxQtThread);
    }
    extern "Rust" {
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        type MyObjectCxxQtThreadQueuedFn;
    }
    #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
    #[cxx_name = "CxxQtConstructorArguments0"]
    #[doc(hidden)]
    struct CxxQtConstructorArgumentsMyObjectRust0 {
        baseArguments: CxxQtConstructorBaseArgumentsMyObjectRust0,
        newArguments: CxxQtConstructorNewArgumentsMyObjectRust0,
        initializeArguments: CxxQtConstructorInitializeArgumentsMyObjectRust0,
    }
    #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
    #[cxx_name = "CxxQtConstructorBaseArguments0"]
    #[doc(hidden)]
    struct CxxQtConstructorBaseArgumentsMyObjectRust0 {
        arg0: *mut QObject,
    }
    #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
    #[cxx_name = "CxxQtConstructorNewArguments0"]
    #[doc(hidden)]
    struct CxxQtConstructorNewArgumentsMyObjectRust0 {
        arg0: i32,
    }
    #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
    #[cxx_name = "CxxQtConstructorInitializeArguments0"]
    #[doc(hidden)]
    struct CxxQtConstructorInitializeArgumentsMyObjectRust0 {
        not_empty: i8,
    }
    extern "Rust" {
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        #[cxx_name = "routeArguments0"]
        unsafe fn route_arguments_my_object_rust_0(
            arg0: i32,
            arg1: *mut QObject,
        ) -> CxxQtConstructorArgumentsMyObjectRust0;
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        #[cxx_name = "newRs0"]
        fn new_rs_my_object_rust_0(
            args: CxxQtConstructorNewArgumentsMyObjectRust0,
        ) -> Box<MyObjectRust>;
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        #[cxx_name = "initialize0"]
        fn initialize_my_object_rust_0(
            qobject: Pin<&mut MyObject>,
            args: CxxQtConstructorInitializeArgumentsMyObjectRust0,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRust"]
        #[doc(hidden)]
        fn cxx_qt_ffi_rust(self: &MyObject) -> &MyObjectRust;
    }
    unsafe extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        #[doc(hidden)]
        fn cxx_qt_ffi_rust_mut(self: Pin<&mut MyObject>) -> Pin<&mut MyObjectRust>;
    }
}
use self::cxx_qt_ffi::*;
#[doc = r" Internal CXX-Qt module, made public temporarily between API changes"]
pub mod cxx_qt_ffi {
    use super::ffi::*;
    use cxx_qt::CxxQtType;
    use std::pin::Pin;
    #[doc(hidden)]
    type UniquePtr<T> = cxx::UniquePtr<T>;
    type MyObjectRust = super::MyObjectRust;
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn invokable_wrapper(self: &MyObjectRust, cpp: &MyObject) {
            cpp.invokable();
        }
    }
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn invokable_mutable_wrapper(self: &mut MyObjectRust, cpp: Pin<&mut MyObject>) {
            cpp.invokable_mutable();
        }
    }
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn invokable_parameters_wrapper(
            self: &MyObjectRust,
            cpp: &MyObject,
            opaque: &QColor,
            trivial: &QPoint,
            primitive: i32,
        ) {
            cpp.invokable_parameters(opaque, trivial, primitive);
        }
    }
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn invokable_return_opaque_wrapper(
            self: &mut MyObjectRust,
            cpp: Pin<&mut MyObject>,
        ) -> UniquePtr<Opaque> {
            return cpp.invokable_return_opaque();
        }
    }
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn invokable_return_trivial_wrapper(
            self: &mut MyObjectRust,
            cpp: Pin<&mut MyObject>,
        ) -> QPoint {
            return cpp.invokable_return_trivial();
        }
    }
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn invokable_final_wrapper(self: &MyObjectRust, cpp: &MyObject) {
            cpp.invokable_final();
        }
    }
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn invokable_override_wrapper(self: &MyObjectRust, cpp: &MyObject) {
            cpp.invokable_override();
        }
    }
    impl MyObjectRust {
        #[doc(hidden)]
        pub fn invokable_virtual_wrapper(self: &MyObjectRust, cpp: &MyObject) {
            cpp.invokable_virtual();
        }
    }
    impl cxx_qt::Threading for MyObject {
        type BoxedQueuedFn = MyObjectCxxQtThreadQueuedFn;
        type ThreadingTypeId = cxx::type_id!("cxx_qt::my_object::MyObjectCxxQtThread");
        fn qt_thread(&self) -> MyObjectCxxQtThread {
            self.cxx_qt_ffi_qt_thread()
        }
        #[doc(hidden)]
        fn queue<F>(
            cxx_qt_thread: &MyObjectCxxQtThread,
            f: F,
        ) -> std::result::Result<(), cxx::Exception>
        where
            F: FnOnce(std::pin::Pin<&mut MyObject>),
            F: Send + 'static,
        {
            #[allow(clippy::boxed_local)]
            #[doc(hidden)]
            fn func(
                obj: std::pin::Pin<&mut MyObject>,
                arg: std::boxed::Box<MyObjectCxxQtThreadQueuedFn>,
            ) {
                (arg.inner)(obj)
            }
            let arg = MyObjectCxxQtThreadQueuedFn {
                inner: std::boxed::Box::new(f),
            };
            cxx_qt_ffi_my_object_queue_boxed_fn(cxx_qt_thread, func, std::boxed::Box::new(arg))
        }
        #[doc(hidden)]
        fn threading_clone(cxx_qt_thread: &MyObjectCxxQtThread) -> MyObjectCxxQtThread {
            cxx_qt_ffi_my_object_threading_clone(cxx_qt_thread)
        }
        #[doc(hidden)]
        fn threading_drop(cxx_qt_thread: &mut MyObjectCxxQtThread) {
            cxx_qt_ffi_my_object_threading_drop(cxx_qt_thread);
        }
    }
    #[doc(hidden)]
    pub struct MyObjectCxxQtThreadQueuedFn {
        inner: std::boxed::Box<dyn FnOnce(std::pin::Pin<&mut MyObject>) + Send>,
    }
    impl cxx_qt::Locking for MyObject {}
    #[doc(hidden)]
    pub fn route_arguments_my_object_rust_0(
        arg0: i32,
        arg1: *mut QObject,
    ) -> CxxQtConstructorArgumentsMyObjectRust0 {
        #[allow(unused_variables)]
        #[allow(clippy::let_unit_value)]
        let (new_arguments, base_arguments, initialize_arguments) =
            <MyObject as cxx_qt::Constructor<(i32, *mut QObject)>>::route_arguments((arg0, arg1));
        CxxQtConstructorArgumentsMyObjectRust0 {
            baseArguments: CxxQtConstructorBaseArgumentsMyObjectRust0 {
                arg0: base_arguments.0,
            },
            initializeArguments: CxxQtConstructorInitializeArgumentsMyObjectRust0 { not_empty: 0 },
            newArguments: CxxQtConstructorNewArgumentsMyObjectRust0 {
                arg0: new_arguments.0,
            },
        }
    }
    #[doc(hidden)]
    #[allow(unused_variables)]
    pub fn new_rs_my_object_rust_0(
        new_arguments: CxxQtConstructorNewArgumentsMyObjectRust0,
    ) -> std::boxed::Box<MyObjectRust> {
        #[allow(clippy::let_unit_value)]
        let new_arguments = (new_arguments.arg0,);
        std::boxed::Box::new(<MyObject as cxx_qt::Constructor<(i32, *mut QObject)>>::new(
            new_arguments,
        ))
    }
    #[doc(hidden)]
    #[allow(unused_variables)]
    pub fn initialize_my_object_rust_0(
        qobject: core::pin::Pin<&mut MyObject>,
        initialize_arguments: CxxQtConstructorInitializeArgumentsMyObjectRust0,
    ) {
        #[allow(clippy::let_unit_value)]
        let initialize_arguments = ();
        <MyObject as cxx_qt::Constructor<(i32, *mut QObject)>>::initialize(
            qobject,
            initialize_arguments,
        )
    }
    impl cxx_qt::CxxQtType for MyObject {
        type Rust = MyObjectRust;
        fn rust(&self) -> &Self::Rust {
            self.cxx_qt_ffi_rust()
        }
        fn rust_mut(self: core::pin::Pin<&mut Self>) -> Pin<&mut Self::Rust> {
            self.cxx_qt_ffi_rust_mut()
        }
    }
    #[doc = r" Generated CXX-Qt module containing type alias to the C++ types of the QObjects"]
    pub mod qobject {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObject"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        pub type MyObject = super::MyObject;
    }
}
