mod my_object {
    #[derive(Default)]
    struct Data;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn test_point(&self, _cpp: &mut CppObj, point: &QPoint) -> QPoint {
            point
        }

        #[invokable]
        fn test_pointf(&self, _cpp: &mut CppObj, pointf: &QPointF) -> QPointF {
            pointf
        }

        #[invokable]
        fn test_size(&self, _cpp: &mut CppObj, size: &QSize) -> QSize {
            size
        }

        #[invokable]
        fn test_sizef(&self, _cpp: &mut CppObj, sizef: &QSizeF) -> QSizeF {
            sizef
        }

        #[invokable]
        fn test_string(&self, _cpp: &mut CppObj, string: &str) -> String {
            string.to_owned()
        }

        #[invokable]
        fn test_variant(&self, _cpp: &mut CppObj, variant: &Variant) -> Variant {
            variant
        }
    }
}
