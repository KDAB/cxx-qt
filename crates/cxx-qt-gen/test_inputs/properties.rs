#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject {
        #[qproperty]
        primitive: i32,
        #[qproperty]
        trivial: QPoint,
        // Value and Opaque are not real types that would compile; these are only testing the code generation
        #[qproperty(cxx_type = "Value")]
        opaque: UniquePtr<Opaque>,

        private_rust_field: i32,
        pub public_rust_field: f64,
    }
}
