#[cxx::bridge(namespace = "cxx_qt::my_object")]
#[allow(unused_unsafe)]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include ! (< QtCore / QObject >);
        type QObject;
        type Opaque;
    }
    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt/connection.h");
        #[doc(hidden)]
        #[namespace = "Qt"]
        #[rust_name = "CxxQtConnectionType"]
        #[allow(dead_code)]
        type ConnectionType = cxx_qt::ConnectionType;
        #[doc(hidden)]
        #[namespace = "rust::cxxqt1"]
        #[rust_name = "CxxQtQMetaObjectConnection"]
        #[allow(dead_code)]
        type QMetaObjectConnection = cxx_qt::QMetaObjectConnection;
    }
    unsafe extern "C++" {
        include!("directory/file_ident.cxxqt.h");
    }
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObjectRust"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        #[namespace = "cxx_qt::my_object"]
        type MyObject;
    }
    extern "Rust" {
        #[namespace = "cxx_qt::my_object"]
        type MyObjectRust;
    }
    extern "Rust" {
        #[cxx_name = "cpp_method"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn cpp_method(self: &MyObject);
    }
    extern "Rust" {
        #[cxx_name = "invokable"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn invokable(self: &MyObject);
    }
    extern "Rust" {
        #[cxx_name = "invokable_mutable"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn invokable_mutable(self: Pin<&mut MyObject>);
    }
    extern "Rust" {
        #[cxx_name = "invokable_parameters"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn invokable_parameters(self: &MyObject, opaque: &QColor, trivial: &QPoint, primitive: i32);
    }
    extern "Rust" {
        #[cxx_name = "invokable_return_opaque"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn invokable_return_opaque(self: Pin<&mut MyObject>) -> UniquePtr<Opaque>;
    }
    extern "Rust" {
        #[cxx_name = "invokable_return_trivial"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn invokable_return_trivial(self: Pin<&mut MyObject>) -> QPoint;
    }
    extern "Rust" {
        #[cxx_name = "invokable_final"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn invokable_final(self: &MyObject);
    }
    extern "Rust" {
        #[cxx_name = "invokable_override"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn invokable_override(self: &MyObject);
    }
    extern "Rust" {
        #[cxx_name = "invokable_virtual"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn invokable_virtual(self: &MyObject);
    }
    unsafe extern "C++" {
        #[cxx_name = "invokable_pure_virtual"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn invokable_pure_virtual(self: &MyObject);
    }
    extern "Rust" {
        #[cxx_name = "invokable_result_tuple"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn invokable_result_tuple(self: &MyObject) -> Result<()>;
    }
    extern "Rust" {
        #[cxx_name = "invokable_result_type"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn invokable_result_type(self: &MyObject) -> Result<String>;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object"]
        type MyObjectCxxQtThread = cxx_qt::CxxQtThread<MyObject>;
        include!("cxx-qt/thread.h");
        #[doc(hidden)]
        #[cxx_name = "qtThread"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_MyObject_qtThread(qobject: &MyObject) -> MyObjectCxxQtThread;
        #[doc(hidden)]
        #[cxx_name = "cxxQtThreadQueue"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_MyObject_cxxQtThreadQueue(
            cxx_qt_thread: &MyObjectCxxQtThread,
            func: fn(Pin<&mut MyObject>, Box<MyObjectCxxQtThreadQueuedFn>),
            arg: Box<MyObjectCxxQtThreadQueuedFn>,
        ) -> u8;
        #[doc(hidden)]
        #[cxx_name = "cxxQtThreadClone"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_MyObject_cxxQtThreadClone(
            cxx_qt_thread: &MyObjectCxxQtThread,
        ) -> MyObjectCxxQtThread;
        #[doc(hidden)]
        #[cxx_name = "cxxQtThreadDrop"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_MyObject_cxxQtThreadDrop(cxx_qt_thread: &mut MyObjectCxxQtThread);
        #[doc(hidden)]
        #[cxx_name = "cxxQtThreadIsDestroyed"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_MyObject_cxxQtThreadIsDestroyed(cxx_qt_thread: &MyObjectCxxQtThread) -> bool;
    }
    extern "Rust" {
        #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
        type MyObjectCxxQtThreadQueuedFn;
    }
    #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
    #[cxx_name = "CxxQtConstructorArguments0"]
    #[doc(hidden)]
    struct CxxQtConstructorArgumentsMyObject0<'a> {
        base: CxxQtConstructorBaseArgumentsMyObject0,
        #[cxx_name = "new_"]
        new: CxxQtConstructorNewArgumentsMyObject0<'a>,
        initialize: CxxQtConstructorInitializeArgumentsMyObject0,
    }
    #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
    #[cxx_name = "CxxQtConstructorBaseArguments0"]
    #[doc(hidden)]
    struct CxxQtConstructorBaseArgumentsMyObject0 {
        arg0: *mut QObject,
    }
    #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
    #[cxx_name = "CxxQtConstructorNewArguments0"]
    #[doc(hidden)]
    struct CxxQtConstructorNewArgumentsMyObject0<'a> {
        arg0: &'a QString,
    }
    #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
    #[cxx_name = "CxxQtConstructorInitializeArguments0"]
    #[doc(hidden)]
    struct CxxQtConstructorInitializeArgumentsMyObject0 {
        not_empty: i8,
    }
    #[allow(clippy::needless_lifetimes)]
    extern "Rust" {
        #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
        #[cxx_name = "routeArguments0"]
        unsafe fn route_arguments_MyObject_0<'a>(
            arg0: i32,
            arg1: &'a QString,
        ) -> CxxQtConstructorArgumentsMyObject0<'a>;
        #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
        #[cxx_name = "newRs0"]
        unsafe fn new_rs_MyObject_0<'a>(
            args: CxxQtConstructorNewArgumentsMyObject0<'a>,
        ) -> Box<MyObjectRust>;
        #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
        #[cxx_name = "initialize0"]
        unsafe fn initialize_MyObject_0(
            qobject: Pin<&mut MyObject>,
            args: CxxQtConstructorInitializeArgumentsMyObject0,
        );
    }
    #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
    #[cxx_name = "CxxQtConstructorArguments1"]
    #[doc(hidden)]
    struct CxxQtConstructorArgumentsMyObject1 {
        base: CxxQtConstructorBaseArgumentsMyObject1,
        #[cxx_name = "new_"]
        new: CxxQtConstructorNewArgumentsMyObject1,
        initialize: CxxQtConstructorInitializeArgumentsMyObject1,
    }
    #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
    #[cxx_name = "CxxQtConstructorBaseArguments1"]
    #[doc(hidden)]
    struct CxxQtConstructorBaseArgumentsMyObject1 {
        not_empty: i8,
    }
    #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
    #[cxx_name = "CxxQtConstructorNewArguments1"]
    #[doc(hidden)]
    struct CxxQtConstructorNewArgumentsMyObject1 {
        not_empty: i8,
    }
    #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
    #[cxx_name = "CxxQtConstructorInitializeArguments1"]
    #[doc(hidden)]
    struct CxxQtConstructorInitializeArgumentsMyObject1 {
        not_empty: i8,
    }
    #[allow(clippy::needless_lifetimes)]
    extern "Rust" {
        #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
        #[cxx_name = "routeArguments1"]
        fn route_arguments_MyObject_1() -> CxxQtConstructorArgumentsMyObject1;
        #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
        #[cxx_name = "newRs1"]
        fn new_rs_MyObject_1(args: CxxQtConstructorNewArgumentsMyObject1) -> Box<MyObjectRust>;
        #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
        #[cxx_name = "initialize1"]
        fn initialize_MyObject_1(
            qobject: Pin<&mut MyObject>,
            args: CxxQtConstructorInitializeArgumentsMyObject1,
        );
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRust"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_MyObject_unsafeRust(outer: &MyObject) -> &MyObjectRust;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRustMut"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_MyObject_unsafeRustMut(outer: Pin<&mut MyObject>) -> Pin<&mut MyObjectRust>;
    }
}
impl cxx_qt::Threading for ffi::MyObject {
    type BoxedQueuedFn = MyObjectCxxQtThreadQueuedFn;
    type ThreadingTypeId = cxx::type_id!("cxx_qt::my_object::MyObjectCxxQtThread");
    fn qt_thread(&self) -> ffi::MyObjectCxxQtThread {
        ffi::cxx_qt_ffi_MyObject_qtThread(self)
    }
    #[doc(hidden)]
    fn is_destroyed(cxx_qt_thread: &ffi::MyObjectCxxQtThread) -> bool {
        ffi::cxx_qt_ffi_MyObject_cxxQtThreadIsDestroyed(cxx_qt_thread)
    }
    #[doc(hidden)]
    fn queue<F>(
        cxx_qt_thread: &ffi::MyObjectCxxQtThread,
        f: F,
    ) -> std::result::Result<(), cxx_qt::ThreadingQueueError>
    where
        F: FnOnce(core::pin::Pin<&mut ffi::MyObject>),
        F: Send + 'static,
    {
        #[allow(clippy::boxed_local)]
        #[doc(hidden)]
        fn func(
            obj: core::pin::Pin<&mut ffi::MyObject>,
            arg: std::boxed::Box<MyObjectCxxQtThreadQueuedFn>,
        ) {
            (arg.inner)(obj)
        }
        let arg = MyObjectCxxQtThreadQueuedFn {
            inner: std::boxed::Box::new(f),
        };
        match ffi::cxx_qt_ffi_MyObject_cxxQtThreadQueue(
            cxx_qt_thread,
            func,
            std::boxed::Box::new(arg),
        ) {
            0 => Ok(()),
            others => Err(others.into()),
        }
    }
    #[doc(hidden)]
    fn threading_clone(cxx_qt_thread: &ffi::MyObjectCxxQtThread) -> ffi::MyObjectCxxQtThread {
        ffi::cxx_qt_ffi_MyObject_cxxQtThreadClone(cxx_qt_thread)
    }
    #[doc(hidden)]
    fn threading_drop(cxx_qt_thread: &mut ffi::MyObjectCxxQtThread) {
        ffi::cxx_qt_ffi_MyObject_cxxQtThreadDrop(cxx_qt_thread);
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtThreadQueuedFn {
    inner: std::boxed::Box<dyn FnOnce(core::pin::Pin<&mut ffi::MyObject>) + Send>,
}
#[doc(hidden)]
pub fn route_arguments_MyObject_0<'a>(
    arg0: i32,
    arg1: &'a ffi::QString,
) -> ffi::CxxQtConstructorArgumentsMyObject0<'a> {
    #[allow(unused_variables)]
    #[allow(clippy::let_unit_value)]
    let (new_arguments, base_arguments, initialize_arguments) =
        <ffi::MyObject as cxx_qt::Constructor<(i32, &'a ffi::QString)>>::route_arguments((
            arg0, arg1,
        ));
    ffi::CxxQtConstructorArgumentsMyObject0 {
        base: ffi::CxxQtConstructorBaseArgumentsMyObject0 {
            arg0: base_arguments.0,
        },
        initialize: ffi::CxxQtConstructorInitializeArgumentsMyObject0 { not_empty: 0 },
        new: ffi::CxxQtConstructorNewArgumentsMyObject0 {
            arg0: new_arguments.0,
        },
    }
}
#[doc(hidden)]
#[allow(unused_variables)]
#[allow(clippy::extra_unused_lifetimes)]
#[allow(clippy::unnecessary_box_returns)]
pub fn new_rs_MyObject_0<'a>(
    new_arguments: ffi::CxxQtConstructorNewArgumentsMyObject0<'a>,
) -> std::boxed::Box<MyObjectRust> {
    std::boxed::Box::new(<ffi::MyObject as cxx_qt::Constructor<(
        i32,
        &'a ffi::QString,
    )>>::new((new_arguments.arg0,)))
}
#[doc(hidden)]
#[allow(unused_variables)]
#[allow(clippy::extra_unused_lifetimes)]
pub fn initialize_MyObject_0<'a>(
    qobject: core::pin::Pin<&mut ffi::MyObject>,
    initialize_arguments: ffi::CxxQtConstructorInitializeArgumentsMyObject0,
) {
    <ffi::MyObject as cxx_qt::Constructor<(i32, &'a ffi::QString)>>::initialize(qobject, ());
}
#[doc(hidden)]
pub fn route_arguments_MyObject_1() -> ffi::CxxQtConstructorArgumentsMyObject1 {
    #[allow(unused_variables)]
    #[allow(clippy::let_unit_value)]
    let (new_arguments, base_arguments, initialize_arguments) =
        <ffi::MyObject as cxx_qt::Constructor<()>>::route_arguments(());
    ffi::CxxQtConstructorArgumentsMyObject1 {
        base: ffi::CxxQtConstructorBaseArgumentsMyObject1 { not_empty: 0 },
        initialize: ffi::CxxQtConstructorInitializeArgumentsMyObject1 { not_empty: 0 },
        new: ffi::CxxQtConstructorNewArgumentsMyObject1 { not_empty: 0 },
    }
}
#[doc(hidden)]
#[allow(unused_variables)]
#[allow(clippy::extra_unused_lifetimes)]
#[allow(clippy::unnecessary_box_returns)]
pub fn new_rs_MyObject_1(
    new_arguments: ffi::CxxQtConstructorNewArgumentsMyObject1,
) -> std::boxed::Box<MyObjectRust> {
    std::boxed::Box::new(<ffi::MyObject as cxx_qt::Constructor<()>>::new(()))
}
#[doc(hidden)]
#[allow(unused_variables)]
#[allow(clippy::extra_unused_lifetimes)]
pub fn initialize_MyObject_1(
    qobject: core::pin::Pin<&mut ffi::MyObject>,
    initialize_arguments: ffi::CxxQtConstructorInitializeArgumentsMyObject1,
) {
    <ffi::MyObject as cxx_qt::Constructor<()>>::initialize(qobject, ());
}
impl ::core::ops::Deref for ffi::MyObject {
    type Target = MyObjectRust;
    fn deref(&self) -> &Self::Target {
        ffi::cxx_qt_ffi_MyObject_unsafeRust(self)
    }
}
impl ::cxx_qt::CxxQtType for ffi::MyObject {
    type Rust = MyObjectRust;
    fn rust(&self) -> &Self::Rust {
        ffi::cxx_qt_ffi_MyObject_unsafeRust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        ffi::cxx_qt_ffi_MyObject_unsafeRustMut(self)
    }
}
