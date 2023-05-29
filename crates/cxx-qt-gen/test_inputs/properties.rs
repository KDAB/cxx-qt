#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject {
        #[qproperty]
        primitive: i32,
        #[qproperty]
        trivial: QPoint,
        opaque: UniquePtr<Opaque>,

        private_rust_field: i32,
        pub public_rust_field: f64,
    }
}
