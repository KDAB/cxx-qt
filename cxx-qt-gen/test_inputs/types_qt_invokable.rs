mod my_object {
    #[derive(Default)]
    struct Data;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn test_pointf(&self, _cpp: Pin<&mut FFICppObj>, pointf: &QPointF) -> QPointF {
            pointf
        }

        #[invokable]
        fn test_string(&self, _cpp: Pin<&mut FFICppObj>, string: &QString) -> String {
            string.to_rust()
        }

        #[invokable]
        fn test_variant(&self, _cpp: Pin<&mut FFICppObj>, variant: &QVariant) -> Variant {
            variant
        }
    }
}
