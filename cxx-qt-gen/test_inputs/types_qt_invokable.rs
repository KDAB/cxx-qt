mod my_object {
    #[derive(Default)]
    pub struct Data;

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        #[invokable]
        pub fn test_color(&self, _cpp: &mut CppObj, color: &QColor) -> QColor {
            color
        }

        #[invokable]
        pub fn test_date(&self, _cpp: &mut CppObj, date: &QDate) -> QDate {
            date
        }

        #[invokable]
        pub fn test_date_time(&self, _cpp: &mut CppObj, dateTime: &QDateTime) -> QDateTime {
            dateTime
        }

        #[invokable]
        pub fn test_point(&self, _cpp: &mut CppObj, point: &QPoint) -> QPoint {
            point
        }

        #[invokable]
        pub fn test_pointf(&self, _cpp: &mut CppObj, pointf: &QPointF) -> QPointF {
            pointf
        }

        #[invokable]
        pub fn test_rect(&self, _cpp: &mut CppObj, rect: &QRect) -> QRect {
            rect
        }

        #[invokable]
        pub fn test_rectf(&self, _cpp: &mut CppObj, rectf: &QRectF) -> QRectF {
            rectf
        }

        #[invokable]
        pub fn test_size(&self, _cpp: &mut CppObj, size: &QSize) -> QSize {
            size
        }

        #[invokable]
        pub fn test_sizef(&self, _cpp: &mut CppObj, sizef: &QSizeF) -> QSizeF {
            sizef
        }

        #[invokable]
        pub fn test_string(&self, _cpp: &mut CppObj, string: &str) -> String {
            string.to_owned()
        }

        #[invokable]
        pub fn test_time(&self, _cpp: &mut CppObj, time: &QTime) -> QTime {
            time
        }

        #[invokable]
        pub fn test_url(&self, _cpp: &mut CppObj, url: &QUrl) -> QUrl {
            url
        }

        #[invokable]
        pub fn test_variant(&self, _cpp: &mut CppObj, variant: &QVariant) -> QVariant {
            variant
        }
    }
}
