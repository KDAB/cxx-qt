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
        fn get_color(self: &MyObject, cpp: &MyObjectQt) -> UniquePtr<QColor>;
        #[cxx_name = "setColor"]
        fn set_color(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QColor);

        #[cxx_name = "getDate"]
        fn get_date(self: &MyObject, cpp: &MyObjectQt) -> QDate;
        #[cxx_name = "setDate"]
        fn set_date(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QDate);

        #[cxx_name = "getDateTime"]
        fn get_date_time(self: &MyObject, cpp: &MyObjectQt) -> UniquePtr<QDateTime>;
        #[cxx_name = "setDateTime"]
        fn set_date_time(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QDateTime);

        #[cxx_name = "getPoint"]
        fn get_point(self: &MyObject, cpp: &MyObjectQt) -> QPoint;
        #[cxx_name = "setPoint"]
        fn set_point(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QPoint);

        #[cxx_name = "getPointf"]
        fn get_pointf(self: &MyObject, cpp: &MyObjectQt) -> QPointF;
        #[cxx_name = "setPointf"]
        fn set_pointf(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QPointF);

        #[cxx_name = "getRect"]
        fn get_rect(self: &MyObject, cpp: &MyObjectQt) -> QRect;
        #[cxx_name = "setRect"]
        fn set_rect(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QRect);

        #[cxx_name = "getRectf"]
        fn get_rectf(self: &MyObject, cpp: &MyObjectQt) -> QRectF;
        #[cxx_name = "setRectf"]
        fn set_rectf(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QRectF);

        #[cxx_name = "getSize"]
        fn get_size(self: &MyObject, cpp: &MyObjectQt) -> QSize;
        #[cxx_name = "setSize"]
        fn set_size(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QSize);

        #[cxx_name = "getSizef"]
        fn get_sizef(self: &MyObject, cpp: &MyObjectQt) -> QSizeF;
        #[cxx_name = "setSizef"]
        fn set_sizef(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QSizeF);

        #[cxx_name = "getString"]
        fn get_string(self: &MyObject, cpp: &MyObjectQt) -> UniquePtr<QString>;
        #[cxx_name = "setString"]
        fn set_string(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QString);

        #[cxx_name = "getTime"]
        fn get_time(self: &MyObject, cpp: &MyObjectQt) -> QTime;
        #[cxx_name = "setTime"]
        fn set_time(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QTime);

        #[cxx_name = "getUrl"]
        fn get_url(self: &MyObject, cpp: &MyObjectQt) -> UniquePtr<QUrl>;
        #[cxx_name = "setUrl"]
        fn set_url(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QUrl);

        #[cxx_name = "getVariant"]
        fn get_variant(self: &MyObject, cpp: &MyObjectQt) -> UniquePtr<QVariant>;
        #[cxx_name = "setVariant"]
        fn set_variant(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: &QVariant);
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
        pub fn get_color(&self, cpp: &MyObjectQt) -> UniquePtr<QColor> {
            cpp.get_color()
        }

        pub fn set_color(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QColor) {
            cpp.set_color(value);
        }

        pub fn get_date(&self, cpp: &MyObjectQt) -> QDate {
            cpp.get_date()
        }

        pub fn set_date(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QDate) {
            cpp.set_date(value);
        }

        pub fn get_date_time(&self, cpp: &MyObjectQt) -> UniquePtr<QDateTime> {
            cpp.get_date_time()
        }

        pub fn set_date_time(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QDateTime) {
            cpp.set_date_time(value);
        }

        pub fn get_point(&self, cpp: &MyObjectQt) -> QPoint {
            cpp.get_point()
        }

        pub fn set_point(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QPoint) {
            cpp.set_point(value);
        }

        pub fn get_pointf(&self, cpp: &MyObjectQt) -> QPointF {
            cpp.get_pointf()
        }

        pub fn set_pointf(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QPointF) {
            cpp.set_pointf(value);
        }

        pub fn get_rect(&self, cpp: &MyObjectQt) -> QRect {
            cpp.get_rect()
        }

        pub fn set_rect(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QRect) {
            cpp.set_rect(value);
        }

        pub fn get_rectf(&self, cpp: &MyObjectQt) -> QRectF {
            cpp.get_rectf()
        }

        pub fn set_rectf(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QRectF) {
            cpp.set_rectf(value);
        }

        pub fn get_size(&self, cpp: &MyObjectQt) -> QSize {
            cpp.get_size()
        }

        pub fn set_size(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QSize) {
            cpp.set_size(value);
        }

        pub fn get_sizef(&self, cpp: &MyObjectQt) -> QSizeF {
            cpp.get_sizef()
        }

        pub fn set_sizef(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QSizeF) {
            cpp.set_sizef(value);
        }

        pub fn get_string(&self, cpp: &MyObjectQt) -> UniquePtr<QString> {
            cpp.get_string()
        }

        pub fn set_string(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QString) {
            cpp.set_string(value);
        }

        pub fn get_time(&self, cpp: &MyObjectQt) -> QTime {
            cpp.get_time()
        }

        pub fn set_time(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QTime) {
            cpp.set_time(value);
        }

        pub fn get_url(&self, cpp: &MyObjectQt) -> UniquePtr<QUrl> {
            cpp.get_url()
        }

        pub fn set_url(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QUrl) {
            cpp.set_url(value);
        }

        pub fn get_variant(&self, cpp: &MyObjectQt) -> UniquePtr<QVariant> {
            cpp.get_variant()
        }

        pub fn set_variant(&mut self, cpp: Pin<&mut MyObjectQt>, value: &QVariant) {
            cpp.set_variant(value);
        }
    }

    impl MyObjectQt {
        pub fn get_color(&self) -> UniquePtr<QColor> {
            QColor::from_ref(&self.rust().color)
        }

        pub fn set_color(mut self: Pin<&mut Self>, value: &QColor) {
            unsafe {
                self.as_mut().rust_mut().color = QColor::from_ref(value);
            }
            self.as_mut().emit_color_changed();
        }

        pub fn get_date(&self) -> QDate {
            self.rust().date.clone()
        }

        pub fn set_date(mut self: Pin<&mut Self>, value: &QDate) {
            unsafe {
                self.as_mut().rust_mut().date = value.clone();
            }
            self.as_mut().emit_date_changed();
        }

        pub fn get_date_time(&self) -> UniquePtr<QDateTime> {
            QDateTime::from_ref(&self.rust().date_time)
        }

        pub fn set_date_time(mut self: Pin<&mut Self>, value: &QDateTime) {
            unsafe {
                self.as_mut().rust_mut().date_time = QDateTime::from_ref(value);
            }
            self.as_mut().emit_date_time_changed();
        }

        pub fn get_point(&self) -> QPoint {
            self.rust().point.clone()
        }

        pub fn set_point(mut self: Pin<&mut Self>, value: &QPoint) {
            unsafe {
                self.as_mut().rust_mut().point = value.clone();
            }
            self.as_mut().emit_point_changed();
        }

        pub fn get_pointf(&self) -> QPointF {
            self.rust().pointf.clone()
        }

        pub fn set_pointf(mut self: Pin<&mut Self>, value: &QPointF) {
            unsafe {
                self.as_mut().rust_mut().pointf = value.clone();
            }
            self.as_mut().emit_pointf_changed();
        }

        pub fn get_rect(&self) -> QRect {
            self.rust().rect.clone()
        }

        pub fn set_rect(mut self: Pin<&mut Self>, value: &QRect) {
            unsafe {
                self.as_mut().rust_mut().rect = value.clone();
            }
            self.as_mut().emit_rect_changed();
        }

        pub fn get_rectf(&self) -> QRectF {
            self.rust().rectf.clone()
        }

        pub fn set_rectf(mut self: Pin<&mut Self>, value: &QRectF) {
            unsafe {
                self.as_mut().rust_mut().rectf = value.clone();
            }
            self.as_mut().emit_rectf_changed();
        }

        pub fn get_size(&self) -> QSize {
            self.rust().size.clone()
        }
        pub fn set_size(mut self: Pin<&mut Self>, value: &QSize) {
            unsafe {
                self.as_mut().rust_mut().size = value.clone();
            }
            self.as_mut().emit_size_changed();
        }

        pub fn get_sizef(&self) -> QSizeF {
            self.rust().sizef.clone()
        }

        pub fn set_sizef(mut self: Pin<&mut Self>, value: &QSizeF) {
            unsafe {
                self.as_mut().rust_mut().sizef = value.clone();
            }
            self.as_mut().emit_sizef_changed();
        }

        pub fn get_string(&self) -> UniquePtr<QString> {
            QString::from_ref(&self.rust().string)
        }

        pub fn set_string(mut self: Pin<&mut Self>, value: &QString) {
            unsafe {
                self.as_mut().rust_mut().string = QString::from_ref(value);
            }
            self.as_mut().emit_string_changed();
        }

        pub fn get_time(&self) -> QTime {
            self.rust().time.clone()
        }

        pub fn set_time(mut self: Pin<&mut Self>, value: &QTime) {
            unsafe {
                self.as_mut().rust_mut().time = value.clone();
            }
            self.as_mut().emit_time_changed();
        }

        pub fn get_url(&self) -> UniquePtr<QUrl> {
            QUrl::from_ref(&self.rust().url)
        }

        pub fn set_url(mut self: Pin<&mut Self>, value: &QUrl) {
            unsafe {
                self.as_mut().rust_mut().url = QUrl::from_ref(value);
            }
            self.as_mut().emit_url_changed();
        }

        pub fn get_variant(&self) -> UniquePtr<QVariant> {
            QVariant::from_ref(&self.rust().variant)
        }

        pub fn set_variant(mut self: Pin<&mut Self>, value: &QVariant) {
            unsafe {
                self.as_mut().rust_mut().variant = QVariant::from_ref(value);
            }
            self.as_mut().emit_variant_changed();
        }
    }

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
}
