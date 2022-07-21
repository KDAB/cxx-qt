#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[namespace = "cxx_qt::nested_object"]
        type NestedObject = crate::cxx_qt_nested_object::FFICppObj;

        #[rust_name = "new_cpp_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type RustObj;

        #[cxx_name = "invokable"]
        fn invokable(self: &RustObj);
        #[cxx_name = "invokableCppObjWrapper"]
        fn invokable_cpp_obj_wrapper(self: &RustObj, cpp: Pin<&mut MyObjectQt>);
        #[cxx_name = "invokableMutable"]
        fn invokable_mutable(self: &mut RustObj);
        #[cxx_name = "invokableMutableCppObjWrapper"]
        fn invokable_mutable_cpp_obj_wrapper(self: &mut RustObj, cpp: Pin<&mut MyObjectQt>);
        #[cxx_name = "invokableNestedParameterWrapper"]
        fn invokable_nested_parameter_wrapper(self: &RustObj, nested: Pin<&mut NestedObject>);
        #[cxx_name = "invokableParameters"]
        fn invokable_parameters(self: &RustObj, opaque: &QColor, trivial: &QPoint, primitive: i32);
        #[cxx_name = "invokableParametersCppObjWrapper"]
        fn invokable_parameters_cpp_obj_wrapper(
            self: &RustObj,
            primitive: i32,
            cpp: Pin<&mut MyObjectQt>,
        );
        #[cxx_name = "invokableReturnOpaqueWrapper"]
        fn invokable_return_opaque_wrapper(self: &mut RustObj) -> UniquePtr<QColor>;
        #[cxx_name = "invokableReturnPrimitive"]
        fn invokable_return_primitive(self: &mut RustObj) -> i32;
        #[cxx_name = "invokableReturnStaticWrapper"]
        fn invokable_return_static_wrapper(self: &mut RustObj) -> UniquePtr<QString>;

        #[cxx_name = "createRs"]
        fn create_rs() -> Box<RustObj>;

        #[cxx_name = "initialiseCpp"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QColor = cxx_qt_lib::QColor;
        type QPoint = cxx_qt_lib::QPoint;
        type QString = cxx_qt_lib::QString;
    }
}

pub use self::cxx_qt_my_object::*;
mod cxx_qt_my_object {
    use super::my_object::*;

    pub type FFICppObj = super::my_object::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        pub fn invokable_cpp_obj_wrapper(&self, cpp: std::pin::Pin<&mut FFICppObj>) {
            let mut cpp = CppObj::new(cpp);
            self.invokable_cpp_obj(&mut cpp);
        }

        pub fn invokable_mutable_cpp_obj_wrapper(&mut self, cpp: std::pin::Pin<&mut FFICppObj>) {
            let mut cpp = CppObj::new(cpp);
            self.invokable_mutable_cpp_obj(&mut cpp);
        }

        pub fn invokable_nested_parameter_wrapper(
            &self,
            nested: std::pin::Pin<&mut crate::cxx_qt_nested_object::FFICppObj>,
        ) {
            let mut nested = crate::cxx_qt_nested_object::CppObj::new(nested);
            self.invokable_nested_parameter(&mut nested);
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

        pub fn invokable_nested_parameter(&self, nested: &mut crate::cxx_qt_nested_object::CppObj) {
            println!("nested!");
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

        pub fn rust_only_method(&self) {
            println!("QML can't call this :)");
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

    pub fn create_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(Data::default());
    }
}
