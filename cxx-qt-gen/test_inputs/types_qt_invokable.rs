mod my_object {
    #[derive(Default)]
    struct Data;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn test_pointf(&self, _cpp: Pin<&mut CppObj>, pointf: &QPointF) -> QPointF {
            pointf
        }

        #[invokable]
        fn test_string(&self, _cpp: Pin<&mut CppObj>, string: &QString) -> String {
            string.to_rust()
        }
    }
}
