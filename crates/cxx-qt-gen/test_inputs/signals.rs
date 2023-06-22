#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    unsafe extern "RustQt" {
        #[qsignal]
        fn ready(self: Pin<&mut qobject::MyObject>);

        #[qsignal]
        fn data_changed(
            self: Pin<&mut qobject::MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        );

        #[cxx_name = "newData"]
        #[inherit]
        #[qsignal]
        fn base_class_new_data(
            self: Pin<&mut qobject::MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        );

        #[qinvokable]
        fn invokable(self: Pin<&mut qobject::MyObject>);
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject;
}
