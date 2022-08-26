#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;

        #[cxx_name = "invokableWrapper"]
        fn invokable_wrapper(self: &MyObject, cpp: &MyObjectQt);
        #[cxx_name = "invokableMutableWrapper"]
        fn invokable_mutable_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>);
        #[cxx_name = "invokableParametersWrapper"]
        fn invokable_parameters_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            opaque: &QColor,
            trivial: &QPoint,
            primitive: i32,
        );
        #[cxx_name = "invokableReturnOpaqueWrapper"]
        fn invokable_return_opaque_wrapper(
            self: &mut MyObject,
            cpp: Pin<&mut MyObjectQt>,
        ) -> UniquePtr<QColor>;
        #[cxx_name = "invokableReturnPrimitiveWrapper"]
        fn invokable_return_primitive_wrapper(
            self: &mut MyObject,
            cpp: Pin<&mut MyObjectQt>,
        ) -> i32;
        #[cxx_name = "invokableReturnStaticWrapper"]
        fn invokable_return_static_wrapper(
            self: &mut MyObject,
            cpp: Pin<&mut MyObjectQt>,
        ) -> UniquePtr<QString>;
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QColor = cxx_qt_lib::QColor;
        type QPoint = cxx_qt_lib::QPoint;
        type QString = cxx_qt_lib::QString;
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

    impl MyObject {
        pub fn invokable_wrapper(&self, cpp: &FFICppObj) {
            cpp.invokable();
        }

        pub fn invokable_mutable_wrapper(&mut self, cpp: std::pin::Pin<&mut FFICppObj>) {
            cpp.invokable_mutable();
        }

        pub fn invokable_parameters_wrapper(
            &self,
            cpp: &FFICppObj,
            opaque: &cxx_qt_lib::QColor,
            trivial: &cxx_qt_lib::QPoint,
            primitive: i32,
        ) {
            cpp.invokable_parameters(opaque, trivial, primitive);
        }

        pub fn invokable_return_opaque_wrapper(
            &mut self,
            cpp: std::pin::Pin<&mut FFICppObj>,
        ) -> UniquePtr<cxx_qt_lib::QColor> {
            return cpp.invokable_return_opaque();
        }

        pub fn invokable_return_primitive_wrapper(
            &mut self,
            cpp: std::pin::Pin<&mut FFICppObj>,
        ) -> i32 {
            return cpp.invokable_return_primitive();
        }

        pub fn invokable_return_static_wrapper(
            &mut self,
            cpp: std::pin::Pin<&mut FFICppObj>,
        ) -> UniquePtr<cxx_qt_lib::QString> {
            return cpp.invokable_return_static();
        }
    }

    impl MyObjectQt {
        pub fn invokable(&self) {
            println!("invokable");
        }

        pub fn invokable_mutable(self: Pin<&mut Self>) {
            println!("This method is mutable!");
        }

        pub fn invokable_parameters(&self, opaque: &QColor, trivial: &QPoint, primitive: i32) {
            println!(
                "Red: {}, Point X: {}, Number: {}",
                opaque.red(),
                trivial.x(),
                primitive,
            );
        }

        pub fn invokable_return_opaque(self: Pin<&mut Self>) -> UniquePtr<QColor> {
            cxx_qt_lib::QColor::from_rgba(255, 0, 0, 0)
        }

        pub fn invokable_return_primitive(self: Pin<&mut Self>) -> i32 {
            2
        }

        pub fn invokable_return_static(self: Pin<&mut Self>) -> UniquePtr<QString> {
            QString::from_str("static")
        }

        pub fn cpp_context_method(&self) {
            println!("C++ context method");
        }

        pub fn cpp_context_method_mutable(self: Pin<&mut Self>) {
            println!("mutable method");
        }

        pub fn cpp_context_method_return_opaque(&self) -> UniquePtr<QColor> {
            cxx_qt_lib::QColor::from_rgba(255, 0, 0, 0)
        }

        pub fn grab_values_from_data(mut self: Pin<&mut Self>, mut data: Data) {}
    }

    pub struct Data;

    impl From<&MyObjectQt> for Data {
        fn from(_value: &MyObjectQt) -> Self {
            Self {}
        }
    }

    impl MyObject {
        pub fn rust_only_method(&self) {
            println!("QML or C++ can't call this :)");
        }
    }

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut MyObjectQt>) {
        cpp.grab_values_from_data(Data::default());
    }
}
