mod my_object {
    use cxx_qt_lib::ToUniquePtr;

    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        enum Property {
            Public,
        }

        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;

            include!("cxx-qt-lib/include/qt_types.h");
            #[namespace = ""]
            type QColor = cxx_qt_lib::QColorCpp;
            #[namespace = ""]
            type QDate = cxx_qt_lib::QDate;
            #[namespace = ""]
            type QDateTime = cxx_qt_lib::QDateTimeCpp;
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
            type QString = cxx_qt_lib::QStringCpp;
            #[namespace = ""]
            type QTime = cxx_qt_lib::QTime;
            #[namespace = ""]
            type QUrl = cxx_qt_lib::QUrlCpp;
            #[namespace = ""]
            type QVariant = cxx_qt_lib::QVariantCpp;

            #[rust_name = "public"]
            fn getPublic(self: &MyObject) -> i32;
            #[rust_name = "set_public"]
            fn setPublic(self: Pin<&mut MyObject>, value: i32);

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

    struct RustObj {
        private: i32,
    }

    impl RustObj {}

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn public(&self) -> i32 {
            self.cpp.public()
        }

        pub fn set_public(&mut self, value: i32) {
            self.cpp.as_mut().set_public(value);
        }

        pub fn grab_values_from_data(&mut self, mut data: Data) {
            self.set_public(data.public);
        }
    }

    struct Data {
        public: i32,
    }

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(value: &CppObj<'a>) -> Self {
            Self {
                public: value.public().into(),
            }
        }
    }

    impl<'a> From<&mut CppObj<'a>> for Data {
        fn from(value: &mut CppObj<'a>) -> Self {
            Self::from(&*value)
        }
    }

    impl Default for Data {
        fn default() -> Self {
            Self { public: 32 }
        }
    }

    impl Default for RustObj {
        fn default() -> Self {
            Self { private: 64 }
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
