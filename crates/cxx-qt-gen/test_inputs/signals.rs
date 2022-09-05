#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QPoint = cxx_qt_lib::QPoint;
        type QVariant = cxx_qt_lib::QVariant;
    }

    #[cxx_qt::signals(MyObject)]
    enum MySignals {
        Ready,
        DataChanged {
            first: i32,
            #[cxx_type = "QVariant"]
            second: UniquePtr<QVariant>,
            third: QPoint,
        },
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject;

    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn invokable(self: Pin<&mut Self>) {
            unsafe {
                self.as_mut().emit_immediate(MySignals::Ready);
            }

            self.as_mut().emit_queued(MySignals::DataChanged {
                first: 1,
                second: QVariant::from_bool(true),
                third: QPoint::new(1, 2),
            });
        }
    }
}
