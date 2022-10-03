// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge(cxx_file_stem = "mock_qt_types")]
mod ffi {
    use cxx_qt_lib::QVariantValue;

    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qdate.h");
        type QDate = cxx_qt_lib::QDate;
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = cxx_qt_lib::QDateTime;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = cxx_qt_lib::QPointF;
        include!("cxx-qt-lib/qrect.h");
        type QRect = cxx_qt_lib::QRect;
        include!("cxx-qt-lib/qrectf.h");
        type QRectF = cxx_qt_lib::QRectF;
        include!("cxx-qt-lib/qsize.h");
        type QSize = cxx_qt_lib::QSize;
        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = cxx_qt_lib::QSizeF;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qtime.h");
        type QTime = cxx_qt_lib::QTime;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
    }

    #[cxx_qt::signals(MockQtTypes)]
    pub enum Signal {
        Ready,
        DataChanged { variant: QVariant },
    }

    #[cxx_qt::qobject]
    pub struct MockQtTypes {
        #[qproperty]
        color: QColor,
        #[qproperty]
        date: QDate,
        #[qproperty]
        date_time: QDateTime,
        #[qproperty]
        point: QPoint,
        #[qproperty]
        pointf: QPointF,
        #[qproperty]
        rect: QRect,
        #[qproperty]
        rectf: QRectF,
        #[qproperty]
        size: QSize,
        #[qproperty]
        sizef: QSizeF,
        #[qproperty]
        string: QString,
        #[qproperty]
        time: QTime,
        #[qproperty]
        url: QUrl,
        #[qproperty]
        variant: QVariant,
    }

    impl Default for MockQtTypes {
        fn default() -> Self {
            Self {
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
                string: QString::from("KDAB"),
                time: QTime::new(1, 2, 3, 4),
                url: QUrl::from("https://github.com/KDAB"),
                variant: QVariant::from(1_i32),
            }
        }
    }

    impl cxx_qt::QObject<MockQtTypes> {
        #[qinvokable]
        pub fn test_signal(mut self: Pin<&mut Self>) {
            self.as_mut().emit(Signal::Ready);
            self.as_mut().emit(Signal::DataChanged {
                variant: QVariant::from(true),
            });
        }

        #[qinvokable]
        pub fn test_color_property(self: Pin<&mut Self>) {
            self.set_color(QColor::from_rgba(0, 0, 255, 255));
        }

        #[qinvokable]
        pub fn test_color_invokable(&self, _color: &QColor) -> QColor {
            QColor::from_rgba(0, 255, 0, 255)
        }

        #[qinvokable]
        pub fn test_date_property(self: Pin<&mut Self>) {
            let mut date = self.date().clone();
            date.set_date(2021, 12, 31);
            self.set_date(date);
        }

        #[qinvokable]
        pub fn test_date_invokable(&self, date: &QDate) -> QDate {
            let mut date = date.clone();
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
            self.set_date_time(new_date_time);
        }

        #[qinvokable(return_cxx_type = "QDateTime")]
        pub fn test_date_time_invokable(&self, date_time: &QDateTime) -> QDateTime {
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
            let mut point = self.point().clone();
            point.set_x(point.x() * 2);
            point.set_y(point.y() * 3);
            self.set_point(point);
        }

        #[qinvokable]
        pub fn test_point_invokable(&self, point: &QPoint) -> QPoint {
            let mut point = point.clone();
            point.set_x(point.x() * 2);
            point.set_y(point.y() * 3);
            point
        }

        #[qinvokable]
        pub fn test_pointf_property(self: Pin<&mut Self>) {
            let mut point = self.pointf().clone();
            point.set_x(point.x() * 2.0);
            point.set_y(point.y() * 3.0);
            self.set_pointf(point);
        }

        #[qinvokable]
        pub fn test_pointf_invokable(&self, point: &QPointF) -> QPointF {
            let mut point = point.clone();
            point.set_x(point.x() * 2.0);
            point.set_y(point.y() * 3.0);
            point
        }

        #[qinvokable]
        pub fn test_rect_property(self: Pin<&mut Self>) {
            let mut rect = self.rect().clone();
            // Copy width and height, otherwise when we adjust the x and y it affects the width and height
            let (width, height) = (rect.width(), rect.height());
            rect.set_x(rect.x() * 2);
            rect.set_y(rect.y() * 3);
            rect.set_width(width * 4);
            rect.set_height(height * 5);
            self.set_rect(rect);
        }

        #[qinvokable]
        pub fn test_rect_invokable(&self, rect: &QRect) -> QRect {
            let mut rect = rect.clone();
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
            let mut rect = self.rectf().clone();
            // Copy width and height, otherwise when we adjust the x and y it affects the width and height
            let (width, height) = (rect.width(), rect.height());
            rect.set_x(rect.x() * 2.0);
            rect.set_y(rect.y() * 3.0);
            rect.set_width(width * 4.0);
            rect.set_height(height * 5.0);
            self.set_rectf(rect);
        }

        #[qinvokable]
        pub fn test_rectf_invokable(&self, rect: &QRectF) -> QRectF {
            let mut rect = rect.clone();
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
            let mut size = self.size().clone();
            size.set_width(size.width() * 2);
            size.set_height(size.height() * 3);
            self.set_size(size);
        }

        #[qinvokable]
        pub fn test_size_invokable(&self, size: &QSize) -> QSize {
            let mut size = size.clone();
            size.set_width(size.width() * 2);
            size.set_height(size.height() * 3);
            size
        }

        #[qinvokable]
        pub fn test_sizef_property(self: Pin<&mut Self>) {
            let mut size = self.sizef().clone();
            size.set_width(size.width() * 2.0);
            size.set_height(size.height() * 3.0);
            self.set_sizef(size);
        }

        #[qinvokable]
        pub fn test_sizef_invokable(&self, size: &QSizeF) -> QSizeF {
            let mut size = size.clone();
            size.set_width(size.width() * 2.0);
            size.set_height(size.height() * 3.0);
            size
        }

        #[qinvokable]
        pub fn test_string_property(self: Pin<&mut Self>) {
            let string = QString::from(&(self.string().to_string() + "/cxx-qt"));
            self.set_string(string);
        }

        #[qinvokable]
        pub fn test_string_invokable(&self, string: &QString) -> QString {
            QString::from(&(string.to_string() + "/cxx-qt"))
        }

        #[qinvokable]
        pub fn test_time_property(self: Pin<&mut Self>) {
            let mut time = self.time().clone();
            time.set_hms(
                time.hour() * 2,
                time.minute() * 3,
                time.second() * 4,
                time.msec() * 5,
            );
            self.set_time(time);
        }

        #[qinvokable]
        pub fn test_time_invokable(&self, time: &QTime) -> QTime {
            let mut time = time.clone();
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
            let url = QUrl::from(&(self.url().to_string() + "/cxx-qt"));
            self.set_url(url);
        }

        #[qinvokable(return_cxx_type = "QUrl")]
        pub fn test_url_invokable(&self, url: &QUrl) -> QUrl {
            QUrl::from(&(url.to_string() + "/cxx-qt"))
        }

        #[qinvokable]
        pub fn test_variant_property(mut self: Pin<&mut Self>) {
            match self.variant().value() {
                QVariantValue::Bool(b) => self.as_mut().set_variant(QVariant::from(!b)),
                QVariantValue::F32(f) => self.as_mut().set_variant(QVariant::from(f * 2.0)),
                QVariantValue::F64(d) => self.as_mut().set_variant(QVariant::from(d * 2.0)),
                QVariantValue::I8(i) => self.as_mut().set_variant(QVariant::from(i * 2)),
                QVariantValue::I16(i) => self.as_mut().set_variant(QVariant::from(i * 2)),
                QVariantValue::I32(i) => self.as_mut().set_variant(QVariant::from(i * 2)),
                QVariantValue::QColor(mut color) => {
                    color.set_red(0);
                    color.set_green(0);
                    color.set_blue(255);
                    color.set_alpha(255);
                    self.as_mut().set_variant(QVariant::from(&color));
                }
                QVariantValue::QDate(mut date) => {
                    date.set_date(2021, 12, 31);
                    self.as_mut().set_variant(QVariant::from(&date));
                }
                QVariantValue::QDateTime(mut date_time) => {
                    date_time.set_date(QDate::new(2021, 12, 31));
                    let new_time = QTime::new(
                        date_time.time().hour() * 2,
                        date_time.time().minute() * 3,
                        date_time.time().second() * 4,
                        date_time.time().msec() * 5,
                    );
                    date_time.set_time(new_time);
                    self.as_mut().set_variant(QVariant::from(&date_time));
                }
                QVariantValue::QPoint(point) => {
                    self.as_mut()
                        .set_variant(QVariant::from(&QPoint::new(point.x() * 2, point.y() * 2)));
                }
                QVariantValue::QPointF(pointf) => {
                    self.as_mut().set_variant(QVariant::from(&QPointF::new(
                        pointf.x() * 2.0,
                        pointf.y() * 2.0,
                    )));
                }
                QVariantValue::QRect(rect) => {
                    self.as_mut().set_variant(QVariant::from(&QRect::new(
                        rect.x() * 2,
                        rect.y() * 3,
                        rect.width() * 4,
                        rect.height() * 5,
                    )));
                }
                QVariantValue::QRectF(rectf) => {
                    self.as_mut().set_variant(QVariant::from(&QRectF::new(
                        rectf.x() * 2.0,
                        rectf.y() * 3.0,
                        rectf.width() * 4.0,
                        rectf.height() * 5.0,
                    )));
                }
                QVariantValue::QSize(size) => {
                    self.as_mut().set_variant(QVariant::from(&QSize::new(
                        size.width() * 2,
                        size.height() * 2,
                    )));
                }
                QVariantValue::QSizeF(sizef) => {
                    self.as_mut().set_variant(QVariant::from(&QSizeF::new(
                        sizef.width() * 2.0,
                        sizef.height() * 2.0,
                    )));
                }
                QVariantValue::QString(string) => {
                    let string = QString::from(&(string.to_string() + "/cxx-qt"));
                    self.as_mut().set_variant(QVariant::from(&string));
                }
                QVariantValue::QTime(mut time) => {
                    time.set_hms(
                        time.hour() * 2,
                        time.minute() * 3,
                        time.second() * 4,
                        time.msec() * 5,
                    );
                    self.as_mut().set_variant(QVariant::from(&time));
                }
                QVariantValue::QUrl(url) => {
                    let url = QUrl::from(&(url.to_string() + "/cxx-qt"));
                    self.as_mut().set_variant(QVariant::from(&url));
                }
                QVariantValue::U8(i) => self.as_mut().set_variant(QVariant::from(i * 2)),
                QVariantValue::U16(i) => self.as_mut().set_variant(QVariant::from(i * 2)),
                QVariantValue::U32(i) => self.as_mut().set_variant(QVariant::from(i * 2)),
                _ => panic!("Incorrect variant type!"),
            }
        }

        #[qinvokable(return_cxx_type = "QVariant")]
        pub fn test_variant_invokable(&self, variant: &QVariant) -> QVariant {
            match variant.value() {
                QVariantValue::Bool(b) => QVariant::from(!b),
                QVariantValue::F32(f) => QVariant::from(f * 2.0),
                QVariantValue::F64(d) => QVariant::from(d * 2.0),
                QVariantValue::I8(i) => QVariant::from(i * 2),
                QVariantValue::I16(i) => QVariant::from(i * 2),
                QVariantValue::I32(i) => QVariant::from(i * 2),
                QVariantValue::QColor(mut color) => {
                    color.set_red(0);
                    color.set_green(255);
                    color.set_blue(0);
                    color.set_alpha(255);
                    QVariant::from(&color)
                }
                QVariantValue::QDate(mut date) => {
                    date.set_date(2021, 12, 31);
                    QVariant::from(&date)
                }
                QVariantValue::QDateTime(mut date_time) => {
                    date_time.set_date(QDate::new(2021, 12, 31));
                    let new_time = QTime::new(
                        date_time.time().hour() * 2,
                        date_time.time().minute() * 3,
                        date_time.time().second() * 4,
                        date_time.time().msec() * 5,
                    );
                    date_time.set_time(new_time);
                    QVariant::from(&date_time)
                }
                QVariantValue::QPoint(point) => {
                    QVariant::from(&QPoint::new(point.x() * 2, point.y() * 2))
                }
                QVariantValue::QPointF(pointf) => {
                    QVariant::from(&QPointF::new(pointf.x() * 2.0, pointf.y() * 2.0))
                }
                QVariantValue::QRect(rect) => QVariant::from(&QRect::new(
                    rect.x() * 2,
                    rect.y() * 3,
                    rect.width() * 4,
                    rect.height() * 5,
                )),
                QVariantValue::QRectF(rectf) => QVariant::from(&QRectF::new(
                    rectf.x() * 2.0,
                    rectf.y() * 3.0,
                    rectf.width() * 4.0,
                    rectf.height() * 5.0,
                )),
                QVariantValue::QSize(size) => {
                    QVariant::from(&QSize::new(size.width() * 2, size.height() * 2))
                }
                QVariantValue::QSizeF(sizef) => {
                    QVariant::from(&QSizeF::new(sizef.width() * 2.0, sizef.height() * 2.0))
                }
                QVariantValue::QString(string) => {
                    let string = QString::from(&(string.to_string() + "/cxx-qt"));
                    QVariant::from(&string)
                }
                QVariantValue::QTime(mut time) => {
                    time.set_hms(
                        time.hour() * 2,
                        time.minute() * 3,
                        time.second() * 4,
                        time.msec() * 5,
                    );
                    QVariant::from(&time)
                }
                QVariantValue::QUrl(url) => {
                    let url = QUrl::from(&(url.to_string() + "/cxx-qt"));
                    QVariant::from(&url)
                }
                QVariantValue::U8(i) => QVariant::from(i * 2),
                QVariantValue::U16(i) => QVariant::from(i * 2),
                QVariantValue::U32(i) => QVariant::from(i * 2),
                _ => panic!("Incorrect variant type!"),
            }
        }
    }
}
