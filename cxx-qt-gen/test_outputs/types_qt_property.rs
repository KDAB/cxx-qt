#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "color"]
        fn getColor(self: &MyObjectQt) -> &QColor;
        #[rust_name = "set_color"]
        fn setColor(self: Pin<&mut MyObjectQt>, value: &QColor);

        #[rust_name = "date"]
        fn getDate(self: &MyObjectQt) -> &QDate;
        #[rust_name = "set_date"]
        fn setDate(self: Pin<&mut MyObjectQt>, value: &QDate);

        #[rust_name = "date_time"]
        fn getDateTime(self: &MyObjectQt) -> &QDateTime;
        #[rust_name = "set_date_time"]
        fn setDateTime(self: Pin<&mut MyObjectQt>, value: &QDateTime);

        #[rust_name = "point"]
        fn getPoint(self: &MyObjectQt) -> &QPoint;
        #[rust_name = "set_point"]
        fn setPoint(self: Pin<&mut MyObjectQt>, value: &QPoint);

        #[rust_name = "pointf"]
        fn getPointf(self: &MyObjectQt) -> &QPointF;
        #[rust_name = "set_pointf"]
        fn setPointf(self: Pin<&mut MyObjectQt>, value: &QPointF);

        #[rust_name = "rect"]
        fn getRect(self: &MyObjectQt) -> &QRect;
        #[rust_name = "set_rect"]
        fn setRect(self: Pin<&mut MyObjectQt>, value: &QRect);

        #[rust_name = "rectf"]
        fn getRectf(self: &MyObjectQt) -> &QRectF;
        #[rust_name = "set_rectf"]
        fn setRectf(self: Pin<&mut MyObjectQt>, value: &QRectF);

        #[rust_name = "size"]
        fn getSize(self: &MyObjectQt) -> &QSize;
        #[rust_name = "set_size"]
        fn setSize(self: Pin<&mut MyObjectQt>, value: &QSize);

        #[rust_name = "sizef"]
        fn getSizef(self: &MyObjectQt) -> &QSizeF;
        #[rust_name = "set_sizef"]
        fn setSizef(self: Pin<&mut MyObjectQt>, value: &QSizeF);

        #[rust_name = "string"]
        fn getString(self: &MyObjectQt) -> &QString;
        #[rust_name = "set_string"]
        fn setString(self: Pin<&mut MyObjectQt>, value: &QString);

        #[rust_name = "time"]
        fn getTime(self: &MyObjectQt) -> &QTime;
        #[rust_name = "set_time"]
        fn setTime(self: Pin<&mut MyObjectQt>, value: &QTime);

        #[rust_name = "url"]
        fn getUrl(self: &MyObjectQt) -> &QUrl;
        #[rust_name = "set_url"]
        fn setUrl(self: Pin<&mut MyObjectQt>, value: &QUrl);

        #[rust_name = "variant"]
        fn getVariant(self: &MyObjectQt) -> &QVariant;
        #[rust_name = "set_variant"]
        fn setVariant(self: Pin<&mut MyObjectQt>, value: &QVariant);

        #[cxx_name = "unsafe_rust"]
        fn rust(self: &MyObjectQt) -> &RustObj;
        #[rust_name = "new_cpp_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafe_rust_mut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut RustObj>;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type RustObj;

        #[cxx_name = "createRs"]
        fn create_rs() -> Box<RustObj>;

        #[cxx_name = "initialiseCpp"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QColor = cxx_qt_lib::QColor;
        type QDate = cxx_qt_lib::QDate;
        type QDateTime = cxx_qt_lib::QDateTime;
        type QPoint = cxx_qt_lib::QPoint;
        type QPointF = cxx_qt_lib::QPointF;
        type QRect = cxx_qt_lib::QRect;
        type QRectF = cxx_qt_lib::QRectF;
        type QSize = cxx_qt_lib::QSize;
        type QSizeF = cxx_qt_lib::QSizeF;
        type QString = cxx_qt_lib::QString;
        type QTime = cxx_qt_lib::QTime;
        type QUrl = cxx_qt_lib::QUrl;
        type QVariant = cxx_qt_lib::QVariant;
    }
}

pub use self::cxx_qt_my_object::*;
mod cxx_qt_my_object {
    use super::my_object::*;

    pub type FFICppObj = super::my_object::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

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

        pub fn color(&self) -> &cxx_qt_lib::QColor {
            self.cpp.color()
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

        pub fn date_time(&self) -> &cxx_qt_lib::QDateTime {
            self.cpp.date_time()
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

        pub fn string(&self) -> &cxx_qt_lib::QString {
            self.cpp.string()
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

        pub fn url(&self) -> &cxx_qt_lib::QUrl {
            self.cpp.url()
        }

        pub fn set_url(&mut self, value: &cxx_qt_lib::QUrl) {
            self.cpp.as_mut().set_url(value);
        }

        pub fn variant(&self) -> &cxx_qt_lib::QVariant {
            self.cpp.variant()
        }

        pub fn set_variant(&mut self, value: &cxx_qt_lib::QVariant) {
            self.cpp.as_mut().set_variant(value);
        }

        pub fn grab_values_from_data(&mut self, mut data: Data) {
            self.set_color(data.color.as_ref().unwrap());
            self.set_date(&data.date);
            self.set_date_time(data.date_time.as_ref().unwrap());
            self.set_point(&data.point);
            self.set_pointf(&data.pointf);
            self.set_rect(&data.rect);
            self.set_rectf(&data.rectf);
            self.set_size(&data.size);
            self.set_sizef(&data.sizef);
            self.set_string(data.string.as_ref().unwrap());
            self.set_time(&data.time);
            self.set_url(data.url.as_ref().unwrap());
            self.set_variant(data.variant.as_ref().unwrap());
        }
    }

    #[derive(Default)]
    pub struct Data {
        color: UniquePtr<QColor>,
        date: QDate,
        date_time: UniquePtr<QDateTime>,
        point: QPoint,
        pointf: QPointF,
        rect: QRect,
        rectf: QRectF,
        size: QSize,
        sizef: QSizeF,
        string: UniquePtr<QString>,
        time: QTime,
        url: UniquePtr<QUrl>,
        variant: UniquePtr<QVariant>,
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
