#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QColor = cxx_qt_lib::QColor;
        type QPoint = cxx_qt_lib::QPoint;
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject {
        #[qproperty]
        primitive: i32,
        #[qproperty]
        trivial: QPoint,
        #[qproperty]
        opaque: UniquePtr<QColor>,
    }
}
