#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    unsafe extern "RustQt" {
        #[qobject]
        type MyObject = super::MyObjectRust;

        #[qsignal]
        fn dataReady(self: Pin<&mut MyObject>);

        #[qsignal]
        #[rust_name = "data_changed"]
        fn dataChanged(
            self: Pin<&mut MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        );

        #[cxx_name = "newData"]
        #[inherit]
        #[qsignal]
        fn base_class_new_data(
            self: Pin<&mut MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        );

        #[qinvokable]
        fn invokable(self: Pin<&mut MyObject>);
    }
}
