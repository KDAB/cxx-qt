#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
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

    #[derive(Default)]
    pub struct Data;

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject;

    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn test_color(&self, _cpp: &mut CppObj, color: &QColor) -> UniquePtr<QColor> {
            color
        }

        #[qinvokable]
        pub fn test_date(&self, _cpp: &mut CppObj, date: &QDate) -> QDate {
            date
        }

        #[qinvokable]
        pub fn test_date_time(
            &self,
            _cpp: &mut CppObj,
            dateTime: &QDateTime,
        ) -> UniquePtr<QDateTime> {
            dateTime
        }

        #[qinvokable]
        pub fn test_point(&self, _cpp: &mut CppObj, point: &QPoint) -> QPoint {
            point
        }

        #[qinvokable]
        pub fn test_pointf(&self, _cpp: &mut CppObj, pointf: &QPointF) -> QPointF {
            pointf
        }

        #[qinvokable]
        pub fn test_rect(&self, _cpp: &mut CppObj, rect: &QRect) -> QRect {
            rect
        }

        #[qinvokable]
        pub fn test_rectf(&self, _cpp: &mut CppObj, rectf: &QRectF) -> QRectF {
            rectf
        }

        #[qinvokable]
        pub fn test_size(&self, _cpp: &mut CppObj, size: &QSize) -> QSize {
            size
        }

        #[qinvokable]
        pub fn test_sizef(&self, _cpp: &mut CppObj, sizef: &QSizeF) -> QSizeF {
            sizef
        }

        #[qinvokable]
        pub fn test_string(&self, _cpp: &mut CppObj, string: &QString) -> UniquePtr<QString> {
            string.to_owned()
        }

        #[qinvokable]
        pub fn test_time(&self, _cpp: &mut CppObj, time: &QTime) -> QTime {
            time
        }

        #[qinvokable]
        pub fn test_url(&self, _cpp: &mut CppObj, url: &QUrl) -> UniquePtr<QUrl> {
            url
        }

        #[qinvokable]
        pub fn test_variant(&self, _cpp: &mut CppObj, variant: &QVariant) -> UniquePtr<QVariant> {
            variant
        }
    }
}
