// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt_lib::{
    QColor, QDate, QDateTime, QPoint, QPointF, QRect, QRectF, QSize, QSizeF, QString, QTime, QUrl,
    QVariant, QVariantValue,
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
        include!("cxx-qt-lib/include/qt_types.h");

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
        VariantTest::Bool => QVariant::from(true),
        VariantTest::F32 => QVariant::from(1.23_f32),
        VariantTest::F64 => QVariant::from(1.23_f64),
        VariantTest::I8 => QVariant::from(12_i8),
        VariantTest::I16 => QVariant::from(123_i16),
        VariantTest::I32 => QVariant::from(123_i32),
        VariantTest::QColor => QVariant::from(&QColor::from_rgba(255, 0, 0, 255)),
        VariantTest::QDate => QVariant::from(&QDate::new(2022, 1, 1)),
        VariantTest::QDateTime => QVariant::from(&QDateTime::from_date_and_time(
            &QDate::new(2022, 1, 1),
            &QTime::new(1, 2, 3, 4),
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
        VariantTest::U8 => QVariant::from(12_u8),
        VariantTest::U16 => QVariant::from(123_u16),
        VariantTest::U32 => QVariant::from(123_u32),
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn read_qvariant(v: &cxx_qt_lib::QVariant, test: VariantTest) -> bool {
    let variant = v.value();
    match test {
        VariantTest::Bool => match variant {
            QVariantValue::Bool(b) => !b,
            _others => false,
        },
        VariantTest::F32 => match variant {
            QVariantValue::F32(f) => f == 89.1,
            _others => false,
        },
        VariantTest::F64 => match variant {
            QVariantValue::F64(f) => f == 89.1,
            _others => false,
        },
        VariantTest::I8 => match variant {
            QVariantValue::I8(i) => i == 89,
            _others => false,
        },
        VariantTest::I16 => match variant {
            QVariantValue::I16(i) => i == 8910,
            _others => false,
        },
        VariantTest::I32 => match variant {
            QVariantValue::I32(i) => i == 8910,
            _others => false,
        },
        VariantTest::QColor => match variant {
            QVariantValue::QColor(color) => {
                color.alpha() == 255
                    && color.red() == 0
                    && color.green() == 255
                    && color.blue() == 0
            }
            _others => false,
        },
        VariantTest::QDate => match variant {
            QVariantValue::QDate(date) => {
                date.year() == 2021 && date.month() == 12 && date.day() == 31
            }
            _others => false,
        },
        VariantTest::QDateTime => match variant {
            QVariantValue::QDateTime(date_time) => {
                date_time.date().year() == 2021
                    && date_time.date().month() == 12
                    && date_time.date().day() == 31
                    && date_time.time().hour() == 4
                    && date_time.time().minute() == 3
                    && date_time.time().second() == 2
                    && date_time.time().msec() == 1
            }
            _others => false,
        },
        VariantTest::QPoint => match variant {
            QVariantValue::QPoint(point) => point.x() == 8 && point.y() == 9,
            _others => false,
        },
        VariantTest::QPointF => match variant {
            QVariantValue::QPointF(pointf) => pointf.x() == 8.0 && pointf.y() == 9.0,
            _others => false,
        },
        VariantTest::QRect => match variant {
            QVariantValue::QRect(rect) => {
                rect.x() == 123 && rect.y() == 456 && rect.width() == 246 && rect.height() == 912
            }
            _others => false,
        },
        VariantTest::QRectF => match variant {
            QVariantValue::QRectF(rectf) => {
                ((rectf.x() - 1.23).abs() < f64::EPSILON)
                    && ((rectf.y() - 4.56).abs() < f64::EPSILON)
                    && ((rectf.width() - 2.46).abs() < f64::EPSILON)
                    && ((rectf.height() - 9.12).abs() < f64::EPSILON)
            }
            _others => false,
        },
        VariantTest::QSize => match variant {
            QVariantValue::QSize(size) => size.width() == 8 && size.height() == 9,
            _others => false,
        },
        VariantTest::QSizeF => match variant {
            QVariantValue::QSizeF(sizef) => sizef.width() == 8.0 && sizef.height() == 9.0,
            _others => false,
        },
        VariantTest::QString => match variant {
            QVariantValue::QString(s) => s.to_string() == "C++ string",
            _others => false,
        },
        VariantTest::QTime => match variant {
            QVariantValue::QTime(time) => {
                time.hour() == 4 && time.minute() == 3 && time.second() == 2 && time.msec() == 1
            }
            _others => false,
        },
        VariantTest::QUrl => match variant {
            QVariantValue::QUrl(url) => url.to_string() == "https://github.com/KDAB/cxx-qt",
            _others => false,
        },
        VariantTest::U8 => match variant {
            QVariantValue::U8(i) => i == 89,
            _others => false,
        },
        VariantTest::U16 => match variant {
            QVariantValue::U16(i) => i == 8910,
            _others => false,
        },
        VariantTest::U32 => match variant {
            QVariantValue::U32(i) => i == 8910,
            _others => false,
        },
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn clone_qvariant(v: &QVariant) -> QVariant {
    v.clone()
}
