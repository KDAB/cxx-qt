// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::pin::Pin;
use cxx_qt_lib::{
    QColor, QDate, QDateTime, QPoint, QPointF, QRect, QRectF, QSize, QSizeF, QString, QTime, QUrl,
    QVariant, QVariantValue,
};

#[cxx::bridge]
mod ffi {
    enum ColorTest {
        Rgb_Red,
        Rgb_Green,
        Rgb_Blue,
        Rgb_Transparent,
    }

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
        include!("bridge.h");

        type QColor = cxx_qt_lib::QColor;
        type QDate = cxx_qt_lib::QDate;
        type QDateTime = cxx_qt_lib::QDateTime;
        type QString = cxx_qt_lib::QString;
        type QUrl = cxx_qt_lib::QUrl;
        type QVariant = cxx_qt_lib::QVariant;
        type QSize = cxx_qt_lib::QSize;
        type QSizeF = cxx_qt_lib::QSizeF;
        type QPoint = cxx_qt_lib::QPoint;
        type QPointF = cxx_qt_lib::QPointF;
        type QRectF = cxx_qt_lib::QRectF;
        type QRect = cxx_qt_lib::QRect;
        type QTime = cxx_qt_lib::QTime;

        fn test_constructed_qvariant(s: &QVariant, test: VariantTest) -> bool;
    }

    extern "Rust" {
        fn construct_qstring(slice: bool) -> QString;
        fn read_qstring(s: &QString) -> bool;
        fn modify_qstring(s: Pin<&mut QString>);
        fn can_handle_qstring_change() -> bool;
        fn clone_qstring(s: &QString) -> QString;
        fn clone_value_qstring(s: QString) -> QString;

        fn construct_qcolor(test: ColorTest) -> QColor;
        fn read_qcolor(c: &QColor, test: ColorTest) -> bool;
        fn clone_qcolor(c: &QColor) -> QColor;
        fn clone_value_qcolor(c: QColor) -> QColor;

        fn construct_qdatetime(date: &QDate, time: &QTime) -> QDateTime;
        fn read_qdatetime(c: &QDateTime, date: &QDate, time: &QTime) -> bool;
        fn clone_qdatetime(c: &QDateTime) -> QDateTime;
        fn clone_value_qdatetime(c: QDateTime) -> QDateTime;

        fn construct_qurl(test: &QString) -> QUrl;
        fn read_qurl(u: &QUrl, test: &QString) -> bool;
        fn clone_qurl(u: &QUrl) -> QUrl;
        fn clone_value_qurl(u: QUrl) -> QUrl;

        fn make_variant(test: VariantTest) -> UniquePtr<QVariant>;
        fn can_construct_qvariant(test: VariantTest) -> bool;
        fn can_read_qvariant(v: &QVariant, test: VariantTest) -> bool;

        fn construct_qdate() -> QDate;
        fn read_qdate(d: &QDate) -> bool;
        fn clone_qdate(d: &QDate) -> QDate;
        fn clone_value_qdate(d: QDate) -> QDate;

        fn construct_qpoint() -> QPoint;
        fn read_qpoint(p: &QPoint) -> bool;
        fn clone_qpoint(p: &QPoint) -> QPoint;
        fn clone_value_qpoint(p: QPoint) -> QPoint;

        fn construct_qpointf() -> QPointF;
        fn read_qpointf(p: &QPointF) -> bool;
        fn clone_qpointf(p: &QPointF) -> QPointF;
        fn clone_value_qpointf(p: QPointF) -> QPointF;

        fn construct_qrect() -> QRect;
        fn read_qrect(p: &QRect) -> bool;
        fn clone_qrect(p: &QRect) -> QRect;
        fn clone_value_qrect(p: QRect) -> QRect;

        fn construct_qrectf() -> QRectF;
        fn read_qrectf(p: &QRectF) -> bool;
        fn clone_qrectf(p: &QRectF) -> QRectF;
        fn clone_value_qrectf(p: QRectF) -> QRectF;

        fn construct_qsize() -> QSize;
        fn read_qsize(p: &QSize) -> bool;
        fn clone_qsize(p: &QSize) -> QSize;
        fn clone_value_qsize(p: QSize) -> QSize;

        fn construct_qsizef() -> QSizeF;
        fn read_qsizef(p: &QSizeF) -> bool;
        fn clone_qsizef(p: &QSizeF) -> QSizeF;
        fn clone_value_qsizef(p: QSizeF) -> QSizeF;

        fn construct_qtime() -> QTime;
        fn read_qtime(p: &QTime) -> bool;
        fn clone_qtime(p: &QTime) -> QTime;
        fn clone_value_qtime(p: QTime) -> QTime;
    }
}

