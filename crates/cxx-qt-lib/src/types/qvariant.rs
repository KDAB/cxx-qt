// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

use crate::{
    QColor, QDate, QDateTime, QPoint, QPointF, QRect, QRectF, QSize, QSizeF, QString, QTime, QUrl,
};

#[cxx::bridge]
mod ffi {
    #[repr(u8)]
    #[namespace = "rust::cxxqtlib1::types"]
    enum QVariantType {
        Unsupported = 0,
        Bool = 1,
        F32 = 2,
        F64 = 3,
        I8 = 4,
        I16 = 5,
        I32 = 6,
        QColor = 7,
        QDate = 8,
        QDateTime = 9,
        QPoint = 10,
        QPointF = 11,
        QRect = 12,
        QRectF = 13,
        QSize = 14,
        QSizeF = 15,
        QString = 16,
        QTime = 17,
        QUrl = 18,
        U8 = 19,
        U16 = 20,
        U32 = 21,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;
        include!("cxx-qt-lib/qdate.h");
        type QDate = crate::QDate;
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = crate::QDateTime;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = crate::QPointF;
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;
        include!("cxx-qt-lib/qrectf.h");
        type QRectF = crate::QRectF;
        include!("cxx-qt-lib/qsize.h");
        type QSize = crate::QSize;
        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = crate::QSizeF;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qtime.h");
        type QTime = crate::QTime;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = crate::QUrl;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = super::QVariant;

        #[namespace = "rust::cxxqtlib1::types"]
        type QVariantType;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qvariant_get_type"]
        fn qvariantType(qvariant: &QVariant) -> QVariantType;

        #[doc(hidden)]
        #[rust_name = "qvariant_drop"]
        fn qvariantDrop(variant: &mut QVariant);
        #[doc(hidden)]
        #[rust_name = "qvariant_init_default"]
        fn qvariantInitDefault() -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_qvariant"]
        fn qvariantInitFromQVariant(variant: &QVariant) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_bool"]
        fn qvariantInitFromBool(b: bool) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_f32"]
        fn qvariantInitFromF32(f: f32) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_f64"]
        fn qvariantInitFromF64(f: f64) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_i8"]
        fn qvariantInitFromI8(i: i8) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_i16"]
        fn qvariantInitFromI16(i: i16) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_i32"]
        fn qvariantInitFromI32(i: i32) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_qcolor"]
        fn qvariantInitFromQColor(color: &QColor) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_qdate"]
        fn qvariantInitFromQDate(date: &QDate) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_qdatetime"]
        fn qvariantInitFromQDateTime(dateTime: &QDateTime) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_qpoint"]
        fn qvariantInitFromQPoint(point: &QPoint) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_qpointf"]
        fn qvariantInitFromQPointF(pointf: &QPointF) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_qrect"]
        fn qvariantInitFromQRect(rect: &QRect) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_qrectf"]
        fn qvariantInitFromQRectF(rectf: &QRectF) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_qsize"]
        fn qvariantInitFromQSize(size: &QSize) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_qsizef"]
        fn qvariantInitFromQSizeF(sizef: &QSizeF) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_qtime"]
        fn qvariantInitFromQTime(time: &QTime) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_qurl"]
        fn qvariantInitFromQUrl(url: &QUrl) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_qstring"]
        fn qvariantInitFromQString(string: &QString) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_u8"]
        fn qvariantInitFromU8(u: u8) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_u16"]
        fn qvariantInitFromU16(u: u16) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_init_from_u32"]
        fn qvariantInitFromU32(u: u32) -> QVariant;

