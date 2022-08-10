// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge]
mod ffi {
    use cxx_qt_lib::QVariantValue;

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

    #[cxx_qt::signals(MockQtTypes)]
    pub enum Signal {
        Ready,
        DataChanged { variant: UniquePtr<QVariant> },
    }

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

    impl Default for Data {
        fn default() -> Self {
            Data {
                color: QColor::from_rgba(255, 0, 0, 255),
                date: QDate::new(2022, 1, 1),
                date_time: QDateTime::from_date_and_time(
                    &QDate::new(2022, 1, 1),
                    &QTime::new(1, 2, 3, 4),
                ),
                point: QPoint::new(1, 3),
                pointf: QPointF::new(1.0, 3.0),
                rect: QRect::new(1, 2, 3, 4),
                rectf: QRectF::new(1.0, 2.0, 3.0, 4.0),
                size: QSize::new(1, 3),
                sizef: QSizeF::new(1.0, 3.0),
                string: QString::from_str("KDAB"),
                time: QTime::new(1, 2, 3, 4),
                url: QUrl::from_str("https://github.com/KDAB"),
                variant: QVariant::from(1_i32),
            }
        }
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MockQtTypes;

    impl cxx_qt::QObject<MockQtTypes> {
        #[qinvokable]
        pub fn test_signal(mut self: Pin<&mut Self>) {
            self.as_mut().emit_queued(Signal::Ready);
            self.as_mut().emit_queued(Signal::DataChanged {
                variant: QVariant::from(true),
            });
        }

        #[qinvokable]
        pub fn test_unsafe_signal(mut self: Pin<&mut Self>) {
            unsafe {
                self.as_mut().emit_immediate(Signal::Ready);
                self.as_mut().emit_immediate(Signal::DataChanged {
                    variant: QVariant::from(true),
                });
            }
        }

        #[qinvokable]
        pub fn test_color_property(self: Pin<&mut Self>) {
            self.set_color(QColor::from_rgba(0, 0, 255, 255).as_ref().unwrap());
        }

        #[qinvokable]
        pub fn test_color_invokable(&self, _color: &QColor) -> UniquePtr<QColor> {
            QColor::from_rgba(0, 255, 0, 255)
        }

        #[qinvokable]
        pub fn test_date_property(self: Pin<&mut Self>) {
            let mut date = *self.date();
            date.set_date(2021, 12, 31);
            self.set_date(&date);
        }

        #[qinvokable]
        pub fn test_date_invokable(&self, date: &QDate) -> QDate {
            let mut date = *date;
            date.set_date(2021, 12, 31);
            date
        }

        #[qinvokable]
        pub fn test_date_time_property(self: Pin<&mut Self>) {
            let date_time = self.date_time();
            let new_date_time = QDateTime::from_date_and_time(
                &QDate::new(2021, 12, 31),
                &QTime::new(
                    date_time.time().hour() * 2,
                    date_time.time().minute() * 3,
                    date_time.time().second() * 4,
                    date_time.time().msec() * 5,
                ),
            );
            self.set_date_time(new_date_time.as_ref().unwrap());
        }

        #[qinvokable]
        pub fn test_date_time_invokable(&self, date_time: &QDateTime) -> UniquePtr<QDateTime> {
            QDateTime::from_date_and_time(
                &QDate::new(2021, 12, 31),
                &QTime::new(
                    date_time.time().hour() * 2,
                    date_time.time().minute() * 3,
                    date_time.time().second() * 4,
                    date_time.time().msec() * 5,
                ),
            )
        }

        #[qinvokable]
        pub fn test_point_property(self: Pin<&mut Self>) {
            let mut point = *self.point();
            point.set_x(point.x() * 2);
            point.set_y(point.y() * 3);
            self.set_point(&point);
        }

        #[qinvokable]
        pub fn test_point_invokable(&self, point: &QPoint) -> QPoint {
            let mut point = *point;
            point.set_x(point.x() * 2);
            point.set_y(point.y() * 3);
            point
        }

        #[qinvokable]
        pub fn test_pointf_property(self: Pin<&mut Self>) {
            let mut point = *self.pointf();
            point.set_x(point.x() * 2.0);
            point.set_y(point.y() * 3.0);
            self.set_pointf(&point);
        }

