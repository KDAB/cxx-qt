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
    pub struct MyObject {
        #[qproperty(cxx_type = "QColor")]
        color: UniquePtr<QColor>,
        #[qproperty]
        date: QDate,
        #[qproperty(cxx_type = "QDateTime")]
        date_time: UniquePtr<QDateTime>,
        #[qproperty]
        point: QPoint,
        #[qproperty]
        pointf: QPointF,
        #[qproperty]
        rect: QRect,
        #[qproperty]
        rectf: QRectF,
        #[qproperty]
        size: QSize,
        #[qproperty]
        sizef: QSizeF,
        #[qproperty(cxx_type = "QString")]
        string: UniquePtr<QString>,
        #[qproperty]
        time: QTime,
        #[qproperty(cxx_type = "QUrl")]
        url: UniquePtr<QUrl>,
        #[qproperty(cxx_type = "QVariant")]
        variant: UniquePtr<QVariant>,
    }
}
