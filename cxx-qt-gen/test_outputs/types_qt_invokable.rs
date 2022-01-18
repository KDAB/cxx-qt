mod my_object {
    use cxx_qt_lib::QString;
    use cxx_qt_lib::ToUniquePtr;

    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        enum Property {}

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
            fn test_point_wrapper(
                self: &RustObj,
                _cpp: Pin<&mut MyObject>,
                point: &QPoint,
            ) -> QPoint;

            #[cxx_name = "testPointfWrapper"]
            fn test_pointf_wrapper(
                self: &RustObj,
                _cpp: Pin<&mut MyObject>,
                pointf: &QPointF,
            ) -> QPointF;

            #[cxx_name = "testRectWrapper"]
            fn test_rect_wrapper(self: &RustObj, _cpp: Pin<&mut MyObject>, rect: &QRect) -> QRect;

            #[cxx_name = "testRectfWrapper"]
            fn test_rectf_wrapper(
                self: &RustObj,
                _cpp: Pin<&mut MyObject>,
                rectf: &QRectF,
            ) -> QRectF;

            #[cxx_name = "testSizeWrapper"]
            fn test_size_wrapper(self: &RustObj, _cpp: Pin<&mut MyObject>, size: &QSize) -> QSize;

            #[cxx_name = "testSizefWrapper"]
            fn test_sizef_wrapper(
                self: &RustObj,
                _cpp: Pin<&mut MyObject>,
                sizef: &QSizeF,
            ) -> QSizeF;

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

    pub type FFICppObj = ffi::MyObject;
    pub type Property = ffi::Property;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        fn test_color_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            color: &QColor,
        ) -> cxx::UniquePtr<cxx_qt_lib::QColor> {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_color(&mut _cpp, color).to_unique_ptr();
        }

        fn test_date_wrapper(&self, _cpp: std::pin::Pin<&mut FFICppObj>, date: &QDate) -> QDate {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_date(&mut _cpp, date);
        }

        fn test_date_time_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            dateTime: &QDateTime,
        ) -> cxx::UniquePtr<cxx_qt_lib::QDateTime> {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_date_time(&mut _cpp, dateTime).to_unique_ptr();
        }

        fn test_point_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            point: &QPoint,
        ) -> QPoint {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_point(&mut _cpp, point);
        }

        fn test_pointf_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            pointf: &QPointF,
        ) -> QPointF {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_pointf(&mut _cpp, pointf);
        }

        fn test_rect_wrapper(&self, _cpp: std::pin::Pin<&mut FFICppObj>, rect: &QRect) -> QRect {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_rect(&mut _cpp, rect);
        }

        fn test_rectf_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            rectf: &QRectF,
        ) -> QRectF {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_rectf(&mut _cpp, rectf);
        }

        fn test_size_wrapper(&self, _cpp: std::pin::Pin<&mut FFICppObj>, size: &QSize) -> QSize {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_size(&mut _cpp, size);
        }

        fn test_sizef_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            sizef: &QSizeF,
        ) -> QSizeF {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_sizef(&mut _cpp, sizef);
        }

        fn test_string_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            string: &QString,
        ) -> cxx::UniquePtr<cxx_qt_lib::QString> {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_string(&mut _cpp, string).to_unique_ptr();
        }

        fn test_time_wrapper(&self, _cpp: std::pin::Pin<&mut FFICppObj>, time: &QTime) -> QTime {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_time(&mut _cpp, time);
        }

        fn test_url_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            url: &QUrl,
        ) -> cxx::UniquePtr<cxx_qt_lib::QUrl> {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_url(&mut _cpp, url).to_unique_ptr();
        }

        fn test_variant_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            variant: &QVariant,
        ) -> cxx::UniquePtr<cxx_qt_lib::QVariant> {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_variant(&mut _cpp, variant).to_unique_ptr();
        }

        fn test_color(&self, _cpp: &mut CppObj, color: &QColor) -> Color {
            color
        }

        fn test_date(&self, _cpp: &mut CppObj, date: &QDate) -> QDate {
            date
        }

        fn test_date_time(&self, _cpp: &mut CppObj, dateTime: &QDateTime) -> DateTime {
            dateTime
        }

        fn test_point(&self, _cpp: &mut CppObj, point: &QPoint) -> QPoint {
            point
        }

        fn test_pointf(&self, _cpp: &mut CppObj, pointf: &QPointF) -> QPointF {
            pointf
        }

        fn test_rect(&self, _cpp: &mut CppObj, rect: &QRect) -> QRect {
            rect
        }

        fn test_rectf(&self, _cpp: &mut CppObj, rectf: &QRectF) -> QRectF {
            rectf
        }

        fn test_size(&self, _cpp: &mut CppObj, size: &QSize) -> QSize {
            size
        }

        fn test_sizef(&self, _cpp: &mut CppObj, sizef: &QSizeF) -> QSizeF {
            sizef
        }

        fn test_string(&self, _cpp: &mut CppObj, string: &QString) -> String {
            string.to_rust()
        }

        fn test_time(&self, _cpp: &mut CppObj, time: &QTime) -> QTime {
            time
        }

        fn test_url(&self, _cpp: &mut CppObj, url: &QUrl) -> Url {
            url
        }

        fn test_variant(&self, _cpp: &mut CppObj, variant: &QVariant) -> Variant {
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

        pub fn update_requester(&self) -> cxx_qt_lib::update_requester::UpdateRequester {
            use cxx_qt_lib::update_requester::{CxxQObject, UpdateRequester};

            let ptr: *const FFICppObj = unsafe { &*self.cpp.as_ref() };
            unsafe { UpdateRequester::new(ptr as *mut CxxQObject) }
        }

        pub fn grab_values_from_data(&mut self, data: &Data) {
            use cxx_qt_lib::MapQtValue;
        }
    }

    #[derive(Default)]
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
        wrapper.grab_values_from_data(&Data::default());
    }
}