        #[doc(hidden)]
        #[rust_name = "qvariant_to_bool"]
        fn qvariantToBool(qvariant: &QVariant) -> bool;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_f32"]
        fn qvariantToF32(qvariant: &QVariant) -> f32;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_f64"]
        fn qvariantToF64(qvariant: &QVariant) -> f64;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_i8"]
        fn qvariantToI8(qvariant: &QVariant) -> i8;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_i16"]
        fn qvariantToI16(qvariant: &QVariant) -> i16;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_i32"]
        fn qvariantToI32(qvariant: &QVariant) -> i32;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_qcolor"]
        fn qvariantToQColor(qvariant: &QVariant) -> QColor;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_qdate"]
        fn qvariantToQDate(qvariant: &QVariant) -> QDate;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_qdatetime"]
        fn qvariantToQDateTime(qvariant: &QVariant) -> QDateTime;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_qpoint"]
        fn qvariantToQPoint(qvariant: &QVariant) -> QPoint;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_qpointf"]
        fn qvariantToQPointF(qvariant: &QVariant) -> QPointF;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_qrect"]
        fn qvariantToQRect(qvariant: &QVariant) -> QRect;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_qrectf"]
        fn qvariantToQRectF(qvariant: &QVariant) -> QRectF;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_qsize"]
        fn qvariantToQSize(qvariant: &QVariant) -> QSize;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_qsizef"]
        fn qvariantToQSizeF(qvariant: &QVariant) -> QSizeF;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_qtime"]
        fn qvariantToQTime(qvariant: &QVariant) -> QTime;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_qurl"]
        fn qvariantToQUrl(qvariant: &QVariant) -> QUrl;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_qstring"]
        fn qvariantToQString(qvariant: &QVariant) -> QString;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_u8"]
        fn qvariantToU8(qvariant: &QVariant) -> u8;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_u16"]
        fn qvariantToU16(qvariant: &QVariant) -> u16;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_u32"]
        fn qvariantToU32(qvariant: &QVariant) -> u32;
    }
}

/// The QVariant class acts like a union for the most common Qt data types.
#[repr(C)]
pub struct QVariant {
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 QString has one member, which contains three uints (but they are optimised to a size of 8) and a union
    /// Qt6 QString has one member, which contains three pointers and a union (pointer largest)
    #[cfg(qt_version_major = "5")]
    _space: MaybeUninit<[usize; 2]>,
    #[cfg(qt_version_major = "6")]
    _space: MaybeUninit<[usize; 4]>,
}

/// The Rust inner value of a QVariant
pub enum QVariantValue {
    Unsupported,
    Bool(bool),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    QColor(QColor),
    QDate(QDate),
    QDateTime(QDateTime),
    QPoint(QPoint),
    QPointF(QPointF),
    QRect(QRect),
    QRectF(QRectF),
    QSize(QSize),
    QSizeF(QSizeF),
    QString(QString),
    QTime(QTime),
    QUrl(QUrl),
    U8(u8),
    U16(u16),
    U32(u32),
}

impl QVariant {
    // TODO: add a set_value(&mut self, value: QVariantValue);

    /// Returns the value of the QVariant as a Rust enum
    pub fn value(&self) -> QVariantValue {
        match ffi::qvariant_get_type(self) {
            ffi::QVariantType::Unsupported => QVariantValue::Unsupported,
            ffi::QVariantType::Bool => QVariantValue::Bool(ffi::qvariant_to_bool(self)),
            ffi::QVariantType::F32 => QVariantValue::F32(ffi::qvariant_to_f32(self)),
            ffi::QVariantType::F64 => QVariantValue::F64(ffi::qvariant_to_f64(self)),
            ffi::QVariantType::I8 => QVariantValue::I8(ffi::qvariant_to_i8(self)),
            ffi::QVariantType::I16 => QVariantValue::I16(ffi::qvariant_to_i16(self)),
            ffi::QVariantType::I32 => QVariantValue::I32(ffi::qvariant_to_i32(self)),
            ffi::QVariantType::QColor => QVariantValue::QColor(ffi::qvariant_to_qcolor(self)),
            ffi::QVariantType::QDate => QVariantValue::QDate(ffi::qvariant_to_qdate(self)),
            ffi::QVariantType::QDateTime => {
                QVariantValue::QDateTime(ffi::qvariant_to_qdatetime(self))
            }
            ffi::QVariantType::QPoint => QVariantValue::QPoint(ffi::qvariant_to_qpoint(self)),
            ffi::QVariantType::QPointF => QVariantValue::QPointF(ffi::qvariant_to_qpointf(self)),
            ffi::QVariantType::QRect => QVariantValue::QRect(ffi::qvariant_to_qrect(self)),
            ffi::QVariantType::QRectF => QVariantValue::QRectF(ffi::qvariant_to_qrectf(self)),
            ffi::QVariantType::QSize => QVariantValue::QSize(ffi::qvariant_to_qsize(self)),
            ffi::QVariantType::QSizeF => QVariantValue::QSizeF(ffi::qvariant_to_qsizef(self)),
            ffi::QVariantType::QString => QVariantValue::QString(ffi::qvariant_to_qstring(self)),
            ffi::QVariantType::QTime => QVariantValue::QTime(ffi::qvariant_to_qtime(self)),
            ffi::QVariantType::QUrl => QVariantValue::QUrl(ffi::qvariant_to_qurl(self)),
            ffi::QVariantType::U8 => QVariantValue::U8(ffi::qvariant_to_u8(self)),
            ffi::QVariantType::U16 => QVariantValue::U16(ffi::qvariant_to_u16(self)),
            ffi::QVariantType::U32 => QVariantValue::U32(ffi::qvariant_to_u32(self)),
            _others => QVariantValue::Unsupported,
        }
    }
}

