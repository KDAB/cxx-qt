#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    #[cxx_qt::qsignals]
    unsafe extern "C++" {
        fn ready(self: Pin<&mut qobject::MyObject>);

        fn data_changed(
            self: Pin<&mut qobject::MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        );

        #[cxx_name = "newData"]
        #[inherit]
        fn base_class_new_data(
            self: Pin<&mut qobject::MyObject>,
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        );
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject;

    impl qobject::MyObject {
        #[qinvokable]
        pub fn invokable(self: Pin<&mut Self>) {
            self.as_mut().on_data_changed(
                |_sender, _first, _second, _third, _fourth| {
                    println!("DataChanged");
                },
                cxx_qt_lib::ConnectionType::AutoConnection,
            );
            self.as_mut()
                .data_changed(1, Opaque::new(), QPoint::new(1, 2), &QPoint::new(1, 2));
        }
    }
}
