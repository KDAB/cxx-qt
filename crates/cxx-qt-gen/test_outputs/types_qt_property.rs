#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "emit_color_changed"]
        fn emitColorChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_date_changed"]
        fn emitDateChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_date_time_changed"]
        fn emitDateTimeChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_point_changed"]
        fn emitPointChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_pointf_changed"]
        fn emitPointfChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_rect_changed"]
        fn emitRectChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_rectf_changed"]
        fn emitRectfChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_size_changed"]
        fn emitSizeChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_sizef_changed"]
        fn emitSizefChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_string_changed"]
        fn emitStringChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_time_changed"]
        fn emitTimeChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_url_changed"]
        fn emitUrlChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_variant_changed"]
        fn emitVariantChanged(self: Pin<&mut MyObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;

        #[cxx_name = "getColor"]
        unsafe fn get_color<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a UniquePtr<QColor>;
        #[cxx_name = "setColor"]
        fn set_color(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QColor>);

        #[cxx_name = "getDate"]
        unsafe fn get_date<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a QDate;
        #[cxx_name = "setDate"]
        fn set_date(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: QDate);

        #[cxx_name = "getDateTime"]
        unsafe fn get_date_time<'a>(
            self: &'a MyObject,
            cpp: &'a MyObjectQt,
        ) -> &'a UniquePtr<QDateTime>;
        #[cxx_name = "setDateTime"]
        fn set_date_time(
            self: &mut MyObject,
            cpp: Pin<&mut MyObjectQt>,
            value: UniquePtr<QDateTime>,
        );

        #[cxx_name = "getPoint"]
        unsafe fn get_point<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a QPoint;
        #[cxx_name = "setPoint"]
        fn set_point(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: QPoint);

        #[cxx_name = "getPointf"]
        unsafe fn get_pointf<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a QPointF;
        #[cxx_name = "setPointf"]
        fn set_pointf(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: QPointF);

        #[cxx_name = "getRect"]
        unsafe fn get_rect<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a QRect;
        #[cxx_name = "setRect"]
        fn set_rect(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: QRect);

        #[cxx_name = "getRectf"]
        unsafe fn get_rectf<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a QRectF;
        #[cxx_name = "setRectf"]
        fn set_rectf(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: QRectF);

        #[cxx_name = "getSize"]
        unsafe fn get_size<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a QSize;
        #[cxx_name = "setSize"]
        fn set_size(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: QSize);

        #[cxx_name = "getSizef"]
        unsafe fn get_sizef<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a QSizeF;
        #[cxx_name = "setSizef"]
        fn set_sizef(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: QSizeF);

        #[cxx_name = "getString"]
        unsafe fn get_string<'a>(self: &'a MyObject, cpp: &'a MyObjectQt)
            -> &'a UniquePtr<QString>;
        #[cxx_name = "setString"]
        fn set_string(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QString>);

        #[cxx_name = "getTime"]
        unsafe fn get_time<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a QTime;
        #[cxx_name = "setTime"]
        fn set_time(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: QTime);

        #[cxx_name = "getUrl"]
        unsafe fn get_url<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a UniquePtr<QUrl>;
        #[cxx_name = "setUrl"]
        fn set_url(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QUrl>);

        #[cxx_name = "getVariant"]
        unsafe fn get_variant<'a>(
            self: &'a MyObject,
            cpp: &'a MyObjectQt,
        ) -> &'a UniquePtr<QVariant>;
        #[cxx_name = "setVariant"]
        fn set_variant(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QVariant>);
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

        #[rust_name = "new_cpp_object_my_object_qt"]
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
        fn create_rs_my_object() -> Box<MyObject>;
    }
}