use ffi::ColorTest;
use ffi::VariantTest;

fn construct_qstring(slice: bool) -> QString {
    if slice {
        QString::from("String constructed by Rust")
    } else {
        let rs_string = "String constructed by Rust".to_owned();
        QString::from(&rs_string)
    }
}

fn read_qstring(s: &cxx_qt_lib::QString) -> bool {
    let rs = s.to_string();
    rs == "String constructed by C++"
}

fn modify_qstring(mut s: Pin<&mut cxx_qt_lib::QString>) {
    *s = QString::from("Updated string value");
}

fn can_handle_qstring_change() -> bool {
    let long_s = "Very very long string that is hopefully long enough to allocate and get Valgrind's attention :)";
    let long_s_ptr = QString::from(long_s);

    let short_s = "Short string";
    let mut short_s_ptr = QString::from(short_s);
    assert!(short_s_ptr.to_string() == short_s);

    short_s_ptr = long_s_ptr;

    short_s_ptr.to_string() == long_s
}

fn clone_qstring(s: &QString) -> QString {
    s.clone()
}

fn clone_value_qstring(s: QString) -> QString {
    s
}

fn construct_qcolor(test: ColorTest) -> QColor {
    match test {
        ColorTest::Rgb_Red => QColor::from_rgba(255, 0, 0, 255),
        ColorTest::Rgb_Green => QColor::from_rgba(0, 255, 0, 255),
        ColorTest::Rgb_Blue => QColor::from_rgba(0, 0, 255, 255),
        ColorTest::Rgb_Transparent => QColor::from_rgba(0, 0, 0, 0),
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn read_qcolor(color: &QColor, test: ColorTest) -> bool {
    match test {
        ColorTest::Rgb_Red => {
            color.alpha() == 255 && color.red() == 255 && color.green() == 0 && color.blue() == 0
        }
        ColorTest::Rgb_Green => {
            color.alpha() == 255 && color.red() == 0 && color.green() == 255 && color.blue() == 0
        }
        ColorTest::Rgb_Blue => {
            color.alpha() == 255 && color.red() == 0 && color.green() == 0 && color.blue() == 255
        }
        ColorTest::Rgb_Transparent => {
            color.alpha() == 0 && color.red() == 0 && color.green() == 0 && color.blue() == 0
        }
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn clone_qcolor(c: &QColor) -> QColor {
    c.clone()
}

fn clone_value_qcolor(c: QColor) -> QColor {
    c
}

fn construct_qdatetime(date: &QDate, time: &QTime) -> QDateTime {
    QDateTime::from_date_and_time(date, time)
}

fn read_qdatetime(dt: &cxx_qt_lib::QDateTime, date: &QDate, time: &QTime) -> bool {
    dt.date().year() == date.year()
        && dt.date().month() == date.month()
        && dt.date().day() == date.day()
        && dt.time().hour() == time.hour()
        && dt.time().minute() == time.minute()
        && dt.time().second() == time.second()
        && dt.time().msec() == time.msec()
}

fn clone_qdatetime(dt: &QDateTime) -> QDateTime {
    dt.clone()
}

fn clone_value_qdatetime(dt: QDateTime) -> QDateTime {
    dt
}

fn construct_qurl(test: &QString) -> QUrl {
    QUrl::from(test)
}

fn read_qurl(u: &QUrl, test: &QString) -> bool {
    u.to_string() == test.to_string()
}

fn clone_qurl(u: &QUrl) -> QUrl {
    u.clone()
}

fn clone_value_qurl(u: QUrl) -> QUrl {
    u
}

fn make_variant(test: VariantTest) -> cxx::UniquePtr<cxx_qt_lib::QVariant> {
    match test {
        VariantTest::Bool => QVariant::from(true),
        VariantTest::F32 => QVariant::from(1.23_f32),
        VariantTest::F64 => QVariant::from(1.23_f64),
        VariantTest::I8 => QVariant::from(12_i8),
        VariantTest::I16 => QVariant::from(123_i16),
        VariantTest::I32 => QVariant::from(123_i32),
        VariantTest::QColor => QVariant::from(QColor::from_rgba(255, 0, 0, 255)),
        VariantTest::QDate => QVariant::from(QDate::new(2022, 1, 1)),
        VariantTest::QDateTime => QVariant::from(QDateTime::from_date_and_time(
            &QDate::new(2022, 1, 1),
            &QTime::new(1, 2, 3, 4),
        )),
        VariantTest::QPoint => QVariant::from(QPoint::new(1, 3)),
        VariantTest::QPointF => QVariant::from(QPointF::new(1.0, 3.0)),
        VariantTest::QRect => QVariant::from(QRect::new(123, 456, 246, 912)),
        VariantTest::QRectF => QVariant::from(QRectF::new(1.23, 4.56, 2.46, 9.12)),
        VariantTest::QSize => QVariant::from(QSize::new(1, 3)),
        VariantTest::QSizeF => QVariant::from(QSizeF::new(1.0, 3.0)),
        VariantTest::QString => QVariant::from(QString::from("Rust string")),
        VariantTest::QTime => QVariant::from(QTime::new(1, 2, 3, 4)),
        VariantTest::QUrl => QVariant::from(QUrl::from("https://github.com/KDAB")),
        VariantTest::U8 => QVariant::from(12_u8),
        VariantTest::U16 => QVariant::from(123_u16),
        VariantTest::U32 => QVariant::from(123_u32),
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn can_construct_qvariant(test: VariantTest) -> bool {
    let variant = make_variant(test);
    ffi::test_constructed_qvariant(&variant, test)
}

fn can_read_qvariant(v: &cxx_qt_lib::QVariant, test: VariantTest) -> bool {
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

fn construct_qdate() -> QDate {
    QDate::new(2022, 1, 1)
}

fn read_qdate(d: &QDate) -> bool {
    d.year() == 2022 && d.month() == 1 && d.day() == 1
}

fn clone_qdate(d: &QDate) -> QDate {
    d.clone()
}

fn clone_value_qdate(d: QDate) -> QDate {
    d
}

fn construct_qpoint() -> QPoint {
    QPoint::new(2, 4)
}

fn read_qpoint(p: &QPoint) -> bool {
    p.x() == 2 && p.y() == 4
}

fn clone_qpoint(p: &QPoint) -> QPoint {
    p.clone()
}

fn clone_value_qpoint(p: QPoint) -> QPoint {
    p
}

fn construct_qpointf() -> QPointF {
    QPointF::new(1.23, 4.56)
}

fn read_qpointf(p: &QPointF) -> bool {
    ((p.x() - 1.23).abs() < f64::EPSILON) && ((p.y() - 4.56).abs() < f64::EPSILON)
}

fn clone_qpointf(p: &QPointF) -> QPointF {
    p.clone()
}

fn clone_value_qpointf(p: QPointF) -> QPointF {
    p
}

fn construct_qrect() -> QRect {
    QRect::new(1, 4, 2, 8)
}

fn read_qrect(r: &QRect) -> bool {
    r.x() == 1 && r.y() == 4 && r.width() == 2 && r.height() == 8
}

fn clone_qrect(r: &QRect) -> QRect {
    r.clone()
}

fn clone_value_qrect(r: QRect) -> QRect {
    r
}

fn construct_qrectf() -> QRectF {
    QRectF::new(1.23, 4.56, 2.46, 9.12)
}

fn read_qrectf(p: &QRectF) -> bool {
    ((p.x() - 1.23).abs() < f64::EPSILON)
        && ((p.y() - 4.56).abs() < f64::EPSILON)
        && ((p.width() - 2.46).abs() < f64::EPSILON)
        && ((p.height() - 9.12).abs() < f64::EPSILON)
}

fn clone_qrectf(p: &QRectF) -> QRectF {
    p.clone()
}

fn clone_value_qrectf(p: QRectF) -> QRectF {
    p
}

fn construct_qsize() -> QSize {
    QSize::new(1, 4)
}

fn read_qsize(s: &QSize) -> bool {
    s.width() == 1 && s.height() == 4
}

fn clone_qsize(s: &QSize) -> QSize {
    s.clone()
}

fn clone_value_qsize(s: QSize) -> QSize {
    s
}

fn construct_qsizef() -> QSizeF {
    QSizeF::new(1.23, 4.56)
}

fn read_qsizef(s: &QSizeF) -> bool {
    ((s.width() - 1.23).abs() < f64::EPSILON) && ((s.height() - 4.56).abs() < f64::EPSILON)
}

fn clone_qsizef(s: &QSizeF) -> QSizeF {
    s.clone()
}

fn clone_value_qsizef(s: QSizeF) -> QSizeF {
    s
}

fn construct_qtime() -> QTime {
    QTime::new(1, 2, 3, 4)
}

fn read_qtime(s: &QTime) -> bool {
    s.hour() == 1 && s.minute() == 2 && s.second() == 3 && s.msec() == 4
}

fn clone_qtime(s: &QTime) -> QTime {
    s.clone()
}

fn clone_value_qtime(s: QTime) -> QTime {
    s
}
