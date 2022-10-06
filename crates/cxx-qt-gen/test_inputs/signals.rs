#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    #[cxx_qt::signals(MyObject)]
    enum MySignals {
        Ready,
        DataChanged {
            first: i32,
            // Value and Opaque are not real types that would compile; these are only testing the code generation
            #[cxx_type = "Value"]
            second: UniquePtr<Opaque>,
            third: QPoint,
        },
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject;

    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn invokable(self: Pin<&mut Self>) {
            self.as_mut().emit_queued(MySignals::DataChanged {
                first: 1,
                second: Opaque::new(),
                third: QPoint::new(1, 2),
            });
        }
    }
}
