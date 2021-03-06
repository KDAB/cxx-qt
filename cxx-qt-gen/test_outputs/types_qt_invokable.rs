#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

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

        #[rust_name = "new_cpp_object"]
        fn newCppObject() -> UniquePtr<MyObject>;
    }

    extern "Rust" {
        type RustObj;

        #[cxx_name = "testColorWrapper"]
        fn test_color_wrapper(
            self: &RustObj,
            _cpp: Pin<&mut MyObject>,
            color: &QColor,
        ) -> UniquePtr<QColor>;

        #[cxx_name = "testDateWrapper"]
        fn test_date_wrapper(self: &RustObj, _cpp: Pin<&mut MyObject>, date: &QDate) -> QDate;

        #[cxx_name = "testDateTimeWrapper"]
        fn test_date_time_wrapper(
            self: &RustObj,
            _cpp: Pin<&mut MyObject>,
            dateTime: &QDateTime,
        ) -> UniquePtr<QDateTime>;

        #[cxx_name = "testPointWrapper"]
        fn test_point_wrapper(self: &RustObj, _cpp: Pin<&mut MyObject>, point: &QPoint) -> QPoint;

        #[cxx_name = "testPointfWrapper"]
        fn test_pointf_wrapper(
            self: &RustObj,
            _cpp: Pin<&mut MyObject>,
            pointf: &QPointF,
        ) -> QPointF;

        #[cxx_name = "testRectWrapper"]
        fn test_rect_wrapper(self: &RustObj, _cpp: Pin<&mut MyObject>, rect: &QRect) -> QRect;

        #[cxx_name = "testRectfWrapper"]
        fn test_rectf_wrapper(self: &RustObj, _cpp: Pin<&mut MyObject>, rectf: &QRectF) -> QRectF;

        #[cxx_name = "testSizeWrapper"]
        fn test_size_wrapper(self: &RustObj, _cpp: Pin<&mut MyObject>, size: &QSize) -> QSize;

        #[cxx_name = "testSizefWrapper"]
        fn test_sizef_wrapper(self: &RustObj, _cpp: Pin<&mut MyObject>, sizef: &QSizeF) -> QSizeF;

        #[cxx_name = "testStringWrapper"]
        fn test_string_wrapper(
            self: &RustObj,
            _cpp: Pin<&mut MyObject>,
            string: &QString,
        ) -> UniquePtr<QString>;

        #[cxx_name = "testTimeWrapper"]
        fn test_time_wrapper(self: &RustObj, _cpp: Pin<&mut MyObject>, time: &QTime) -> QTime;

        #[cxx_name = "testUrlWrapper"]
        fn test_url_wrapper(
            self: &RustObj,
            _cpp: Pin<&mut MyObject>,
            url: &QUrl,
        ) -> UniquePtr<QUrl>;

        #[cxx_name = "testVariantWrapper"]
        fn test_variant_wrapper(
            self: &RustObj,
            _cpp: Pin<&mut MyObject>,
            variant: &QVariant,
        ) -> UniquePtr<QVariant>;

        #[cxx_name = "createRs"]
        fn create_rs() -> Box<RustObj>;

        #[cxx_name = "initialiseCpp"]
        fn initialise_cpp(cpp: Pin<&mut MyObject>);
    }
}

pub use self::cxx_qt_my_object::*;
mod cxx_qt_my_object {
    use super::my_object::*;

    use cxx_qt_lib::ToUniquePtr;

