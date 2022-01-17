mod my_object {
    use cxx_qt_lib::QString;

    #[derive(Default)]
    struct Data;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn test_color(&self, _cpp: &mut CppObj, color: &QColor) -> Color {
            color
        }

        #[invokable]
        fn test_date(&self, _cpp: &mut CppObj, date: &QDate) -> QDate {
            date
        }

        #[invokable]
        fn test_point(&self, _cpp: &mut CppObj, point: &QPoint) -> QPoint {
            point
        }

        #[invokable]
        fn test_pointf(&self, _cpp: &mut CppObj, pointf: &QPointF) -> QPointF {
            pointf
        }

        #[invokable]
        fn test_rect(&self, _cpp: &mut CppObj, rect: &QRect) -> QRect {
            rect
        }

        #[invokable]
        fn test_rectf(&self, _cpp: &mut CppObj, rectf: &QRectF) -> QRectF {
            rectf
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
        fn test_string(&self, _cpp: &mut CppObj, string: &QString) -> String {
            string.to_rust()
        }

        #[invokable]
        fn test_time(&self, _cpp: &mut CppObj, time: &QTime) -> QTime {
            time
        }

        #[invokable]
        fn test_variant(&self, _cpp: &mut CppObj, variant: &QVariant) -> Variant {
            variant
        }
    }
}
