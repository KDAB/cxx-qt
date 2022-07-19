mod my_object {
    use cxx_qt_lib::ToUniquePtr;

    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.cxxqt.h");

            type MyObject;

            include!("cxx-qt-lib/include/qt_types.h");
            #[namespace = ""]
            type QColor = cxx_qt_lib::QColorCpp;
            #[namespace = ""]
            type QDate = cxx_qt_lib::QDate;
            #[namespace = ""]
            type QDateTime = cxx_qt_lib::QDateTimeCpp;
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
            type QString = cxx_qt_lib::QStringCpp;
            #[namespace = ""]
            type QTime = cxx_qt_lib::QTime;
            #[namespace = ""]
            type QUrl = cxx_qt_lib::QUrlCpp;
            #[namespace = ""]
            type QVariant = cxx_qt_lib::QVariantCpp;

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

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {}

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn color(&self) -> cxx_qt_lib::QColor {
            self.cpp.color().to_rust()
        }

        pub fn set_color(&mut self, value: cxx_qt_lib::QColor) {
            self.cpp.as_mut().set_color(&value.to_unique_ptr());
        }

        pub fn date(&self) -> &cxx_qt_lib::QDate {
            self.cpp.date()
        }

        pub fn set_date(&mut self, value: &cxx_qt_lib::QDate) {
            self.cpp.as_mut().set_date(value);
        }

        pub fn date_time(&self) -> cxx_qt_lib::QDateTime {
            self.cpp.date_time().to_rust()
        }

        pub fn set_date_time(&mut self, value: cxx_qt_lib::QDateTime) {
            self.cpp.as_mut().set_date_time(&value.to_unique_ptr());
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

        pub fn set_string(&mut self, value: &str) {
            self.cpp.as_mut().set_string(&value.to_unique_ptr());
        }

        pub fn time(&self) -> &cxx_qt_lib::QTime {
            self.cpp.time()
        }

        pub fn set_time(&mut self, value: &cxx_qt_lib::QTime) {
            self.cpp.as_mut().set_time(value);
        }

        pub fn url(&self) -> cxx_qt_lib::QUrl {
            self.cpp.url().to_rust()
        }

        pub fn set_url(&mut self, value: cxx_qt_lib::QUrl) {
            self.cpp.as_mut().set_url(&value.to_unique_ptr());
        }

        pub fn variant(&self) -> cxx_qt_lib::QVariant {
            self.cpp.variant().to_rust()
        }

        pub fn set_variant(&mut self, value: cxx_qt_lib::QVariant) {
            self.cpp.as_mut().set_variant(&value.to_unique_ptr());
        }

        pub fn grab_values_from_data(&mut self, mut data: Data) {
            self.set_color(std::mem::take(&mut data.color));
            self.set_date(&data.date);
            self.set_date_time(std::mem::take(&mut data.date_time));
            self.set_point(&data.point);
            self.set_pointf(&data.pointf);
            self.set_rect(&data.rect);
            self.set_rectf(&data.rectf);
            self.set_size(&data.size);
            self.set_sizef(&data.sizef);
            self.set_string(&data.string);
            self.set_time(&data.time);
            self.set_url(std::mem::take(&mut data.url));
            self.set_variant(std::mem::take(&mut data.variant));
        }
    }

    #[derive(Default)]
    pub struct Data {
        color: QColor,
        date: QDate,
        date_time: QDateTime,
        point: QPoint,
        pointf: QPointF,
        rect: QRect,
        rectf: QRectF,
        size: QSize,
        sizef: QSizeF,
        string: String,
        time: QTime,
        url: QUrl,
        variant: QVariant,
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

    pub fn create_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(Data::default());
    }
}
