mod my_object {
    use cxx_qt_lib::QVariant;

    enum Signal {
        Ready,
        DataChanged {
            first: i32,
            second: QVariant,
            third: QPoint,
        },
    }

    #[derive(Default)]
    struct Data;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn invokable(&self, cpp: &mut CppObj) {
            unsafe {
                cpp.emit_immediate(Signal::Ready);
            }

            cpp.emit_queued(Signal::DataChanged {
                first: 1,
                second: QVariant::from_bool(true),
                third: QPoint::new(1, 2),
            });
        }
    }
}
