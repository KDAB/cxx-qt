#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
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

    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/include/convert.h");
        include!("cxx-qt-lib/include/cxxqt_thread.h");
    }

    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;

        #[cxx_name = "testColorWrapper"]
        fn test_color_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            color: &QColor,
        ) -> UniquePtr<QColor>;

        #[cxx_name = "testDateWrapper"]
        fn test_date_wrapper(self: &MyObject, cpp: &MyObjectQt, date: &QDate) -> QDate;

        #[cxx_name = "testDateTimeWrapper"]
        fn test_date_time_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            dateTime: &QDateTime,
        ) -> UniquePtr<QDateTime>;

        #[cxx_name = "testPointWrapper"]
        fn test_point_wrapper(self: &MyObject, cpp: &MyObjectQt, point: &QPoint) -> QPoint;

        #[cxx_name = "testPointfWrapper"]
        fn test_pointf_wrapper(self: &MyObject, cpp: &MyObjectQt, pointf: &QPointF) -> QPointF;

        #[cxx_name = "testRectWrapper"]
        fn test_rect_wrapper(self: &MyObject, cpp: &MyObjectQt, rect: &QRect) -> QRect;

        #[cxx_name = "testRectfWrapper"]
        fn test_rectf_wrapper(self: &MyObject, cpp: &MyObjectQt, rectf: &QRectF) -> QRectF;

        #[cxx_name = "testSizeWrapper"]
        fn test_size_wrapper(self: &MyObject, cpp: &MyObjectQt, size: &QSize) -> QSize;

        #[cxx_name = "testSizefWrapper"]
        fn test_sizef_wrapper(self: &MyObject, cpp: &MyObjectQt, sizef: &QSizeF) -> QSizeF;

        #[cxx_name = "testStringWrapper"]
        fn test_string_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            string: &QString,
        ) -> UniquePtr<QString>;

        #[cxx_name = "testTimeWrapper"]
        fn test_time_wrapper(self: &MyObject, cpp: &MyObjectQt, time: &QTime) -> QTime;

        #[cxx_name = "testUrlWrapper"]
        fn test_url_wrapper(self: &MyObject, cpp: &MyObjectQt, url: &QUrl) -> UniquePtr<QUrl>;

        #[cxx_name = "testVariantWrapper"]
        fn test_variant_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            variant: &QVariant,
        ) -> UniquePtr<QVariant>;
    }

    unsafe extern "C++" {
        type MyObjectCxxQtThread;

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObjectQt) -> UniquePtr<MyObjectCxxQtThread>;
        fn queue(self: &MyObjectCxxQtThread, func: fn(ctx: Pin<&mut MyObjectQt>)) -> Result<()>;

        #[rust_name = "new_cpp_object_my_object_qt"]
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
        fn create_rs_my_object() -> Box<MyObject>;
    }
}

pub use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;

    type UniquePtr<T> = cxx::UniquePtr<T>;

    use std::pin::Pin;

    #[derive(Default)]
    pub struct MyObject;

    impl MyObject {
        pub fn test_color_wrapper(
            &self,
            cpp: &MyObjectQt,
            color: &cxx_qt_lib::QColor,
        ) -> UniquePtr<cxx_qt_lib::QColor> {
            return cpp.test_color(color);
        }

        pub fn test_date_wrapper(
            &self,
            cpp: &MyObjectQt,
            date: &cxx_qt_lib::QDate,
        ) -> cxx_qt_lib::QDate {
            return cpp.test_date(date);
        }

        pub fn test_date_time_wrapper(
            &self,
            cpp: &MyObjectQt,
            dateTime: &cxx_qt_lib::QDateTime,
        ) -> UniquePtr<cxx_qt_lib::QDateTime> {
            return cpp.test_date_time(dateTime);
        }

        pub fn test_point_wrapper(
            &self,
            cpp: &MyObjectQt,
            point: &cxx_qt_lib::QPoint,
        ) -> cxx_qt_lib::QPoint {
            return cpp.test_point(point);
        }

        pub fn test_pointf_wrapper(
            &self,
            cpp: &MyObjectQt,
            pointf: &cxx_qt_lib::QPointF,
        ) -> cxx_qt_lib::QPointF {
            return cpp.test_pointf(pointf);
        }

        pub fn test_rect_wrapper(
            &self,
            cpp: &MyObjectQt,
            rect: &cxx_qt_lib::QRect,
        ) -> cxx_qt_lib::QRect {
            return cpp.test_rect(rect);
        }

        pub fn test_rectf_wrapper(
            &self,
            cpp: &MyObjectQt,
            rectf: &cxx_qt_lib::QRectF,
        ) -> cxx_qt_lib::QRectF {
            return cpp.test_rectf(rectf);
        }

        pub fn test_size_wrapper(
            &self,
            cpp: &MyObjectQt,
            size: &cxx_qt_lib::QSize,
        ) -> cxx_qt_lib::QSize {
            return cpp.test_size(size);
        }

        pub fn test_sizef_wrapper(
            &self,
            cpp: &MyObjectQt,
            sizef: &cxx_qt_lib::QSizeF,
        ) -> cxx_qt_lib::QSizeF {
            return cpp.test_sizef(sizef);
        }

        pub fn test_string_wrapper(
            &self,
            cpp: &MyObjectQt,
            string: &cxx_qt_lib::QString,
        ) -> UniquePtr<cxx_qt_lib::QString> {
            return cpp.test_string(string);
        }

        pub fn test_time_wrapper(
            &self,
            cpp: &MyObjectQt,
            time: &cxx_qt_lib::QTime,
        ) -> cxx_qt_lib::QTime {
            return cpp.test_time(time);
        }

        pub fn test_url_wrapper(
            &self,
            cpp: &MyObjectQt,
            url: &cxx_qt_lib::QUrl,
        ) -> UniquePtr<cxx_qt_lib::QUrl> {
            return cpp.test_url(url);
        }

        pub fn test_variant_wrapper(
            &self,
            cpp: &MyObjectQt,
            variant: &cxx_qt_lib::QVariant,
        ) -> UniquePtr<cxx_qt_lib::QVariant> {
            return cpp.test_variant(variant);
        }
    }

    impl MyObjectQt {
        pub fn test_color(&self, color: &QColor) -> UniquePtr<QColor> {
            color
        }

        pub fn test_date(&self, date: &QDate) -> QDate {
            date
        }

        pub fn test_date_time(&self, dateTime: &QDateTime) -> UniquePtr<QDateTime> {
            dateTime
        }

        pub fn test_point(&self, point: &QPoint) -> QPoint {
            point
        }

        pub fn test_pointf(&self, pointf: &QPointF) -> QPointF {
            pointf
        }

        pub fn test_rect(&self, rect: &QRect) -> QRect {
            rect
        }

        pub fn test_rectf(&self, rectf: &QRectF) -> QRectF {
            rectf
        }

        pub fn test_size(&self, size: &QSize) -> QSize {
            size
        }

        pub fn test_sizef(&self, sizef: &QSizeF) -> QSizeF {
            sizef
        }

        pub fn test_string(&self, string: &QString) -> UniquePtr<QString> {
            string.to_owned()
        }

        pub fn test_time(&self, time: &QTime) -> QTime {
            time
        }

        pub fn test_url(&self, url: &QUrl) -> UniquePtr<QUrl> {
            url
        }

        pub fn test_variant(&self, variant: &QVariant) -> UniquePtr<QVariant> {
            variant
        }
    }

    unsafe impl Send for MyObjectCxxQtThread {}

    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
}
