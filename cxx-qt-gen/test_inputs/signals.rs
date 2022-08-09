#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QPoint = cxx_qt_lib::QPoint;
        type QVariant = cxx_qt_lib::QVariant;
    }

    #[cxx_qt::signals(RustObj)]
    enum MySignals {
        Ready,
        DataChanged {
            first: i32,
            second: UniquePtr<QVariant>,
            third: QPoint,
        },
    }

    #[derive(Default)]
    pub struct Data;

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct RustObj;

    impl cxx_qt::QObject<RustObj> {
        #[invokable]
        pub fn invokable(&self, cpp: &mut CppObj) {
            unsafe {
                cpp.emit_immediate(MySignals::Ready);
            }

            cpp.emit_queued(MySignals::DataChanged {
                first: 1,
                second: QVariant::from_bool(true),
                third: QPoint::new(1, 2),
            });
        }
    }
}
