#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    extern "RustQt" {
        #[cxx_qt::qobject]
        #[derive(Default)]
        #[qproperty(i32, primitive)]
        #[qproperty(QPoint, trivial)]
        type MyObject = super::MyObjectRust;
    }
}