        #[qinvokable]
        pub fn test_pointf_invokable(&self, point: &QPointF) -> QPointF {
            let mut point = *point;
            point.set_x(point.x() * 2.0);
            point.set_y(point.y() * 3.0);
            point
        }

        #[qinvokable]
        pub fn test_rect_property(self: Pin<&mut Self>) {
            let mut rect = *self.rect();
            // Copy width and height, otherwise when we adjust the x and y it affects the width and height
            let (width, height) = (rect.width(), rect.height());
            rect.set_x(rect.x() * 2);
            rect.set_y(rect.y() * 3);
            rect.set_width(width * 4);
            rect.set_height(height * 5);
            self.set_rect(&rect);
        }

        #[qinvokable]
        pub fn test_rect_invokable(&self, rect: &QRect) -> QRect {
            let mut rect = *rect;
            // Copy width and height, otherwise when we adjust the x and y it affects the width and height
            let (width, height) = (rect.width(), rect.height());
            rect.set_x(rect.x() * 2);
            rect.set_y(rect.x() * 3);
            rect.set_width(width * 4);
            rect.set_height(height * 5);
            rect
        }

        #[qinvokable]
        pub fn test_rectf_property(self: Pin<&mut Self>) {
            let mut rect = *self.rectf();
            // Copy width and height, otherwise when we adjust the x and y it affects the width and height
            let (width, height) = (rect.width(), rect.height());
            rect.set_x(rect.x() * 2.0);
            rect.set_y(rect.y() * 3.0);
            rect.set_width(width * 4.0);
            rect.set_height(height * 5.0);
            self.set_rectf(&rect);
        }

        #[qinvokable]
        pub fn test_rectf_invokable(&self, rect: &QRectF) -> QRectF {
            let mut rect = *rect;
            // Copy width and height, otherwise when we adjust the x and y it affects the width and height
            let (width, height) = (rect.width(), rect.height());
            rect.set_x(rect.x() * 2.0);
            rect.set_y(rect.y() * 3.0);
            rect.set_width(width * 4.0);
            rect.set_height(height * 5.0);
            rect
        }

        #[qinvokable]
        pub fn test_size_property(self: Pin<&mut Self>) {
            let mut size = *self.size();
            size.set_width(size.width() * 2);
            size.set_height(size.height() * 3);
            self.set_size(&size);
        }

        #[qinvokable]
        pub fn test_size_invokable(&self, size: &QSize) -> QSize {
            let mut size = *size;
            size.set_width(size.width() * 2);
            size.set_height(size.height() * 3);
            size
        }

        #[qinvokable]
        pub fn test_sizef_property(self: Pin<&mut Self>) {
            let mut size = *self.sizef();
            size.set_width(size.width() * 2.0);
            size.set_height(size.height() * 3.0);
            self.set_sizef(&size);
        }

        #[qinvokable]
        pub fn test_sizef_invokable(&self, size: &QSizeF) -> QSizeF {
            let mut size = *size;
            size.set_width(size.width() * 2.0);
            size.set_height(size.height() * 3.0);
            size
        }

        #[qinvokable]
        pub fn test_string_property(self: Pin<&mut Self>) {
            let string = QString::from_str(&(self.string().to_string() + "/cxx-qt"));
            self.set_string(string.as_ref().unwrap());
        }

        #[qinvokable]
        pub fn test_string_invokable(&self, string: &QString) -> UniquePtr<QString> {
            QString::from_str(&(string.to_string() + "/cxx-qt"))
        }

        #[qinvokable]
        pub fn test_time_property(self: Pin<&mut Self>) {
            let mut time = *self.time();
            time.set_hms(
                time.hour() * 2,
                time.minute() * 3,
                time.second() * 4,
                time.msec() * 5,
            );
            self.set_time(&time);
        }

        #[qinvokable]
        pub fn test_time_invokable(&self, time: &QTime) -> QTime {
            let mut time = *time;
            time.set_hms(
                time.hour() * 2,
                time.minute() * 3,
                time.second() * 4,
                time.msec() * 5,
            );
            time
        }

        #[qinvokable]
        pub fn test_url_property(self: Pin<&mut Self>) {
            let url = QUrl::from_str(&(self.url().string() + "/cxx-qt"));
            self.set_url(url.as_ref().unwrap());
        }

        #[qinvokable]
        pub fn test_url_invokable(&self, url: &QUrl) -> UniquePtr<QUrl> {
            QUrl::from_str(&(url.string() + "/cxx-qt"))
        }

