mod my_object {
    use cxx_qt_lib::Color;
    use cxx_qt_lib::ToUniquePtr;

    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;

            include!("cxx-qt-lib/include/qt_types.h");
            #[namespace = ""]
            type QColor = cxx_qt_lib::QColor;
            #[namespace = ""]
            type QDate = cxx_qt_lib::QDate;
            #[namespace = ""]
            type QDateTime = cxx_qt_lib::QDateTime;
            #[namespace = ""]
            type QPoint = cxx_qt_lib::QPoint;
            #[namespace = ""]
            type QPointF = cxx_qt_lib::QPointF;
            #[namespace = ""]
            type QRect = cxx_qt_lib::QRect;
            #[namespace = ""]
            type QRectF = cxx_qt_lib::QRectF;
            #[namespace = ""]
            type QSize = cxx_qt_lib::QSize;
            #[namespace = ""]
            type QSizeF = cxx_qt_lib::QSizeF;
            #[namespace = ""]
            type QString = cxx_qt_lib::QString;
            #[namespace = ""]
            type QTime = cxx_qt_lib::QTime;
            #[namespace = ""]
            type QUrl = cxx_qt_lib::QUrl;
            #[namespace = ""]
            type QVariant = cxx_qt_lib::QVariant;

            #[namespace = "cxx_qt::nested_object"]
            type NestedObject = crate::nested_object::FFICppObj;

            #[rust_name = "new_cpp_object"]
            fn newCppObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "invokable"]
            fn invokable(self: &RustObj);
            #[cxx_name = "invokableCppObjWrapper"]
            fn invokable_cpp_obj_wrapper(self: &RustObj, cpp: Pin<&mut MyObject>);
            #[cxx_name = "invokableMutable"]
            fn invokable_mutable(self: &mut RustObj);
            #[cxx_name = "invokableMutableCppObjWrapper"]
            fn invokable_mutable_cpp_obj_wrapper(self: &mut RustObj, cpp: Pin<&mut MyObject>);
            #[cxx_name = "invokableNestedParameterWrapper"]
            fn invokable_nested_parameter_wrapper(self: &RustObj, nested: Pin<&mut NestedObject>);
            #[cxx_name = "invokableParametersWrapper"]
            fn invokable_parameters_wrapper(self: &RustObj, opaque: &QColor, primitive: i32);
            #[cxx_name = "invokableParametersCppObjWrapper"]
            fn invokable_parameters_cpp_obj_wrapper(
                self: &RustObj,
                primitive: i32,
                cpp: Pin<&mut MyObject>,
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
            fn initialise_cpp(cpp: Pin<&mut MyObject>);
        }
    }

    pub type FFICppObj = ffi::MyObject;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        fn invokable_cpp_obj_wrapper(&self, cpp: std::pin::Pin<&mut FFICppObj>) {
            let mut cpp = CppObj::new(cpp);
            self.invokable_cpp_obj(&mut cpp);
        }

        fn invokable_mutable_cpp_obj_wrapper(&mut self, cpp: std::pin::Pin<&mut FFICppObj>) {
            let mut cpp = CppObj::new(cpp);
            self.invokable_mutable_cpp_obj(&mut cpp);
        }

        fn invokable_nested_parameter_wrapper(
            &self,
            nested: std::pin::Pin<&mut crate::nested_object::FFICppObj>,
        ) {
            let mut nested = crate::nested_object::CppObj::new(nested);
            self.invokable_nested_parameter(&mut nested);
        }

        fn invokable_parameters_wrapper(&self, opaque: &cxx_qt_lib::QColor, primitive: i32) {
            let opaque = opaque.to_rust();
            self.invokable_parameters(&opaque, primitive);
        }

        fn invokable_parameters_cpp_obj_wrapper(
            &self,
            primitive: i32,
            cpp: std::pin::Pin<&mut FFICppObj>,
        ) {
            let mut cpp = CppObj::new(cpp);
            self.invokable_parameters_cpp_obj(primitive, &mut cpp);
        }

        fn invokable_return_opaque_wrapper(&mut self) -> cxx::UniquePtr<cxx_qt_lib::QColor> {
            return self.invokable_return_opaque().to_unique_ptr();
        }

        fn invokable_return_static_wrapper(&mut self) -> cxx::UniquePtr<cxx_qt_lib::QString> {
            return self.invokable_return_static().to_unique_ptr();
        }

        fn invokable(&self) {
            println!("invokable");
        }

        fn invokable_cpp_obj(&self, cpp: &mut CppObj) {
            println!("cppobj");
        }

        fn invokable_mutable(&mut self) {
            println!("This method is mutable!");
        }

        fn invokable_mutable_cpp_obj(&mut self, cpp: &mut CppObj) {
            println!("This method is mutable!");
        }

        fn invokable_nested_parameter(&self, nested: &mut crate::nested_object::CppObj) {
            println!("nested!");
        }

        fn invokable_parameters(&self, opaque: &Color, primitive: i32) {
            println!("Red: {} Number: {}", opaque.red(), primitive);
        }

        fn invokable_parameters_cpp_obj(&self, primitive: i32, cpp: &mut CppObj) {
            println!("{}", primitive);
        }

        fn invokable_return_opaque(&mut self) -> Color {
            cxx_qt_lib::Color::from_rgba(255, 0, 0, 0)
        }

        fn invokable_return_primitive(&mut self) -> i32 {
            2
        }

        fn invokable_return_static(&mut self) -> &str {
            "static"
        }

        fn rust_only_method(&self) {
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

    struct Data;

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

    fn create_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(Data::default());
    }
}
