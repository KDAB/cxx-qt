#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[cxx_name = "unsafe_rust"]
        fn rust(self: &MyObjectQt) -> &MyObject;
        #[rust_name = "new_cpp_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafe_rust_mut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;

        #[cxx_name = "testColorWrapper"]
        fn test_color_wrapper(
            self: &MyObject,
            _cpp: Pin<&mut MyObjectQt>,
            color: &QColor,
        ) -> UniquePtr<QColor>;

        #[cxx_name = "testDateWrapper"]
        fn test_date_wrapper(self: &MyObject, _cpp: Pin<&mut MyObjectQt>, date: &QDate) -> QDate;

        #[cxx_name = "testDateTimeWrapper"]
        fn test_date_time_wrapper(
            self: &MyObject,
            _cpp: Pin<&mut MyObjectQt>,
            dateTime: &QDateTime,
        ) -> UniquePtr<QDateTime>;

        #[cxx_name = "testPointWrapper"]
        fn test_point_wrapper(
            self: &MyObject,
            _cpp: Pin<&mut MyObjectQt>,
            point: &QPoint,
        ) -> QPoint;

        #[cxx_name = "testPointfWrapper"]
        fn test_pointf_wrapper(
            self: &MyObject,
            _cpp: Pin<&mut MyObjectQt>,
            pointf: &QPointF,
        ) -> QPointF;

        #[cxx_name = "testRectWrapper"]
        fn test_rect_wrapper(self: &MyObject, _cpp: Pin<&mut MyObjectQt>, rect: &QRect) -> QRect;

        #[cxx_name = "testRectfWrapper"]
        fn test_rectf_wrapper(
            self: &MyObject,
            _cpp: Pin<&mut MyObjectQt>,
            rectf: &QRectF,
        ) -> QRectF;

        #[cxx_name = "testSizeWrapper"]
        fn test_size_wrapper(self: &MyObject, _cpp: Pin<&mut MyObjectQt>, size: &QSize) -> QSize;

        #[cxx_name = "testSizefWrapper"]
        fn test_sizef_wrapper(
            self: &MyObject,
            _cpp: Pin<&mut MyObjectQt>,
            sizef: &QSizeF,
        ) -> QSizeF;

        #[cxx_name = "testStringWrapper"]
        fn test_string_wrapper(
            self: &MyObject,
            _cpp: Pin<&mut MyObjectQt>,
            string: &QString,
        ) -> UniquePtr<QString>;

        #[cxx_name = "testTimeWrapper"]
        fn test_time_wrapper(self: &MyObject, _cpp: Pin<&mut MyObjectQt>, time: &QTime) -> QTime;

        #[cxx_name = "testUrlWrapper"]
        fn test_url_wrapper(
            self: &MyObject,
            _cpp: Pin<&mut MyObjectQt>,
            url: &QUrl,
        ) -> UniquePtr<QUrl>;

        #[cxx_name = "testVariantWrapper"]
        fn test_variant_wrapper(
            self: &MyObject,
            _cpp: Pin<&mut MyObjectQt>,
            variant: &QVariant,
        ) -> UniquePtr<QVariant>;

        #[cxx_name = "createRs"]
        fn create_rs() -> Box<MyObject>;

        #[cxx_name = "initialiseCpp"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QColor = cxx_qt_lib::QColor;
        type QDate = cxx_qt_lib::QDate;
        type QDateTime = cxx_qt_lib::QDateTime;
        type QPoint = cxx_qt_lib::QPoint;
        type QPointF = cxx_qt_lib::QPointF;
        type QRect = cxx_qt_lib::QRect;
        type QRectF = cxx_qt_lib::QRectF;
        type QSize = cxx_qt_lib::QSize;
        type QSizeF = cxx_qt_lib::QSizeF;
        type QString = cxx_qt_lib::QString;
        type QTime = cxx_qt_lib::QTime;
        type QUrl = cxx_qt_lib::QUrl;
        type QVariant = cxx_qt_lib::QVariant;
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
        pub fn test_color_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            color: &cxx_qt_lib::QColor,
        ) -> UniquePtr<cxx_qt_lib::QColor> {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_color(&mut _cpp, color);
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
            dateTime: &cxx_qt_lib::QDateTime,
        ) -> UniquePtr<cxx_qt_lib::QDateTime> {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_date_time(&mut _cpp, dateTime);
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
            string: &cxx_qt_lib::QString,
        ) -> UniquePtr<cxx_qt_lib::QString> {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_string(&mut _cpp, string);
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
            url: &cxx_qt_lib::QUrl,
        ) -> UniquePtr<cxx_qt_lib::QUrl> {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_url(&mut _cpp, url);
        }

        pub fn test_variant_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            variant: &cxx_qt_lib::QVariant,
        ) -> UniquePtr<cxx_qt_lib::QVariant> {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_variant(&mut _cpp, variant);
        }

        pub fn test_color(&self, _cpp: &mut CppObj, color: &QColor) -> UniquePtr<QColor> {
            color
        }

        pub fn test_date(&self, _cpp: &mut CppObj, date: &QDate) -> QDate {
            date
        }

        pub fn test_date_time(
            &self,
            _cpp: &mut CppObj,
            dateTime: &QDateTime,
        ) -> UniquePtr<QDateTime> {
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

        pub fn test_string(&self, _cpp: &mut CppObj, string: &QString) -> UniquePtr<QString> {
            string.to_owned()
        }

        pub fn test_time(&self, _cpp: &mut CppObj, time: &QTime) -> QTime {
            time
        }

        pub fn test_url(&self, _cpp: &mut CppObj, url: &QUrl) -> UniquePtr<QUrl> {
            url
        }

        pub fn test_variant(&self, _cpp: &mut CppObj, variant: &QVariant) -> UniquePtr<QVariant> {
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

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(Data::default());
    }
}
