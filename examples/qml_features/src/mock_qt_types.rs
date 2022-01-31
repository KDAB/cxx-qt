// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt::make_qobject;

#[make_qobject]
mod mock_qt_types {
    use cxx_qt_lib::{
        Color, DateTime, QDate, QPoint, QPointF, QRect, QRectF, QSize, QSizeF, QTime, QVariant,
        QVariantValue, Url,
    };
    use std::str::FromStr;

    pub struct Data {
        color: Color,
        date: QDate,
        date_time: DateTime,
        point: QPoint,
        pointf: QPointF,
        rect: QRect,
        rectf: QRectF,
        size: QSize,
        sizef: QSizeF,
        time: QTime,
        url: Url,
        variant: QVariant,
    }

    impl Default for Data {
        fn default() -> Self {
            Data {
                color: Color::from_rgba(255, 0, 0, 255),
                date: QDate::new(2022, 1, 1),
                date_time: DateTime::from_date_and_time(
                    &QDate::new(2022, 1, 1),
                    &QTime::new(1, 2, 3, 4),
                ),
                point: QPoint::new(1, 3),
                pointf: QPointF::new(1.0, 3.0),
                rect: QRect::new(1, 2, 3, 4),
                rectf: QRectF::new(1.0, 2.0, 3.0, 4.0),
                size: QSize::new(1, 3),
                sizef: QSizeF::new(1.0, 3.0),
                time: QTime::new(1, 2, 3, 4),
                url: Url::from_str("https://github.com/KDAB").unwrap(),
                variant: QVariant::from(1_i32),
            }
        }
    }

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn test_color_property(&self, cpp: &mut CppObj) {
            cpp.set_color(Color::from_rgba(0, 0, 255, 255));
        }

        #[invokable]
        fn test_color_invokable(&self, _color: &Color) -> Color {
            Color::from_rgba(0, 255, 0, 255)
        }

        #[invokable]
        fn test_date_property(&self, cpp: &mut CppObj) {
            let mut date = *cpp.date();
            date.set_date(2021, 12, 31);
            cpp.set_date(&date);
        }

        #[invokable]
        fn test_date_invokable(&self, date: &QDate) -> QDate {
            let mut date = *date;
            date.set_date(2021, 12, 31);
            date
        }

        #[invokable]
        fn test_date_time_property(&self, cpp: &mut CppObj) {
            let date_time = cpp.date_time();
            let new_date_time = DateTime::from_date_and_time(
                &QDate::new(2021, 12, 31),
                &QTime::new(
                    date_time.time().hour() * 2,
                    date_time.time().minute() * 3,
                    date_time.time().second() * 4,
                    date_time.time().msec() * 5,
                ),
            );
            cpp.set_date_time(new_date_time);
        }

        #[invokable]
        fn test_date_time_invokable(&self, date_time: &DateTime) -> DateTime {
            DateTime::from_date_and_time(
                &QDate::new(2021, 12, 31),
                &QTime::new(
                    date_time.time().hour() * 2,
                    date_time.time().minute() * 3,
                    date_time.time().second() * 4,
                    date_time.time().msec() * 5,
                ),
            )
        }

        #[invokable]
        fn test_point_property(&self, cpp: &mut CppObj) {
            let mut point = *cpp.point();
            point.set_x(point.x() * 2);
            point.set_y(point.y() * 3);
            cpp.set_point(&point);
        }

        #[invokable]
        fn test_point_invokable(&self, point: &QPoint) -> QPoint {
            let mut point = *point;
            point.set_x(point.x() * 2);
            point.set_y(point.y() * 3);
            point
        }

        #[invokable]
        fn test_pointf_property(&self, cpp: &mut CppObj) {
            let mut point = *cpp.pointf();
            point.set_x(point.x() * 2.0);
            point.set_y(point.y() * 3.0);
            cpp.set_pointf(&point);
        }

        #[invokable]
        fn test_pointf_invokable(&self, point: &QPointF) -> QPointF {
            let mut point = *point;
            point.set_x(point.x() * 2.0);
            point.set_y(point.y() * 3.0);
            point
        }