pub use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;

    type UniquePtr<T> = cxx::UniquePtr<T>;

    unsafe impl Send for MyObjectCxxQtThread {}

    use std::pin::Pin;

    #[derive(Default)]
    pub struct MyObject {
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

    impl MyObject {
        pub fn get_color<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a UniquePtr<QColor> {
            cpp.get_color()
        }

        pub fn set_color(&mut self, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QColor>) {
            cpp.set_color(value);
        }

        pub fn get_date<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a QDate {
            cpp.get_date()
        }

        pub fn set_date(&mut self, cpp: Pin<&mut MyObjectQt>, value: QDate) {
            cpp.set_date(value);
        }

        pub fn get_date_time<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a UniquePtr<QDateTime> {
            cpp.get_date_time()
        }

        pub fn set_date_time(&mut self, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QDateTime>) {
            cpp.set_date_time(value);
        }

        pub fn get_point<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a QPoint {
            cpp.get_point()
        }

        pub fn set_point(&mut self, cpp: Pin<&mut MyObjectQt>, value: QPoint) {
            cpp.set_point(value);
        }

        pub fn get_pointf<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a QPointF {
            cpp.get_pointf()
        }

        pub fn set_pointf(&mut self, cpp: Pin<&mut MyObjectQt>, value: QPointF) {
            cpp.set_pointf(value);
        }

        pub fn get_rect<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a QRect {
            cpp.get_rect()
        }

        pub fn set_rect(&mut self, cpp: Pin<&mut MyObjectQt>, value: QRect) {
            cpp.set_rect(value);
        }

        pub fn get_rectf<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a QRectF {
            cpp.get_rectf()
        }

        pub fn set_rectf(&mut self, cpp: Pin<&mut MyObjectQt>, value: QRectF) {
            cpp.set_rectf(value);
        }

        pub fn get_size<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a QSize {
            cpp.get_size()
        }

        pub fn set_size(&mut self, cpp: Pin<&mut MyObjectQt>, value: QSize) {
            cpp.set_size(value);
        }

        pub fn get_sizef<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a QSizeF {
            cpp.get_sizef()
        }

        pub fn set_sizef(&mut self, cpp: Pin<&mut MyObjectQt>, value: QSizeF) {
            cpp.set_sizef(value);
        }

        pub fn get_string<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a UniquePtr<QString> {
            cpp.get_string()
        }

        pub fn set_string(&mut self, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QString>) {
            cpp.set_string(value);
        }

        pub fn get_time<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a QTime {
            cpp.get_time()
        }

        pub fn set_time(&mut self, cpp: Pin<&mut MyObjectQt>, value: QTime) {
            cpp.set_time(value);
        }

        pub fn get_url<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a UniquePtr<QUrl> {
            cpp.get_url()
        }

        pub fn set_url(&mut self, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QUrl>) {
            cpp.set_url(value);
        }

        pub fn get_variant<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a UniquePtr<QVariant> {
            cpp.get_variant()
        }

        pub fn set_variant(&mut self, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QVariant>) {
            cpp.set_variant(value);
        }
    }

    impl MyObjectQt {
        pub fn get_color(&self) -> &UniquePtr<QColor> {
            &self.rust().color
        }

        pub fn set_color(mut self: Pin<&mut Self>, value: UniquePtr<QColor>) {
            unsafe {
                self.as_mut().rust_mut().color = value;
            }
            self.as_mut().emit_color_changed();
        }

        pub fn get_date(&self) -> &QDate {
            &self.rust().date
        }

        pub fn set_date(mut self: Pin<&mut Self>, value: QDate) {
            unsafe {
                self.as_mut().rust_mut().date = value;
            }
            self.as_mut().emit_date_changed();
        }

        pub fn get_date_time(&self) -> &UniquePtr<QDateTime> {
            &self.rust().date_time
        }

        pub fn set_date_time(mut self: Pin<&mut Self>, value: UniquePtr<QDateTime>) {
            unsafe {
                self.as_mut().rust_mut().date_time = value;
            }
            self.as_mut().emit_date_time_changed();
        }

        pub fn get_point(&self) -> &QPoint {
            &self.rust().point
        }

        pub fn set_point(mut self: Pin<&mut Self>, value: QPoint) {
            unsafe {
                self.as_mut().rust_mut().point = value;
            }
            self.as_mut().emit_point_changed();
        }

        pub fn get_pointf(&self) -> &QPointF {
            &self.rust().pointf
        }

        pub fn set_pointf(mut self: Pin<&mut Self>, value: QPointF) {
            unsafe {
                self.as_mut().rust_mut().pointf = value;
            }
            self.as_mut().emit_pointf_changed();
        }

        pub fn get_rect(&self) -> &QRect {
            &self.rust().rect
        }

        pub fn set_rect(mut self: Pin<&mut Self>, value: QRect) {
            unsafe {
                self.as_mut().rust_mut().rect = value;
            }
            self.as_mut().emit_rect_changed();
        }

        pub fn get_rectf(&self) -> &QRectF {
            &self.rust().rectf
        }

        pub fn set_rectf(mut self: Pin<&mut Self>, value: QRectF) {
            unsafe {
                self.as_mut().rust_mut().rectf = value;
            }
            self.as_mut().emit_rectf_changed();
        }

        pub fn get_size(&self) -> &QSize {
            &self.rust().size
        }

        pub fn set_size(mut self: Pin<&mut Self>, value: QSize) {
            unsafe {
                self.as_mut().rust_mut().size = value;
            }
            self.as_mut().emit_size_changed();
        }

        pub fn get_sizef(&self) -> &QSizeF {
            &self.rust().sizef
        }

        pub fn set_sizef(mut self: Pin<&mut Self>, value: QSizeF) {
            unsafe {
                self.as_mut().rust_mut().sizef = value;
            }
            self.as_mut().emit_sizef_changed();
        }

        pub fn get_string(&self) -> &UniquePtr<QString> {
            &self.rust().string
        }

        pub fn set_string(mut self: Pin<&mut Self>, value: UniquePtr<QString>) {
            unsafe {
                self.as_mut().rust_mut().string = value;
            }
            self.as_mut().emit_string_changed();
        }

        pub fn get_time(&self) -> &QTime {
            &self.rust().time
        }

        pub fn set_time(mut self: Pin<&mut Self>, value: QTime) {
            unsafe {
                self.as_mut().rust_mut().time = value;
            }
            self.as_mut().emit_time_changed();
        }

        pub fn get_url(&self) -> &UniquePtr<QUrl> {
            &self.rust().url
        }

        pub fn set_url(mut self: Pin<&mut Self>, value: UniquePtr<QUrl>) {
            unsafe {
                self.as_mut().rust_mut().url = value;
            }
            self.as_mut().emit_url_changed();
        }

        pub fn get_variant(&self) -> &UniquePtr<QVariant> {
            &self.rust().variant
        }

        pub fn set_variant(mut self: Pin<&mut Self>, value: UniquePtr<QVariant>) {
            unsafe {
                self.as_mut().rust_mut().variant = value;
            }
            self.as_mut().emit_variant_changed();
        }
    }
    pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
}
