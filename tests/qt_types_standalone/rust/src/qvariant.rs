// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt_lib::{
    QByteArray, QColor, QDate, QDateTime, QPoint, QPointF, QRect, QRectF, QSize, QSizeF, QString,
    QTime, QTimeZone, QUrl, QVariant,
};

#[cxx::bridge]
mod qvariant_cxx {
    enum VariantTest {
        Bool,
        F32,
        F64,
        I8,
        I16,
        I32,
        QByteArray,
        QColor,
        QDate,
        QDateTime,
        QPoint,
        QPointF,
        QRect,
        QRectF,
        QSize,
        QSizeF,
        QTime,
        QUrl,
        QString,
        U8,
        U16,
        U32,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qvariant.h");

        type QVariant = cxx_qt_lib::QVariant;
    }

    extern "Rust" {
        fn construct_qvariant(test: VariantTest) -> QVariant;
        fn read_qvariant(v: &QVariant, test: VariantTest) -> bool;
        fn clone_qvariant(v: &QVariant) -> QVariant;
    }
}

use qvariant_cxx::VariantTest;

fn construct_qvariant(test: VariantTest) -> QVariant {
    match test {
        VariantTest::Bool => QVariant::from(&true),
        VariantTest::F32 => QVariant::from(&1.23_f32),
        VariantTest::F64 => QVariant::from(&1.23_f64),
        VariantTest::I8 => QVariant::from(&12_i8),
        VariantTest::I16 => QVariant::from(&123_i16),
        VariantTest::I32 => QVariant::from(&123_i32),
        VariantTest::QByteArray => QVariant::from(&QByteArray::from("Rust bytes")),
        VariantTest::QColor => QVariant::from(&QColor::from_rgb(255, 0, 0)),
        VariantTest::QDate => QVariant::from(&QDate::new(2022, 1, 1)),
        VariantTest::QDateTime => QVariant::from(&QDateTime::from_date_and_time_time_zone(
            &QDate::new(2022, 1, 1),
            &QTime::new(1, 2, 3, 4),
            &QTimeZone::owned_from_offset_seconds(0),
        )),
        VariantTest::QPoint => QVariant::from(&QPoint::new(1, 3)),
        VariantTest::QPointF => QVariant::from(&QPointF::new(1.0, 3.0)),
        VariantTest::QRect => QVariant::from(&QRect::new(123, 456, 246, 912)),
        VariantTest::QRectF => QVariant::from(&QRectF::new(1.23, 4.56, 2.46, 9.12)),
        VariantTest::QSize => QVariant::from(&QSize::new(1, 3)),
        VariantTest::QSizeF => QVariant::from(&QSizeF::new(1.0, 3.0)),
        VariantTest::QString => QVariant::from(&QString::from("Rust string")),
        VariantTest::QTime => QVariant::from(&QTime::new(1, 2, 3, 4)),
        VariantTest::QUrl => QVariant::from(&QUrl::from("https://github.com/KDAB")),
        VariantTest::U8 => QVariant::from(&12_u8),
        VariantTest::U16 => QVariant::from(&123_u16),
        VariantTest::U32 => QVariant::from(&123_u32),
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn read_qvariant(v: &cxx_qt_lib::QVariant, test: VariantTest) -> bool {
    match test {
        VariantTest::Bool => match v.value::<bool>() {
            Some(b) => !b,
            None => false,
        },
        VariantTest::F32 => match v.value::<f32>() {
            Some(f) => f == 89.1,
            None => false,
        },
        VariantTest::F64 => match v.value::<f64>() {
            Some(f) => f == 89.1,
            None => false,
        },
        VariantTest::I8 => match v.value::<i8>() {
            Some(i) => i == 89,
            None => false,
        },
        VariantTest::I16 => match v.value::<i16>() {
            Some(i) => i == 8910,
            None => false,
        },
        VariantTest::I32 => match v.value::<i32>() {
            Some(i) => i == 8910,
            None => false,
        },
        VariantTest::QByteArray => match v.value::<QByteArray>() {
            Some(bytes) => bytes.to_string() == "C++ bytes",
            None => false,
        },
        VariantTest::QColor => match v.value::<QColor>() {
            Some(color) => {
                color.alpha() == 255
                    && color.red() == 0
                    && color.green() == 255
                    && color.blue() == 0
            }
            None => false,
        },
        VariantTest::QDate => match v.value::<QDate>() {
            Some(date) => date.year() == 2021 && date.month() == 12 && date.day() == 31,
            None => false,
        },
        VariantTest::QDateTime => match v.value::<QDateTime>() {
            Some(date_time) => {
                date_time.date().year() == 2021
                    && date_time.date().month() == 12
                    && date_time.date().day() == 31
                    && date_time.time().hour() == 4
                    && date_time.time().minute() == 3
                    && date_time.time().second() == 2
                    && date_time.time().msec() == 1
                    && date_time.offset_from_utc() == 0
            }
            None => false,
        },
        VariantTest::QPoint => match v.value::<QPoint>() {
            Some(point) => point.x() == 8 && point.y() == 9,
            None => false,
        },
        VariantTest::QPointF => match v.value::<QPointF>() {
            Some(pointf) => pointf.x() == 8.0 && pointf.y() == 9.0,
            None => false,
        },
        VariantTest::QRect => match v.value::<QRect>() {
            Some(rect) => {
                rect.x() == 123 && rect.y() == 456 && rect.width() == 246 && rect.height() == 912
            }
            None => false,
        },
        VariantTest::QRectF => match v.value::<QRectF>() {
            Some(rectf) => {
                ((rectf.x() - 1.23).abs() < f64::EPSILON)
                    && ((rectf.y() - 4.56).abs() < f64::EPSILON)
                    && ((rectf.width() - 2.46).abs() < f64::EPSILON)
                    && ((rectf.height() - 9.12).abs() < f64::EPSILON)
            }
            None => false,
        },
        VariantTest::QSize => match v.value::<QSize>() {
            Some(size) => size.width() == 8 && size.height() == 9,
            None => false,
        },
        VariantTest::QSizeF => match v.value::<QSizeF>() {
            Some(sizef) => sizef.width() == 8.0 && sizef.height() == 9.0,
            None => false,
        },
        VariantTest::QString => match v.value::<QString>() {
            Some(s) => s.to_string() == "C++ string",
            None => false,
        },
        VariantTest::QTime => match v.value::<QTime>() {
            Some(time) => {
                time.hour() == 4 && time.minute() == 3 && time.second() == 2 && time.msec() == 1
            }
            None => false,
        },
        VariantTest::QUrl => match v.value::<QUrl>() {
            Some(url) => url.to_string() == "https://github.com/KDAB/cxx-qt",
            None => false,
        },
        VariantTest::U8 => match v.value::<u8>() {
            Some(i) => i == 89,
            None => false,
        },
        VariantTest::U16 => match v.value::<u16>() {
            Some(i) => i == 8910,
            None => false,
        },
        VariantTest::U32 => match v.value::<u32>() {
            Some(i) => i == 8910,
            None => false,
        },
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn clone_qvariant(v: &QVariant) -> QVariant {
    v.clone()
}
