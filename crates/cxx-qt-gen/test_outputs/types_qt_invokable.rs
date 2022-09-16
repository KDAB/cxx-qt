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
    }

    unsafe extern "C++" {
        #[cxx_name = "MyObject"]
        type MyObjectQt;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;
    }

    extern "Rust" {
        #[cxx_name = "testColorWrapper"]
        fn test_color_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            color: &QColor,
        ) -> UniquePtr<QColor>;
    }

    extern "Rust" {
        #[cxx_name = "testDateWrapper"]
        fn test_date_wrapper(self: &MyObject, cpp: &MyObjectQt, date: &QDate) -> QDate;
    }

    extern "Rust" {
        #[cxx_name = "testDateTimeWrapper"]
        fn test_date_time_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            dateTime: &QDateTime,
        ) -> UniquePtr<QDateTime>;
    }

    extern "Rust" {
        #[cxx_name = "testPointWrapper"]
        fn test_point_wrapper(self: &MyObject, cpp: &MyObjectQt, point: &QPoint) -> QPoint;
    }

    extern "Rust" {
        #[cxx_name = "testPointfWrapper"]
        fn test_pointf_wrapper(self: &MyObject, cpp: &MyObjectQt, pointf: &QPointF) -> QPointF;
    }

    extern "Rust" {
        #[cxx_name = "testRectWrapper"]
        fn test_rect_wrapper(self: &MyObject, cpp: &MyObjectQt, rect: &QRect) -> QRect;
    }

    extern "Rust" {
        #[cxx_name = "testRectfWrapper"]
        fn test_rectf_wrapper(self: &MyObject, cpp: &MyObjectQt, rectf: &QRectF) -> QRectF;
    }

    extern "Rust" {
        #[cxx_name = "testSizeWrapper"]
        fn test_size_wrapper(self: &MyObject, cpp: &MyObjectQt, size: &QSize) -> QSize;
    }

    extern "Rust" {
        #[cxx_name = "testSizefWrapper"]
        fn test_sizef_wrapper(self: &MyObject, cpp: &MyObjectQt, sizef: &QSizeF) -> QSizeF;
    }

    extern "Rust" {
        #[cxx_name = "testStringWrapper"]
        fn test_string_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            string: &QString,
        ) -> UniquePtr<QString>;
    }

    extern "Rust" {
        #[cxx_name = "testTimeWrapper"]
        fn test_time_wrapper(self: &MyObject, cpp: &MyObjectQt, time: &QTime) -> QTime;
    }

    extern "Rust" {
        #[cxx_name = "testUrlWrapper"]
        fn test_url_wrapper(self: &MyObject, cpp: &MyObjectQt, url: &QUrl) -> UniquePtr<QUrl>;
    }

    extern "Rust" {
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
    use std::pin::Pin;

    type UniquePtr<T> = cxx::UniquePtr<T>;

    #[derive(Default)]
    pub struct MyObject;

    impl MyObject {
        pub fn test_color_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            color: &QColor,
        ) -> UniquePtr<QColor> {
            return cpp.test_color(color);
        }
    }

    impl MyObjectQt {
        pub fn test_color(&self, color: &QColor) -> UniquePtr<QColor> {
            color
        }
    }

    impl MyObject {
        pub fn test_date_wrapper(self: &MyObject, cpp: &MyObjectQt, date: &QDate) -> QDate {
            return cpp.test_date(date);
        }
    }

    impl MyObjectQt {
        pub fn test_date(&self, date: &QDate) -> QDate {
            date
        }
    }

    impl MyObject {
        pub fn test_date_time_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            dateTime: &QDateTime,
        ) -> UniquePtr<QDateTime> {
            return cpp.test_date_time(dateTime);
        }
    }

    impl MyObjectQt {
        pub fn test_date_time(&self, dateTime: &QDateTime) -> UniquePtr<QDateTime> {
            dateTime
        }
    }

    impl MyObject {
        pub fn test_point_wrapper(self: &MyObject, cpp: &MyObjectQt, point: &QPoint) -> QPoint {
            return cpp.test_point(point);
        }
    }

    impl MyObjectQt {
        pub fn test_point(&self, point: &QPoint) -> QPoint {
            point
        }
    }

    impl MyObject {
        pub fn test_pointf_wrapper(self: &MyObject, cpp: &MyObjectQt, pointf: &QPointF) -> QPointF {
            return cpp.test_pointf(pointf);
        }
    }

    impl MyObjectQt {
        pub fn test_pointf(&self, pointf: &QPointF) -> QPointF {
            pointf
        }
    }

    impl MyObject {
        pub fn test_rect_wrapper(self: &MyObject, cpp: &MyObjectQt, rect: &QRect) -> QRect {
            return cpp.test_rect(rect);
        }
    }

    impl MyObjectQt {
        pub fn test_rect(&self, rect: &QRect) -> QRect {
            rect
        }
    }

    impl MyObject {
        pub fn test_rectf_wrapper(self: &MyObject, cpp: &MyObjectQt, rectf: &QRectF) -> QRectF {
            return cpp.test_rectf(rectf);
        }
    }

    impl MyObjectQt {
        pub fn test_rectf(&self, rectf: &QRectF) -> QRectF {
            rectf
        }
    }

    impl MyObject {
        pub fn test_size_wrapper(self: &MyObject, cpp: &MyObjectQt, size: &QSize) -> QSize {
            return cpp.test_size(size);
        }
    }

    impl MyObjectQt {
        pub fn test_size(&self, size: &QSize) -> QSize {
            size
        }
    }

    impl MyObject {
        pub fn test_sizef_wrapper(self: &MyObject, cpp: &MyObjectQt, sizef: &QSizeF) -> QSizeF {
            return cpp.test_sizef(sizef);
        }
    }

    impl MyObjectQt {
        pub fn test_sizef(&self, sizef: &QSizeF) -> QSizeF {
            sizef
        }
    }

    impl MyObject {
        pub fn test_string_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            string: &QString,
        ) -> UniquePtr<QString> {
            return cpp.test_string(string);
        }
    }

    impl MyObjectQt {
        pub fn test_string(&self, string: &QString) -> UniquePtr<QString> {
            string.to_owned()
        }
    }

    impl MyObject {
        pub fn test_time_wrapper(self: &MyObject, cpp: &MyObjectQt, time: &QTime) -> QTime {
            return cpp.test_time(time);
        }
    }
    impl MyObjectQt {
        pub fn test_time(&self, time: &QTime) -> QTime {
            time
        }
    }

    impl MyObject {
        pub fn test_url_wrapper(self: &MyObject, cpp: &MyObjectQt, url: &QUrl) -> UniquePtr<QUrl> {
            return cpp.test_url(url);
        }
    }

    impl MyObjectQt {
        pub fn test_url(&self, url: &QUrl) -> UniquePtr<QUrl> {
            url
        }
    }

    impl MyObject {
        pub fn test_variant_wrapper(
            self: &MyObject,
            cpp: &MyObjectQt,
            variant: &QVariant,
        ) -> UniquePtr<QVariant> {
            return cpp.test_variant(variant);
        }
    }

    impl MyObjectQt {
        pub fn test_variant(&self, variant: &QVariant) -> UniquePtr<QVariant> {
            variant
        }
    }

    unsafe impl Send for MyObjectCxxQtThread {}

    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
}