        #[invokable]
        fn test_rect_property(&self, cpp: &mut CppObj) {
            let mut rect = *cpp.rect();
            // Copy width and height, otherwise when we adjust the x and y it affects the width and height
            let (width, height) = (rect.width(), rect.height());
            rect.set_x(rect.x() * 2);
            rect.set_y(rect.y() * 3);
            rect.set_width(width * 4);
            rect.set_height(height * 5);
            cpp.set_rect(&rect);
        }

        #[invokable]
        fn test_rect_invokable(&self, rect: &QRect) -> QRect {
            let mut rect = *rect;
            // Copy width and height, otherwise when we adjust the x and y it affects the width and height
            let (width, height) = (rect.width(), rect.height());
            rect.set_x(rect.x() * 2);
            rect.set_y(rect.x() * 3);
            rect.set_width(width * 4);
            rect.set_height(height * 5);
            rect
        }

        #[invokable]
        fn test_rectf_property(&self, cpp: &mut CppObj) {
            let mut rect = *cpp.rectf();
            // Copy width and height, otherwise when we adjust the x and y it affects the width and height
            let (width, height) = (rect.width(), rect.height());
            rect.set_x(rect.x() * 2.0);
            rect.set_y(rect.y() * 3.0);
            rect.set_width(width * 4.0);
            rect.set_height(height * 5.0);
            cpp.set_rectf(&rect);
        }

        #[invokable]
        fn test_rectf_invokable(&self, rect: &QRectF) -> QRectF {
            let mut rect = *rect;
            // Copy width and height, otherwise when we adjust the x and y it affects the width and height
            let (width, height) = (rect.width(), rect.height());
            rect.set_x(rect.x() * 2.0);
            rect.set_y(rect.y() * 3.0);
            rect.set_width(width * 4.0);
            rect.set_height(height * 5.0);
            rect
        }

        #[invokable]
        fn test_size_property(&self, cpp: &mut CppObj) {
            let mut size = *cpp.size();
            size.set_width(size.width() * 2);
            size.set_height(size.height() * 3);
            cpp.set_size(&size);
        }

        #[invokable]
        fn test_size_invokable(&self, size: &QSize) -> QSize {
            let mut size = *size;
            size.set_width(size.width() * 2);
            size.set_height(size.height() * 3);
            size
        }

        #[invokable]
        fn test_sizef_property(&self, cpp: &mut CppObj) {
            let mut size = *cpp.sizef();
            size.set_width(size.width() * 2.0);
            size.set_height(size.height() * 3.0);
            cpp.set_sizef(&size);
        }

        #[invokable]
        fn test_sizef_invokable(&self, size: &QSizeF) -> QSizeF {
            let mut size = *size;
            size.set_width(size.width() * 2.0);
            size.set_height(size.height() * 3.0);
            size
        }

        #[invokable]
        fn test_time_property(&self, cpp: &mut CppObj) {
            let mut time = *cpp.time();
            time.set_hms(
                time.hour() * 2,
                time.minute() * 3,
                time.second() * 4,
                time.msec() * 5,
            );
            cpp.set_time(&time);
        }

        #[invokable]
        fn test_time_invokable(&self, time: &QTime) -> QTime {
            let mut time = *time;
            time.set_hms(
                time.hour() * 2,
                time.minute() * 3,
                time.second() * 4,
                time.msec() * 5,
            );
            time
        }

        #[invokable]
        fn test_url_property(&self, cpp: &mut CppObj) {
            let url = Url::from_str(&(cpp.url().string() + "/cxx-qt")).unwrap();
            cpp.set_url(url);
        }

        #[invokable]
        fn test_url_invokable(&self, url: &Url) -> Url {
            Url::from_str(&(url.string() + "/cxx-qt")).unwrap()
        }

