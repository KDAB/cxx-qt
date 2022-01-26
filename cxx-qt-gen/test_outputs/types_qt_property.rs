mod my_object {
    use cxx_qt_lib::ToUniquePtr;

    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        enum Property {
            Color,
            Date,
            DateTime,
            Point,
            Pointf,
            Rect,
            Rectf,
            Size,
            Sizef,
            String,
            Time,
            Url,
            Variant,
        }

        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;

            include!("cxx-qt-lib/include/qt_types.h");
            #[namespace = ""]
            type QColor = cxx_qt_lib::QColor;
            #[namespace = ""]
            type QDate = cxx_qt_lib::QDate;
            #[namespace = ""]
            type QDateTime = cxx_qt_lib::QDateTime;
            #[namespace = ""]
            type QPoint = cxx_qt_lib::QPoint;
            #[namespace = ""]
            type QPointF = cxx_qt_lib::QPointF;
            #[namespace = ""]
            type QRect = cxx_qt_lib::QRect;
            #[namespace = ""]
            type QRectF = cxx_qt_lib::QRectF;
            #[namespace = ""]
            type QSize = cxx_qt_lib::QSize;
            #[namespace = ""]
            type QSizeF = cxx_qt_lib::QSizeF;
            #[namespace = ""]
            type QString = cxx_qt_lib::QString;
            #[namespace = ""]
            type QTime = cxx_qt_lib::QTime;
            #[namespace = ""]
            type QUrl = cxx_qt_lib::QUrl;
            #[namespace = ""]
            type QVariant = cxx_qt_lib::QVariant;

            #[rust_name = "color"]
            fn getColor(self: &MyObject) -> &QColor;
            #[rust_name = "set_color"]
            fn setColor(self: Pin<&mut MyObject>, value: &QColor);

            #[rust_name = "date"]
            fn getDate(self: &MyObject) -> &QDate;
            #[rust_name = "set_date"]
            fn setDate(self: Pin<&mut MyObject>, value: &QDate);

            #[rust_name = "date_time"]
            fn getDateTime(self: &MyObject) -> &QDateTime;
            #[rust_name = "set_date_time"]
            fn setDateTime(self: Pin<&mut MyObject>, value: &QDateTime);

            #[rust_name = "point"]
            fn getPoint(self: &MyObject) -> &QPoint;
            #[rust_name = "set_point"]
            fn setPoint(self: Pin<&mut MyObject>, value: &QPoint);

            #[rust_name = "pointf"]
            fn getPointf(self: &MyObject) -> &QPointF;
            #[rust_name = "set_pointf"]
            fn setPointf(self: Pin<&mut MyObject>, value: &QPointF);

            #[rust_name = "rect"]
            fn getRect(self: &MyObject) -> &QRect;
            #[rust_name = "set_rect"]
            fn setRect(self: Pin<&mut MyObject>, value: &QRect);

            #[rust_name = "rectf"]
            fn getRectf(self: &MyObject) -> &QRectF;
            #[rust_name = "set_rectf"]
            fn setRectf(self: Pin<&mut MyObject>, value: &QRectF);

            #[rust_name = "size"]
            fn getSize(self: &MyObject) -> &QSize;
            #[rust_name = "set_size"]
            fn setSize(self: Pin<&mut MyObject>, value: &QSize);

            #[rust_name = "sizef"]
            fn getSizef(self: &MyObject) -> &QSizeF;
            #[rust_name = "set_sizef"]
            fn setSizef(self: Pin<&mut MyObject>, value: &QSizeF);

            #[rust_name = "string"]
            fn getString(self: &MyObject) -> &QString;
            #[rust_name = "set_string"]
            fn setString(self: Pin<&mut MyObject>, value: &QString);

            #[rust_name = "time"]
            fn getTime(self: &MyObject) -> &QTime;
            #[rust_name = "set_time"]
            fn setTime(self: Pin<&mut MyObject>, value: &QTime);

            #[rust_name = "url"]
            fn getUrl(self: &MyObject) -> &QUrl;
            #[rust_name = "set_url"]
            fn setUrl(self: Pin<&mut MyObject>, value: &QUrl);

            #[rust_name = "variant"]
            fn getVariant(self: &MyObject) -> &QVariant;
            #[rust_name = "set_variant"]
            fn setVariant(self: Pin<&mut MyObject>, value: &QVariant);

            #[rust_name = "new_cpp_object"]
            fn newCppObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "createRs"]
            fn create_rs() -> Box<RustObj>;

            #[cxx_name = "initialiseCpp"]
            fn initialise_cpp(cpp: Pin<&mut MyObject>);
        }
    }

    pub type FFICppObj = ffi::MyObject;
    pub type Property = ffi::Property;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {}

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn color(&self) -> cxx_qt_lib::Color {
            self.cpp.color().to_rust()
        }

        pub fn set_color(&mut self, value: &cxx_qt_lib::QColor) {
            self.cpp.as_mut().set_color(value);
        }

        pub fn date(&self) -> &cxx_qt_lib::QDate {
            self.cpp.date()
        }

        pub fn set_date(&mut self, value: &cxx_qt_lib::QDate) {
            self.cpp.as_mut().set_date(value);
        }

        pub fn date_time(&self) -> cxx_qt_lib::DateTime {
            self.cpp.date_time().to_rust()
        }

        pub fn set_date_time(&mut self, value: &cxx_qt_lib::QDateTime) {
            self.cpp.as_mut().set_date_time(value);
        }

        pub fn point(&self) -> &cxx_qt_lib::QPoint {
            self.cpp.point()
        }

        pub fn set_point(&mut self, value: &cxx_qt_lib::QPoint) {
            self.cpp.as_mut().set_point(value);
        }

        pub fn pointf(&self) -> &cxx_qt_lib::QPointF {
            self.cpp.pointf()
        }

        pub fn set_pointf(&mut self, value: &cxx_qt_lib::QPointF) {
            self.cpp.as_mut().set_pointf(value);
        }

        pub fn rect(&self) -> &cxx_qt_lib::QRect {
            self.cpp.rect()
        }

        pub fn set_rect(&mut self, value: &cxx_qt_lib::QRect) {
            self.cpp.as_mut().set_rect(value);
        }

        pub fn rectf(&self) -> &cxx_qt_lib::QRectF {
            self.cpp.rectf()
        }

        pub fn set_rectf(&mut self, value: &cxx_qt_lib::QRectF) {
            self.cpp.as_mut().set_rectf(value);
        }

        pub fn size(&self) -> &cxx_qt_lib::QSize {
            self.cpp.size()
        }

        pub fn set_size(&mut self, value: &cxx_qt_lib::QSize) {
            self.cpp.as_mut().set_size(value);
        }

        pub fn sizef(&self) -> &cxx_qt_lib::QSizeF {
            self.cpp.sizef()
        }

        pub fn set_sizef(&mut self, value: &cxx_qt_lib::QSizeF) {
            self.cpp.as_mut().set_sizef(value);
        }

        pub fn string(&self) -> String {
            self.cpp.string().to_rust()
        }

        pub fn set_string(&mut self, value: &cxx_qt_lib::QString) {
            self.cpp.as_mut().set_string(value);
        }

        pub fn time(&self) -> &cxx_qt_lib::QTime {
            self.cpp.time()
        }

        pub fn set_time(&mut self, value: &cxx_qt_lib::QTime) {
            self.cpp.as_mut().set_time(value);
        }

        pub fn url(&self) -> cxx_qt_lib::Url {
            self.cpp.url().to_rust()
        }

        pub fn set_url(&mut self, value: &cxx_qt_lib::QUrl) {
            self.cpp.as_mut().set_url(value);
        }

        pub fn variant(&self) -> cxx_qt_lib::Variant {
            self.cpp.variant().to_rust()
        }

        pub fn set_variant(&mut self, value: &cxx_qt_lib::QVariant) {
            self.cpp.as_mut().set_variant(value);
        }

        pub fn grab_values_from_data(&mut self, data: &Data) {
            use cxx_qt_lib::MapQtValue;

            data.color
                .map_qt_value(|context, converted| context.set_color(converted), self);
            data.date
                .map_qt_value(|context, converted| context.set_date(converted), self);
            data.date_time
                .map_qt_value(|context, converted| context.set_date_time(converted), self);
            data.point
                .map_qt_value(|context, converted| context.set_point(converted), self);
            data.pointf
                .map_qt_value(|context, converted| context.set_pointf(converted), self);
            data.rect
                .map_qt_value(|context, converted| context.set_rect(converted), self);
            data.rectf
                .map_qt_value(|context, converted| context.set_rectf(converted), self);
            data.size
                .map_qt_value(|context, converted| context.set_size(converted), self);
            data.sizef
                .map_qt_value(|context, converted| context.set_sizef(converted), self);
            data.string
                .map_qt_value(|context, converted| context.set_string(converted), self);
            data.time
                .map_qt_value(|context, converted| context.set_time(converted), self);
            data.url
                .map_qt_value(|context, converted| context.set_url(converted), self);
            data.variant
                .map_qt_value(|context, converted| context.set_variant(converted), self);
        }
    }

    #[derive(Default)]
    struct Data {
        color: Color,
        date: QDate,
        date_time: DateTime,
        point: QPoint,
        pointf: QPointF,
        rect: QRect,
        rectf: QRectF,
        size: QSize,
        sizef: QSizeF,
        string: String,
        time: QTime,
        url: Url,
        variant: Variant,
    }

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(value: &CppObj<'a>) -> Self {
            Self {
                color: value.color().into(),
                date: value.date().into(),
                date_time: value.date_time().into(),
                point: value.point().into(),
                pointf: value.pointf().into(),
                rect: value.rect().into(),
                rectf: value.rectf().into(),
                size: value.size().into(),
                sizef: value.sizef().into(),
                string: value.string().into(),
                time: value.time().into(),
                url: value.url().into(),
                variant: value.variant().into(),
            }
        }
    }

    impl<'a> From<&mut CppObj<'a>> for Data {
        fn from(value: &mut CppObj<'a>) -> Self {
            Self::from(&*value)
        }
    }

    fn create_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(&Data::default());
    }
}
