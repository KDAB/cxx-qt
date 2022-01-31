mod my_object {
    use cxx_qt_lib::Color;
    use cxx_qt_lib::ToUniquePtr;

    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        enum Property {
            Primitive,
            Opaque,
            Nested,
        }

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
            type QVariant = cxx_qt_lib::QVariantCpp;

            #[rust_name = "primitive"]
            fn getPrimitive(self: &MyObject) -> i32;
            #[rust_name = "set_primitive"]
            fn setPrimitive(self: Pin<&mut MyObject>, value: i32);

            #[rust_name = "opaque"]
            fn getOpaque(self: &MyObject) -> &QColor;
            #[rust_name = "set_opaque"]
            fn setOpaque(self: Pin<&mut MyObject>, value: &QColor);

            #[namespace = "cxx_qt::nested_object"]
            type NestedObject = crate::nested_object::FFICppObj;

            #[rust_name = "take_nested"]
            fn takeNested(self: Pin<&mut MyObject>) -> UniquePtr<NestedObject>;
            #[rust_name = "give_nested"]
            fn giveNested(self: Pin<&mut MyObject>, value: UniquePtr<NestedObject>);

            #[rust_name = "new_cpp_object"]
            fn newCppObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "createRs"]
            fn create_rs() -> Box<RustObj>;

            #[cxx_name = "initialiseCpp"]
            fn initialise_cpp(cpp: Pin<&mut MyObject>);
        }
    }

    pub type FFICppObj = ffi::MyObject;
    pub type Property = ffi::Property;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {}

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn primitive(&self) -> i32 {
            self.cpp.primitive()
        }

        pub fn set_primitive(&mut self, value: i32) {
            self.cpp.as_mut().set_primitive(value);
        }

        pub fn opaque(&self) -> cxx_qt_lib::Color {
            self.cpp.opaque().to_rust()
        }

        pub fn set_opaque(&mut self, value: cxx_qt_lib::Color) {
            self.cpp.as_mut().set_opaque(&value.to_unique_ptr());
        }

        pub fn take_nested(&mut self) -> cxx::UniquePtr<ffi::NestedObject> {
            self.cpp.as_mut().take_nested()
        }

        pub fn give_nested(&mut self, value: cxx::UniquePtr<ffi::NestedObject>) {
            self.cpp.as_mut().give_nested(value);
        }

        pub fn grab_values_from_data(&mut self, mut data: Data) {
            self.set_primitive(data.primitive);
            self.set_opaque(std::mem::take(&mut data.opaque));
        }
    }

    #[derive(Default)]
    struct Data {
        primitive: i32,
        opaque: Color,
    }

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(value: &CppObj<'a>) -> Self {
            Self {
                primitive: value.primitive().into(),
                opaque: value.opaque().into(),
            }
        }
    }

    impl<'a> From<&mut CppObj<'a>> for Data {
        fn from(value: &mut CppObj<'a>) -> Self {
            Self::from(&*value)
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
