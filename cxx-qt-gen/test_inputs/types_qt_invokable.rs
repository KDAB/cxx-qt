mod my_object {
    use cxx_qt_lib::QString;

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
        fn test_sizef(&self, _cpp: &mut CppObj, sizef: &QSizeF) -> QSizeF {
            sizef
        }

        #[invokable]
        fn test_string(&self, _cpp: &mut CppObj, string: &QString) -> String {
            string.to_rust()
        }

        #[invokable]
        fn test_variant(&self, _cpp: &mut CppObj, variant: &QVariant) -> Variant {
            variant
        }
    }
}
