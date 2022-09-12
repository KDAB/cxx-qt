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

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject;

    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn test_color(&self, color: &QColor) -> UniquePtr<QColor> {
            color
        }

        #[qinvokable]
        pub fn test_date(&self, date: &QDate) -> QDate {
            date
        }

        #[qinvokable]
        pub fn test_date_time(&self, dateTime: &QDateTime) -> UniquePtr<QDateTime> {
            dateTime
        }

        #[qinvokable]
        pub fn test_point(&self, point: &QPoint) -> QPoint {
            point
        }

        #[qinvokable]
        pub fn test_pointf(&self, pointf: &QPointF) -> QPointF {
            pointf
        }

        #[qinvokable]
        pub fn test_rect(&self, rect: &QRect) -> QRect {
            rect
        }

        #[qinvokable]
        pub fn test_rectf(&self, rectf: &QRectF) -> QRectF {
            rectf
        }

        #[qinvokable]
        pub fn test_size(&self, size: &QSize) -> QSize {
            size
        }

        #[qinvokable]
        pub fn test_sizef(&self, sizef: &QSizeF) -> QSizeF {
            sizef
        }

        #[qinvokable]
        pub fn test_string(&self, string: &QString) -> UniquePtr<QString> {
            string.to_owned()
        }

        #[qinvokable]
        pub fn test_time(&self, time: &QTime) -> QTime {
            time
        }

        #[qinvokable]
        pub fn test_url(&self, url: &QUrl) -> UniquePtr<QUrl> {
            url
        }

        #[qinvokable]
        pub fn test_variant(&self, variant: &QVariant) -> UniquePtr<QVariant> {
            variant
        }
    }
}
