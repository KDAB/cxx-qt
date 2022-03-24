mod my_object {
    use cxx_qt_lib::ToUniquePtr;

    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        enum Property {
            PropertyName,
        }

        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;

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

            #[rust_name = "property_name"]
            fn getPropertyName(self: &MyObject) -> i32;
            #[rust_name = "set_property_name"]
            fn setPropertyName(self: Pin<&mut MyObject>, value: i32);

            #[rust_name = "new_cpp_object"]
            fn newCppObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "invokableName"]
            fn invokable_name(self: &RustObj);

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

    impl RustObj {
        fn invokable_name(&self) {
            println!("Bye from Rust!");
        }
    }

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn property_name(&self) -> i32 {
            self.cpp.property_name()
        }

        pub fn set_property_name(&mut self, value: i32) {
            self.cpp.as_mut().set_property_name(value);
        }

        pub fn update_requester(&self) -> cxx_qt_lib::update_requester::UpdateRequester {
            use cxx_qt_lib::update_requester::{CxxQObject, UpdateRequester};

            let ptr: *const FFICppObj = unsafe { &*self.cpp.as_ref() };
            unsafe { UpdateRequester::new(ptr as *mut CxxQObject) }
        }

        pub fn grab_values_from_data(&mut self, data: &Data) {
            use cxx_qt_lib::MapQtValue;

            data.property_name.map_qt_value(
                |context, converted| context.set_property_name(converted),
                self,
            );
        }
    }

    #[derive(Default)]
    struct Data {
        property_name: i32,
    }

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(value: &CppObj<'a>) -> Self {
            Self {
                property_name: value.property_name().into(),
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
        wrapper.grab_values_from_data(&Data::default());
    }
}
