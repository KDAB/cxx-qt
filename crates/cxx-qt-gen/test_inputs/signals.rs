#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    #[cxx_qt::qsignals(MyObject)]
    enum MySignals<'a> {
        Ready,
        DataChanged {
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        },
        #[cxx_name = "newData"]
        #[inherit]
        BaseClassNewData {
            first: i32,
            second: UniquePtr<Opaque>,
            third: QPoint,
            fourth: &'a QPoint,
        },
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
            self.as_mut().emit(MySignals::DataChanged {
                first: 1,
                second: Opaque::new(),
                third: QPoint::new(1, 2),
                fourth: &QPoint::new(1, 2),
            });
        }
    }
}
