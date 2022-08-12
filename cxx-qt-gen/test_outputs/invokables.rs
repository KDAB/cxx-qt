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

        #[cxx_name = "invokable"]
        fn invokable(self: &MyObject);
        #[cxx_name = "invokableCppObjWrapper"]
        fn invokable_cpp_obj_wrapper(self: &MyObject, cpp: Pin<&mut MyObjectQt>);
        #[cxx_name = "invokableMutable"]
        fn invokable_mutable(self: &mut MyObject);
        #[cxx_name = "invokableMutableCppObjWrapper"]
        fn invokable_mutable_cpp_obj_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>);
        #[cxx_name = "invokableParameters"]
        fn invokable_parameters(self: &MyObject, opaque: &QColor, trivial: &QPoint, primitive: i32);
        #[cxx_name = "invokableParametersCppObjWrapper"]
        fn invokable_parameters_cpp_obj_wrapper(
            self: &MyObject,
            primitive: i32,
            cpp: Pin<&mut MyObjectQt>,
        );
        #[cxx_name = "invokableReturnOpaqueWrapper"]
        fn invokable_return_opaque_wrapper(self: &mut MyObject) -> UniquePtr<QColor>;
        #[cxx_name = "invokableReturnPrimitive"]
        fn invokable_return_primitive(self: &mut MyObject) -> i32;
        #[cxx_name = "invokableReturnStaticWrapper"]
        fn invokable_return_static_wrapper(self: &mut MyObject) -> UniquePtr<QString>;
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

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

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

    #[derive(Default)]
    pub struct MyObject;

    impl MyObject {
        pub fn invokable_cpp_obj_wrapper(&self, cpp: std::pin::Pin<&mut FFICppObj>) {
            let mut cpp = CppObj::new(cpp);
            self.invokable_cpp_obj(&mut cpp);
        }

        pub fn invokable_mutable_cpp_obj_wrapper(&mut self, cpp: std::pin::Pin<&mut FFICppObj>) {
            let mut cpp = CppObj::new(cpp);
            self.invokable_mutable_cpp_obj(&mut cpp);
        }

        pub fn invokable_parameters_cpp_obj_wrapper(
            &self,
            primitive: i32,
            cpp: std::pin::Pin<&mut FFICppObj>,
        ) {
            let mut cpp = CppObj::new(cpp);
            self.invokable_parameters_cpp_obj(primitive, &mut cpp);
        }

        pub fn invokable_return_opaque_wrapper(&mut self) -> UniquePtr<cxx_qt_lib::QColor> {
            return self.invokable_return_opaque();
        }

        pub fn invokable_return_static_wrapper(&mut self) -> UniquePtr<cxx_qt_lib::QString> {
            return self.invokable_return_static();
        }

        pub fn invokable(&self) {
            println!("invokable");
        }

        pub fn invokable_cpp_obj(&self, cpp: &mut CppObj) {
            println!("cppobj");
        }

        pub fn invokable_mutable(&mut self) {
            println!("This method is mutable!");
        }

        pub fn invokable_mutable_cpp_obj(&mut self, cpp: &mut CppObj) {
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

        pub fn invokable_parameters_cpp_obj(&self, primitive: i32, cpp: &mut CppObj) {
            println!("{}", primitive);
        }

        pub fn invokable_return_opaque(&mut self) -> UniquePtr<QColor> {
            cxx_qt_lib::QColor::from_rgba(255, 0, 0, 0)
        }

        pub fn invokable_return_primitive(&mut self) -> i32 {
            2
        }

        pub fn invokable_return_static(&mut self) -> UniquePtr<QString> {
            QString::from_str("static")
        }

        pub fn cpp_context_method(&self) {
            println!("C++ context method");
        }

        pub fn cpp_context_method_mutable(&mut self) {
            println!("mutable method");
        }

        pub fn cpp_context_method_cpp_obj(&mut self, cpp: &mut CppObj) {
            println!("cppobj");
        }

        pub fn cpp_context_method_return_opaque(&self) -> UniquePtr<QColor> {
            cxx_qt_lib::QColor::from_rgba(255, 0, 0, 0)
        }
    }

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn grab_values_from_data(&mut self, mut data: Data) {}
    }

    pub struct Data;

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(_value: &CppObj<'a>) -> Self {
            Self {}
        }
    }

    impl<'a> From<&mut CppObj<'a>> for Data {
        fn from(_value: &mut CppObj<'a>) -> Self {
            Self::from(&*_value)
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

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(Data::default());
    }
}
