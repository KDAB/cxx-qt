#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;

        type Opaque;
    }

    unsafe extern "C++Qt" {
        include!(<QtCore/QTimer>);
        /// QTimer
        #[qobject]
        type QTimer;

        /// When the QTimer timeout occurs
        #[qsignal]
        pub(self) fn timeout(self: Pin<&mut QTimer>);
    }

    unsafe extern "RustQt" {
        #[qobject]
        type MyObject = super::MyObjectRust;

        #[qsignal]
        fn ready(self: Pin<&mut MyObject>);

        #[qsignal]
        fn data_changed(
            self: Pin<&mut MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &QPoint,
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