    pub type FFICppObj = super::my_object::MyObject;

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        pub fn test_color_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            color: &cxx_qt_lib::QColorCpp,
        ) -> cxx::UniquePtr<cxx_qt_lib::QColorCpp> {
            let mut _cpp = CppObj::new(_cpp);
            let color = color.to_rust();
            return self.test_color(&mut _cpp, &color).to_unique_ptr();
        }

        pub fn test_date_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            date: &cxx_qt_lib::QDate,
        ) -> cxx_qt_lib::QDate {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_date(&mut _cpp, date);
        }

        pub fn test_date_time_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            dateTime: &cxx_qt_lib::QDateTimeCpp,
        ) -> cxx::UniquePtr<cxx_qt_lib::QDateTimeCpp> {
            let mut _cpp = CppObj::new(_cpp);
            let dateTime = dateTime.to_rust();
            return self.test_date_time(&mut _cpp, &dateTime).to_unique_ptr();
        }

        pub fn test_point_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            point: &cxx_qt_lib::QPoint,
        ) -> cxx_qt_lib::QPoint {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_point(&mut _cpp, point);
        }

        pub fn test_pointf_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            pointf: &cxx_qt_lib::QPointF,
        ) -> cxx_qt_lib::QPointF {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_pointf(&mut _cpp, pointf);
        }

        pub fn test_rect_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            rect: &cxx_qt_lib::QRect,
        ) -> cxx_qt_lib::QRect {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_rect(&mut _cpp, rect);
        }

        pub fn test_rectf_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            rectf: &cxx_qt_lib::QRectF,
        ) -> cxx_qt_lib::QRectF {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_rectf(&mut _cpp, rectf);
        }

        pub fn test_size_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            size: &cxx_qt_lib::QSize,
        ) -> cxx_qt_lib::QSize {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_size(&mut _cpp, size);
        }

        pub fn test_sizef_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            sizef: &cxx_qt_lib::QSizeF,
        ) -> cxx_qt_lib::QSizeF {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_sizef(&mut _cpp, sizef);
        }

        pub fn test_string_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            string: &cxx_qt_lib::QStringCpp,
        ) -> cxx::UniquePtr<cxx_qt_lib::QStringCpp> {
            let mut _cpp = CppObj::new(_cpp);
            let string = string.to_rust();
            return self.test_string(&mut _cpp, &string).to_unique_ptr();
        }

        pub fn test_time_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            time: &cxx_qt_lib::QTime,
        ) -> cxx_qt_lib::QTime {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_time(&mut _cpp, time);
        }

        pub fn test_url_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            url: &cxx_qt_lib::QUrlCpp,
        ) -> cxx::UniquePtr<cxx_qt_lib::QUrlCpp> {
            let mut _cpp = CppObj::new(_cpp);
            let url = url.to_rust();
            return self.test_url(&mut _cpp, &url).to_unique_ptr();
        }

        pub fn test_variant_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            variant: &cxx_qt_lib::QVariantCpp,
        ) -> cxx::UniquePtr<cxx_qt_lib::QVariantCpp> {
            let mut _cpp = CppObj::new(_cpp);
            let variant = variant.to_rust();
            return self.test_variant(&mut _cpp, &variant).to_unique_ptr();
        }

        pub fn test_color(&self, _cpp: &mut CppObj, color: &QColor) -> QColor {
            color
        }

        pub fn test_date(&self, _cpp: &mut CppObj, date: &QDate) -> QDate {
            date
        }

        pub fn test_date_time(&self, _cpp: &mut CppObj, dateTime: &QDateTime) -> QDateTime {
            dateTime
        }

        pub fn test_point(&self, _cpp: &mut CppObj, point: &QPoint) -> QPoint {
            point
        }

        pub fn test_pointf(&self, _cpp: &mut CppObj, pointf: &QPointF) -> QPointF {
            pointf
        }

        pub fn test_rect(&self, _cpp: &mut CppObj, rect: &QRect) -> QRect {
            rect
        }

        pub fn test_rectf(&self, _cpp: &mut CppObj, rectf: &QRectF) -> QRectF {
            rectf
        }

        pub fn test_size(&self, _cpp: &mut CppObj, size: &QSize) -> QSize {
            size
        }

        pub fn test_sizef(&self, _cpp: &mut CppObj, sizef: &QSizeF) -> QSizeF {
            sizef
        }

        pub fn test_string(&self, _cpp: &mut CppObj, string: &str) -> String {
            string.to_owned()
        }

        pub fn test_time(&self, _cpp: &mut CppObj, time: &QTime) -> QTime {
            time
        }

        pub fn test_url(&self, _cpp: &mut CppObj, url: &QUrl) -> QUrl {
            url
        }

        pub fn test_variant(&self, _cpp: &mut CppObj, variant: &QVariant) -> QVariant {
            variant
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

    #[derive(Default)]
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