        #[qinvokable]
        pub fn test_variant_property(mut self: Pin<&mut Self>) {
            match self.variant().value() {
                QVariantValue::Bool(b) => self
                    .as_mut()
                    .set_variant(QVariant::from(!b).as_ref().unwrap()),
                QVariantValue::F32(f) => self
                    .as_mut()
                    .set_variant(QVariant::from(f * 2.0).as_ref().unwrap()),
                QVariantValue::F64(d) => self
                    .as_mut()
                    .set_variant(QVariant::from(d * 2.0).as_ref().unwrap()),
                QVariantValue::I8(i) => self
                    .as_mut()
                    .set_variant(QVariant::from(i * 2).as_ref().unwrap()),
                QVariantValue::I16(i) => self
                    .as_mut()
                    .set_variant(QVariant::from(i * 2).as_ref().unwrap()),
                QVariantValue::I32(i) => self
                    .as_mut()
                    .set_variant(QVariant::from(i * 2).as_ref().unwrap()),
                QVariantValue::QColor(mut color) => {
                    if let Some(mut color) = color.as_mut() {
                        color.as_mut().set_red(0);
                        color.as_mut().set_green(0);
                        color.as_mut().set_blue(255);
                        color.as_mut().set_alpha(255);
                    }
                    self.as_mut()
                        .set_variant(QVariant::from(color.as_ref().unwrap()).as_ref().unwrap());
                }
                QVariantValue::QDate(mut date) => {
                    date.set_date(2021, 12, 31);
                    self.as_mut()
                        .set_variant(QVariant::from(date).as_ref().unwrap());
                }
                QVariantValue::QDateTime(mut date_time) => {
                    if let Some(mut date_time) = date_time.as_mut() {
                        date_time.as_mut().set_date(QDate::new(2021, 12, 31));
                        let new_time = QTime::new(
                            date_time.time().hour() * 2,
                            date_time.time().minute() * 3,
                            date_time.time().second() * 4,
                            date_time.time().msec() * 5,
                        );
                        date_time.as_mut().set_time(new_time);
                    }
                    self.as_mut().set_variant(
                        QVariant::from(date_time.as_ref().unwrap())
                            .as_ref()
                            .unwrap(),
                    );
                }
                QVariantValue::QPoint(point) => {
                    self.as_mut().set_variant(
                        QVariant::from(QPoint::new(point.x() * 2, point.y() * 2))
                            .as_ref()
                            .unwrap(),
                    );
                }
                QVariantValue::QPointF(pointf) => {
                    self.as_mut().set_variant(
                        QVariant::from(QPointF::new(pointf.x() * 2.0, pointf.y() * 2.0))
                            .as_ref()
                            .unwrap(),
                    );
                }
                QVariantValue::QRect(rect) => {
                    self.as_mut().set_variant(
                        QVariant::from(QRect::new(
                            rect.x() * 2,
                            rect.y() * 3,
                            rect.width() * 4,
                            rect.height() * 5,
                        ))
                        .as_ref()
                        .unwrap(),
                    );
                }
                QVariantValue::QRectF(rectf) => {
                    self.as_mut().set_variant(
                        QVariant::from(QRectF::new(
                            rectf.x() * 2.0,
                            rectf.y() * 3.0,
                            rectf.width() * 4.0,
                            rectf.height() * 5.0,
                        ))
                        .as_ref()
                        .unwrap(),
                    );
                }
                QVariantValue::QSize(size) => {
                    self.as_mut().set_variant(
                        QVariant::from(QSize::new(size.width() * 2, size.height() * 2))
                            .as_ref()
                            .unwrap(),
                    );
                }
                QVariantValue::QSizeF(sizef) => {
                    self.as_mut().set_variant(
                        QVariant::from(QSizeF::new(sizef.width() * 2.0, sizef.height() * 2.0))
                            .as_ref()
                            .unwrap(),
                    );
                }
                QVariantValue::QString(string) => {
                    let string = QString::from_str(&(string.to_string() + "/cxx-qt"));
                    self.as_mut()
                        .set_variant(QVariant::from(string.as_ref().unwrap()).as_ref().unwrap());
                }
                QVariantValue::QTime(mut time) => {
                    time.set_hms(
                        time.hour() * 2,
                        time.minute() * 3,
                        time.second() * 4,
                        time.msec() * 5,
                    );
                    self.as_mut()
                        .set_variant(QVariant::from(time).as_ref().unwrap());
                }
                QVariantValue::QUrl(url) => {
                    let url = QUrl::from_str(&(url.string() + "/cxx-qt"));
                    self.as_mut()
                        .set_variant(QVariant::from(url.as_ref().unwrap()).as_ref().unwrap());
                }
                QVariantValue::U8(i) => self
                    .as_mut()
                    .set_variant(QVariant::from(i * 2).as_ref().unwrap()),
                QVariantValue::U16(i) => self
                    .as_mut()
                    .set_variant(QVariant::from(i * 2).as_ref().unwrap()),
                QVariantValue::U32(i) => self
                    .as_mut()
                    .set_variant(QVariant::from(i * 2).as_ref().unwrap()),
                _ => panic!("Incorrect variant type!"),
            }
        }