        #[invokable]
        fn test_variant_property(&self, cpp: &mut CppObj) {
            match cpp.variant().value() {
                QVariantValue::Bool(b) => cpp.set_variant(QVariant::from(!b)),
                QVariantValue::F32(f) => cpp.set_variant(QVariant::from(f * 2.0)),
                QVariantValue::F64(d) => cpp.set_variant(QVariant::from(d * 2.0)),
                QVariantValue::I8(i) => cpp.set_variant(QVariant::from(i * 2)),
                QVariantValue::I16(i) => cpp.set_variant(QVariant::from(i * 2)),
                QVariantValue::I32(i) => cpp.set_variant(QVariant::from(i * 2)),
                QVariantValue::QColor(mut color) => {
                    color.set_red(0);
                    color.set_green(0);
                    color.set_blue(255);
                    color.set_alpha(255);
                    cpp.set_variant(QVariant::from(color));
                }
                QVariantValue::QDate(mut date) => {
                    date.set_date(2021, 12, 31);
                    cpp.set_variant(QVariant::from(date));
                }
                QVariantValue::QDateTime(mut date_time) => {
                    date_time.set_date(QDate::new(2021, 12, 31));
                    date_time.set_time(QTime::new(
                        date_time.time().hour() * 2,
                        date_time.time().minute() * 3,
                        date_time.time().second() * 4,
                        date_time.time().msec() * 5,
                    ));
                    cpp.set_variant(QVariant::from(date_time));
                }
                QVariantValue::QPoint(point) => {
                    cpp.set_variant(QVariant::from(QPoint::new(point.x() * 2, point.y() * 2)));
                }
                QVariantValue::QPointF(pointf) => {
                    cpp.set_variant(QVariant::from(QPointF::new(
                        pointf.x() * 2.0,
                        pointf.y() * 2.0,
                    )));
                }
                QVariantValue::QRect(rect) => {
                    cpp.set_variant(QVariant::from(QRect::new(
                        rect.x() * 2,
                        rect.y() * 3,
                        rect.width() * 4,
                        rect.height() * 5,
                    )));
                }
                QVariantValue::QRectF(rectf) => {
                    cpp.set_variant(QVariant::from(QRectF::new(
                        rectf.x() * 2.0,
                        rectf.y() * 3.0,
                        rectf.width() * 4.0,
                        rectf.height() * 5.0,
                    )));
                }
                QVariantValue::QSize(size) => {
                    cpp.set_variant(QVariant::from(QSize::new(
                        size.width() * 2,
                        size.height() * 2,
                    )));
                }
                QVariantValue::QSizeF(sizef) => {
                    cpp.set_variant(QVariant::from(QSizeF::new(
                        sizef.width() * 2.0,
                        sizef.height() * 2.0,
                    )));
                }
                QVariantValue::QTime(mut time) => {
                    time.set_hms(
                        time.hour() * 2,
                        time.minute() * 3,
                        time.second() * 4,
                        time.msec() * 5,
                    );
                    cpp.set_variant(QVariant::from(time));
                }
                QVariantValue::QUrl(url) => {
                    let url = Url::from_str(&(url.string() + "/cxx-qt")).unwrap();
                    cpp.set_variant(QVariant::from(url));
                }
                QVariantValue::U8(i) => cpp.set_variant(QVariant::from(i * 2)),
                QVariantValue::U16(i) => cpp.set_variant(QVariant::from(i * 2)),
                QVariantValue::U32(i) => cpp.set_variant(QVariant::from(i * 2)),
                _ => panic!("Incorrect variant type!"),
            }
        }

        #[invokable]
        fn test_variant_invokable(&self, variant: &QVariant) -> QVariant {
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
                    QVariant::from(color)
                }
                QVariantValue::QDate(mut date) => {
                    date.set_date(2021, 12, 31);
                    QVariant::from(date)
                }
                QVariantValue::QDateTime(mut date_time) => {
                    date_time.set_date(QDate::new(2021, 12, 31));
                    date_time.set_time(QTime::new(
                        date_time.time().hour() * 2,
                        date_time.time().minute() * 3,
                        date_time.time().second() * 4,
                        date_time.time().msec() * 5,
                    ));
                    QVariant::from(date_time)
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
                    let url = Url::from_str(&(url.string() + "/cxx-qt")).unwrap();
                    QVariant::from(url)
                }
                QVariantValue::U8(i) => QVariant::from(i * 2),
                QVariantValue::U16(i) => QVariant::from(i * 2),
                QVariantValue::U32(i) => QVariant::from(i * 2),
                _ => panic!("Incorrect variant type!"),
            }
        }
    }
}
