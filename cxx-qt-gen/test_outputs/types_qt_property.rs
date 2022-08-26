#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
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
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;
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

    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/include/convert.h");
        include!("cxx-qt-lib/include/cxxqt_thread.h");

        type MyObjectCxxQtThread;

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObjectQt) -> UniquePtr<MyObjectCxxQtThread>;
        fn queue(self: &MyObjectCxxQtThread, func: fn(ctx: Pin<&mut MyObjectQt>)) -> Result<()>;

        #[rust_name = "new_cpp_object"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }

    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn create_rs() -> Box<MyObject>;

        #[cxx_name = "initialiseCpp"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
    }
}

pub use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;

    pub type FFICppObj = super::ffi::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    unsafe impl Send for MyObjectCxxQtThread {}

    use std::pin::Pin;

    #[derive(Default)]
    pub struct MyObject;

    impl MyObject {}

    impl MyObjectQt {
        pub fn grab_values_from_data(mut self: Pin<&mut Self>, mut data: Data) {
            self.as_mut().set_color(data.color.as_ref().unwrap());
            self.as_mut().set_date(&data.date);
            self.as_mut()
                .set_date_time(data.date_time.as_ref().unwrap());
            self.as_mut().set_point(&data.point);
            self.as_mut().set_pointf(&data.pointf);
            self.as_mut().set_rect(&data.rect);
            self.as_mut().set_rectf(&data.rectf);
            self.as_mut().set_size(&data.size);
            self.as_mut().set_sizef(&data.sizef);
            self.as_mut().set_string(data.string.as_ref().unwrap());
            self.as_mut().set_time(&data.time);
            self.as_mut().set_url(data.url.as_ref().unwrap());
            self.as_mut().set_variant(data.variant.as_ref().unwrap());
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

    impl From<&MyObjectQt> for Data {
        fn from(value: &MyObjectQt) -> Self {
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

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut MyObjectQt>) {
        cpp.grab_values_from_data(Data::default());
    }
}