        #[qinvokable]
        pub fn test_variant_invokable(&self, variant: &QVariant) -> UniquePtr<QVariant> {
            match variant.value() {
                QVariantValue::Bool(b) => QVariant::from(!b),
                QVariantValue::F32(f) => QVariant::from(f * 2.0),
                QVariantValue::F64(d) => QVariant::from(d * 2.0),
                QVariantValue::I8(i) => QVariant::from(i * 2),
                QVariantValue::I16(i) => QVariant::from(i * 2),
                QVariantValue::I32(i) => QVariant::from(i * 2),
                QVariantValue::QColor(mut color) => {
                    if let Some(mut color) = color.as_mut() {
                        color.as_mut().set_red(0);
                        color.as_mut().set_green(255);
                        color.as_mut().set_blue(0);
                        color.as_mut().set_alpha(255);
                    }
                    QVariant::from(color.as_ref().unwrap())
                }
                QVariantValue::QDate(mut date) => {
                    date.set_date(2021, 12, 31);
                    QVariant::from(date)
                }
                QVariantValue::QDateTime(mut date_time) => {
                    if let Some(mut date_time) = date_time.as_mut() {
                        date_time.as_mut().set_date(QDate::new(2021, 12, 31));
                        let new_time = QTime::new(
                            date_time.time().hour() * 2,
                            date_time.time().minute() * 3,
                            date_time.time().second() * 4,
                            date_time.time().msec() * 5,
                        );
                        date_time.as_mut().set_time(new_time);
                    }
                    QVariant::from(date_time.as_ref().unwrap())
                }
                QVariantValue::QPoint(point) => {
                    QVariant::from(QPoint::new(point.x() * 2, point.y() * 2))
                }
                QVariantValue::QPointF(pointf) => {
                    QVariant::from(QPointF::new(pointf.x() * 2.0, pointf.y() * 2.0))
                }
                QVariantValue::QRect(rect) => QVariant::from(QRect::new(
                    rect.x() * 2,
                    rect.y() * 3,
                    rect.width() * 4,
                    rect.height() * 5,
                )),
                QVariantValue::QRectF(rectf) => QVariant::from(QRectF::new(
                    rectf.x() * 2.0,
                    rectf.y() * 3.0,
                    rectf.width() * 4.0,
                    rectf.height() * 5.0,
                )),
                QVariantValue::QSize(size) => {
                    QVariant::from(QSize::new(size.width() * 2, size.height() * 2))
                }
                QVariantValue::QSizeF(sizef) => {
                    QVariant::from(QSizeF::new(sizef.width() * 2.0, sizef.height() * 2.0))
                }
                QVariantValue::QString(string) => {
                    let string = QString::from_str(&(string.to_string() + "/cxx-qt"));
                    QVariant::from(string.as_ref().unwrap())
                }
                QVariantValue::QTime(mut time) => {
                    time.set_hms(
                        time.hour() * 2,
                        time.minute() * 3,
                        time.second() * 4,
                        time.msec() * 5,
                    );
                    QVariant::from(time)
                }
                QVariantValue::QUrl(url) => {
                    let url = QUrl::from_str(&(url.string() + "/cxx-qt"));
                    QVariant::from(url.as_ref().unwrap())
                }
                QVariantValue::U8(i) => QVariant::from(i * 2),
                QVariantValue::U16(i) => QVariant::from(i * 2),
                QVariantValue::U32(i) => QVariant::from(i * 2),
                _ => panic!("Incorrect variant type!"),
            }
        }
    }
}
