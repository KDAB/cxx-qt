// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt::make_qobject;

#[make_qobject]
mod mock_qt_types {
    use cxx_qt_lib::{
        Color, DateTime, QColor, QDate, QDateTime, QPoint, QPointF, QRect, QRectF, QSize, QSizeF,
        QTime, QUrl, QVariant, Url, Variant, VariantValue,
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
        variant: Variant,
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
                variant: Variant::from_i32(1),
            }
        }
    }

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn test_color_property(&self, cpp: &mut CppObj) {
            let color = Color::from_rgba(0, 0, 255, 255).to_unique_ptr();
            cpp.set_color(&color);
        }

        #[invokable]
        fn test_color_invokable(&self, _color: &QColor) -> Color {
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
            let date_time = cpp.date_time().to_rust();
            let new_date_time = DateTime::from_date_and_time(
                &QDate::new(2021, 12, 31),
                &QTime::new(
                    date_time.time().hour() * 2,
                    date_time.time().minute() * 3,
                    date_time.time().second() * 4,
                    date_time.time().msec() * 5,
                ),
            )
            .to_unique_ptr();
            cpp.set_date_time(&new_date_time);
        }

        #[invokable]
        fn test_date_time_invokable(&self, date_time: &QDateTime) -> DateTime {
            let date_time = date_time.to_rust();
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
            rect.set_x(rect.x() * 2.0);
            rect.set_y(rect.y() * 3.0);
            rect.set_width(rect.width() * 4.0);
            rect.set_height(rect.height() * 5.0);
            cpp.set_rectf(&rect);
        }

        #[invokable]
        fn test_rectf_invokable(&self, rect: &QRectF) -> QRectF {
            let mut rect = *rect;
            rect.set_x(rect.x() * 2.0);
            rect.set_y(rect.x() * 3.0);
            rect.set_width(rect.width() * 4.0);
            rect.set_height(rect.height() * 5.0);
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
            let url = Url::from_str(&(cpp.url().to_rust().string() + "/cxx-qt"))
                .unwrap()
                .to_unique_ptr();
            cpp.set_url(&url);
        }

        #[invokable]
        fn test_url_invokable(&self, url: &QUrl) -> Url {
            Url::from_str(&(url.to_rust().string() + "/cxx-qt")).unwrap()
        }

        #[invokable]
        fn test_variant_property(&self, cpp: &mut CppObj) {
            match cpp.variant().to_rust().value() {
                VariantValue::Bool(b) => {
                    let variant = Variant::from_bool(!b).to_unique_ptr();
                    cpp.set_variant(&variant);
                }
                VariantValue::F32(f) => {
                    let variant = Variant::from_f32(f * 2.0).to_unique_ptr();
                    cpp.set_variant(&variant);
                }
                VariantValue::F64(d) => {
                    let variant = Variant::from_f64(d * 2.0).to_unique_ptr();
                    cpp.set_variant(&variant);
                }
                VariantValue::I8(i) => {
                    let variant = Variant::from_i8(i * 2).to_unique_ptr();
                    cpp.set_variant(&variant);
                }
                VariantValue::I16(i) => {
                    let variant = Variant::from_i16(i * 2).to_unique_ptr();
                    cpp.set_variant(&variant);
                }
                VariantValue::I32(i) => {
                    let variant = Variant::from_i32(i * 2).to_unique_ptr();
                    cpp.set_variant(&variant);
                }
                VariantValue::U8(i) => {
                    let variant = Variant::from_u8(i * 2).to_unique_ptr();
                    cpp.set_variant(&variant);
                }
                VariantValue::U16(i) => {
                    let variant = Variant::from_u16(i * 2).to_unique_ptr();
                    cpp.set_variant(&variant);
                }
                VariantValue::U32(i) => {
                    let variant = Variant::from_u32(i * 2).to_unique_ptr();
                    cpp.set_variant(&variant);
                }
                _ => panic!("Incorrect variant type!"),
            }
        }

        #[invokable]
        fn test_variant_invokable(&self, variant: &QVariant) -> Variant {
            match variant.to_rust().value() {
                VariantValue::Bool(b) => Variant::from_bool(!b),
                VariantValue::F32(f) => Variant::from_f32(f * 2.0),
                VariantValue::F64(d) => Variant::from_f64(d * 2.0),
                VariantValue::I8(i) => Variant::from_i8(i * 2),
                VariantValue::I16(i) => Variant::from_i16(i * 2),
                VariantValue::I32(i) => Variant::from_i32(i * 2),
                VariantValue::U8(i) => Variant::from_u8(i * 2),
                VariantValue::U16(i) => Variant::from_u16(i * 2),
                VariantValue::U32(i) => Variant::from_u32(i * 2),
                _ => panic!("Incorrect variant type!"),
            }
        }
    }
}