impl Clone for QVariant {
    /// Constructs a copy of the variant, p, passed as the argument to this constructor.
    fn clone(&self) -> Self {
        ffi::qvariant_init_from_qvariant(self)
    }
}

impl Default for QVariant {
    /// Constructs an invalid variant.
    fn default() -> Self {
        ffi::qvariant_init_default()
    }
}

impl Drop for QVariant {
    /// Destroys the QVariant and the contained object.
    fn drop(&mut self) {
        ffi::qvariant_drop(self)
    }
}

macro_rules! into_qvariant {
    ($typeName:ty, $name:expr) => {
        impl From<$typeName> for QVariant {
            /// Constructs a QVariant from a value of $typeName
            fn from(value: $typeName) -> Self {
                $name(value)
            }
        }
    };
}

macro_rules! into_qvariant_ref {
    ($typeName:ty, $name:expr) => {
        impl From<&$typeName> for QVariant {
            /// Constructs a QVariant from a reference of $typeName
            fn from(value: &$typeName) -> Self {
                $name(value)
            }
        }
    };
}

into_qvariant!(bool, ffi::qvariant_init_from_bool);
into_qvariant!(f32, ffi::qvariant_init_from_f32);
into_qvariant!(f64, ffi::qvariant_init_from_f64);
into_qvariant!(i8, ffi::qvariant_init_from_i8);
into_qvariant!(i16, ffi::qvariant_init_from_i16);
into_qvariant!(i32, ffi::qvariant_init_from_i32);
into_qvariant_ref!(QColor, ffi::qvariant_init_from_qcolor);
into_qvariant_ref!(QDate, ffi::qvariant_init_from_qdate);
into_qvariant_ref!(QDateTime, ffi::qvariant_init_from_qdatetime);
into_qvariant_ref!(QPoint, ffi::qvariant_init_from_qpoint);
into_qvariant_ref!(QPointF, ffi::qvariant_init_from_qpointf);
into_qvariant_ref!(QRect, ffi::qvariant_init_from_qrect);
into_qvariant_ref!(QRectF, ffi::qvariant_init_from_qrectf);
into_qvariant_ref!(QSize, ffi::qvariant_init_from_qsize);
into_qvariant_ref!(QSizeF, ffi::qvariant_init_from_qsizef);
into_qvariant_ref!(QTime, ffi::qvariant_init_from_qtime);
into_qvariant_ref!(QUrl, ffi::qvariant_init_from_qurl);
into_qvariant_ref!(QString, ffi::qvariant_init_from_qstring);
into_qvariant!(u8, ffi::qvariant_init_from_u8);
into_qvariant!(u16, ffi::qvariant_init_from_u16);
into_qvariant!(u32, ffi::qvariant_init_from_u32);

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QVariant {
    type Id = type_id!("QVariant");
    type Kind = cxx::kind::Trivial;
}
