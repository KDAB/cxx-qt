// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    Color, DateTime, QDate, QPoint, QPointF, QRect, QRectF, QSize, QSizeF, QTime, ToUniquePtr, Url,
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
        String = 16,
        QTime = 17,
        QUrl = 18,
        U8 = 19,
        U16 = 20,
        U32 = 21,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QColor = crate::QColor;
        type QDate = crate::QDate;
        type QDateTime = crate::QDateTime;
        type QPoint = crate::QPoint;
        type QPointF = crate::QPointF;
        type QRect = crate::QRect;
        type QRectF = crate::QRectF;
        type QSize = crate::QSize;
        type QSizeF = crate::QSizeF;
        type QTime = crate::QTime;
        type QUrl = crate::QUrl;
        type QVariant;

        #[namespace = "rust::cxxqtlib1::types"]
        type QVariantType;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        #[rust_name = "qvariant_get_type"]
        fn qvariantType(qvariant: &QVariant) -> QVariantType;

        #[rust_name = "qvariant_init"]
        fn qvariantInit() -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_qvariant"]
        fn qvariantInitFromQVariant(variant: &QVariant) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_bool"]
        fn qvariantInitFromBool(b: bool) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_f32"]
        fn qvariantInitFromF32(f: f32) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_f64"]
        fn qvariantInitFromF64(f: f64) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_i8"]
        fn qvariantInitFromI8(i: i8) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_i16"]
        fn qvariantInitFromI16(i: i16) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_i32"]
        fn qvariantInitFromI32(i: i32) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_qcolor"]
        fn qvariantInitFromQColor(color: &QColor) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_qdate"]
        fn qvariantInitFromQDate(date: &QDate) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_qdatetime"]
        fn qvariantInitFromQDateTime(dateTime: &QDateTime) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_qpoint"]
        fn qvariantInitFromQPoint(point: &QPoint) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_qpointf"]
        fn qvariantInitFromQPointF(pointf: &QPointF) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_qrect"]
        fn qvariantInitFromQRect(rect: &QRect) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_qrectf"]
        fn qvariantInitFromQRectF(rectf: &QRectF) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_qsize"]
        fn qvariantInitFromQSize(size: &QSize) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_qsizef"]
        fn qvariantInitFromQSizeF(sizef: &QSizeF) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_qtime"]
        fn qvariantInitFromQTime(time: &QTime) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_qurl"]
        fn qvariantInitFromQUrl(url: &QUrl) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_rust_string"]
        fn qvariantInitFromRustString(string: &str) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_u8"]
        fn qvariantInitFromU8(u: u8) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_u16"]
        fn qvariantInitFromU16(u: u16) -> UniquePtr<QVariant>;
        #[rust_name = "qvariant_init_from_u32"]
        fn qvariantInitFromU32(u: u32) -> UniquePtr<QVariant>;

        #[rust_name = "qvariant_to_bool"]
        fn qvariantToBool(qvariant: &QVariant) -> bool;
        #[rust_name = "qvariant_to_f32"]
        fn qvariantToF32(qvariant: &QVariant) -> f32;
        #[rust_name = "qvariant_to_f64"]
        fn qvariantToF64(qvariant: &QVariant) -> f64;
        #[rust_name = "qvariant_to_i8"]
        fn qvariantToI8(qvariant: &QVariant) -> i8;
        #[rust_name = "qvariant_to_i16"]
        fn qvariantToI16(qvariant: &QVariant) -> i16;
        #[rust_name = "qvariant_to_i32"]
        fn qvariantToI32(qvariant: &QVariant) -> i32;
        #[rust_name = "qvariant_to_qcolor"]
        fn qvariantToQColor(qvariant: &QVariant) -> UniquePtr<QColor>;
        #[rust_name = "qvariant_to_qdate"]
        fn qvariantToQDate(qvariant: &QVariant) -> QDate;
        #[rust_name = "qvariant_to_qdatetime"]
        fn qvariantToQDateTime(qvariant: &QVariant) -> UniquePtr<QDateTime>;
        #[rust_name = "qvariant_to_qpoint"]
        fn qvariantToQPoint(qvariant: &QVariant) -> QPoint;
        #[rust_name = "qvariant_to_qpointf"]
        fn qvariantToQPointF(qvariant: &QVariant) -> QPointF;
        #[rust_name = "qvariant_to_qrect"]
        fn qvariantToQRect(qvariant: &QVariant) -> QRect;
        #[rust_name = "qvariant_to_qrectf"]
        fn qvariantToQRectF(qvariant: &QVariant) -> QRectF;
        #[rust_name = "qvariant_to_qsize"]
        fn qvariantToQSize(qvariant: &QVariant) -> QSize;
        #[rust_name = "qvariant_to_qsizef"]
        fn qvariantToQSizeF(qvariant: &QVariant) -> QSizeF;
        #[rust_name = "qvariant_to_qtime"]
        fn qvariantToQTime(qvariant: &QVariant) -> QTime;
        #[rust_name = "qvariant_to_qurl"]
        fn qvariantToQUrl(qvariant: &QVariant) -> UniquePtr<QUrl>;
        #[rust_name = "qvariant_to_rust_string"]
        fn qvariantToRustString(qvariant: &QVariant) -> String;
        #[rust_name = "qvariant_to_u8"]
        fn qvariantToU8(qvariant: &QVariant) -> u8;
        #[rust_name = "qvariant_to_u16"]
        fn qvariantToU16(qvariant: &QVariant) -> u16;
        #[rust_name = "qvariant_to_u32"]
        fn qvariantToU32(qvariant: &QVariant) -> u32;
    }

    impl UniquePtr<QVariant> {}
}

/// The QVariant class acts like a union for the most common Qt data types.
pub type QVariant = ffi::QVariant;

impl QVariant {
    /// Create a new Rust Variant from this QVariant.
    /// This is a copy operation so any changes will not propagate to the original QVariant.
    pub fn to_rust(&self) -> Variant {
        Variant::from(self)
    }
}

pub enum VariantValue {
    Unsupported,
    Bool(bool),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    QColor(Color),
    QDate(QDate),
    QDateTime(DateTime),
    QPoint(QPoint),
    QPointF(QPointF),
    QRect(QRect),
    QRectF(QRectF),
    QSize(QSize),
    QSizeF(QSizeF),
    QTime(QTime),
    QUrl(Url),
    String(String),
    U8(u8),
    U16(u16),
    U32(u32),
}

// Define how we convert from other types into a QVariant
trait IntoQVariant {
    fn into_qvariant(self) -> cxx::UniquePtr<QVariant>;
}

macro_rules! into_qvariant {
    ($typeName:ty, $name:expr) => {
        impl IntoQVariant for $typeName {
            fn into_qvariant(self) -> cxx::UniquePtr<QVariant> {
                $name(self)
            }
        }
    };
}

macro_rules! into_qvariant_ref {
    ($typeName:ty, $name:expr) => {
        impl IntoQVariant for $typeName {
            fn into_qvariant(self) -> cxx::UniquePtr<QVariant> {
                $name(&self)
            }
        }
    };
}

macro_rules! into_qvariant_opaque {
    ($typeName:ty, $name:expr) => {
        impl IntoQVariant for $typeName {
            fn into_qvariant(self) -> cxx::UniquePtr<QVariant> {
                $name(&self.to_unique_ptr())
            }
        }
    };
}

into_qvariant!(&QVariant, ffi::qvariant_init_from_qvariant);
into_qvariant!(bool, ffi::qvariant_init_from_bool);
into_qvariant!(f32, ffi::qvariant_init_from_f32);
into_qvariant!(f64, ffi::qvariant_init_from_f64);
into_qvariant!(i8, ffi::qvariant_init_from_i8);
into_qvariant!(i16, ffi::qvariant_init_from_i16);
into_qvariant!(i32, ffi::qvariant_init_from_i32);
into_qvariant_opaque!(Color, ffi::qvariant_init_from_qcolor);
into_qvariant_ref!(QDate, ffi::qvariant_init_from_qdate);
into_qvariant_opaque!(DateTime, ffi::qvariant_init_from_qdatetime);
into_qvariant_ref!(QPoint, ffi::qvariant_init_from_qpoint);
into_qvariant_ref!(QPointF, ffi::qvariant_init_from_qpointf);
into_qvariant_ref!(QRect, ffi::qvariant_init_from_qrect);
into_qvariant_ref!(QRectF, ffi::qvariant_init_from_qrectf);
into_qvariant_ref!(QSize, ffi::qvariant_init_from_qsize);
into_qvariant_ref!(QSizeF, ffi::qvariant_init_from_qsizef);
into_qvariant_ref!(QTime, ffi::qvariant_init_from_qtime);
into_qvariant_opaque!(Url, ffi::qvariant_init_from_qurl);
into_qvariant_ref!(String, ffi::qvariant_init_from_rust_string);
into_qvariant!(u8, ffi::qvariant_init_from_u8);
into_qvariant!(u16, ffi::qvariant_init_from_u16);
into_qvariant!(u32, ffi::qvariant_init_from_u32);

/// The Rust representation of Qt's QVariant
///
/// Internally this holds a UniquePtr to a QVariant which has been constructed on the C++ side.
pub struct Variant {
    // Note that once map_qt_value is removed later, this can become private again
    pub(crate) inner: cxx::UniquePtr<QVariant>,
}

impl Default for Variant {
    fn default() -> Self {
        Variant::from_unique_ptr(ffi::qvariant_init())
    }
}

impl<T> From<T> for Variant
where
    T: IntoQVariant,
{
    fn from(t: T) -> Self {
        Self {
            inner: t.into_qvariant(),
        }
    }
}

impl Variant {
    /// Construct a Rust Variant from an existing UniquePtr<QVariant> this is a move operation
    ///
    /// This is used in Variant::default so that we don't need to perform another copy
    fn from_unique_ptr(ptr: cxx::UniquePtr<QVariant>) -> Self {
        Self { inner: ptr }
    }

    // TODO: add a set_value(&mut self, value: VariantValue);

    /// Returns the value of the QVariant
    pub fn value(&self) -> VariantValue {
        match ffi::qvariant_get_type(&self.inner) {
            ffi::QVariantType::Unsupported => VariantValue::Unsupported,
            ffi::QVariantType::Bool => VariantValue::Bool(ffi::qvariant_to_bool(&self.inner)),
            ffi::QVariantType::F32 => VariantValue::F32(ffi::qvariant_to_f32(&self.inner)),
            ffi::QVariantType::F64 => VariantValue::F64(ffi::qvariant_to_f64(&self.inner)),
            ffi::QVariantType::I8 => VariantValue::I8(ffi::qvariant_to_i8(&self.inner)),
            ffi::QVariantType::I16 => VariantValue::I16(ffi::qvariant_to_i16(&self.inner)),
            ffi::QVariantType::I32 => VariantValue::I32(ffi::qvariant_to_i32(&self.inner)),
            ffi::QVariantType::QColor => {
                VariantValue::QColor(Color::from_unique_ptr(ffi::qvariant_to_qcolor(&self.inner)))
            }
            ffi::QVariantType::QDate => VariantValue::QDate(ffi::qvariant_to_qdate(&self.inner)),
            ffi::QVariantType::QDateTime => VariantValue::QDateTime(DateTime::from_unique_ptr(
                ffi::qvariant_to_qdatetime(&self.inner),
            )),
            ffi::QVariantType::QPoint => VariantValue::QPoint(ffi::qvariant_to_qpoint(&self.inner)),
            ffi::QVariantType::QPointF => {
                VariantValue::QPointF(ffi::qvariant_to_qpointf(&self.inner))
            }
            ffi::QVariantType::QRect => VariantValue::QRect(ffi::qvariant_to_qrect(&self.inner)),
            ffi::QVariantType::QRectF => VariantValue::QRectF(ffi::qvariant_to_qrectf(&self.inner)),
            ffi::QVariantType::QSize => VariantValue::QSize(ffi::qvariant_to_qsize(&self.inner)),
            ffi::QVariantType::QSizeF => VariantValue::QSizeF(ffi::qvariant_to_qsizef(&self.inner)),
            ffi::QVariantType::QTime => VariantValue::QTime(ffi::qvariant_to_qtime(&self.inner)),
            ffi::QVariantType::QUrl => {
                VariantValue::QUrl(Url::from_unique_ptr(ffi::qvariant_to_qurl(&self.inner)))
            }
            ffi::QVariantType::String => {
                VariantValue::String(ffi::qvariant_to_rust_string(&self.inner))
            }
            ffi::QVariantType::U8 => VariantValue::U8(ffi::qvariant_to_u8(&self.inner)),
            ffi::QVariantType::U16 => VariantValue::U16(ffi::qvariant_to_u16(&self.inner)),
            ffi::QVariantType::U32 => VariantValue::U32(ffi::qvariant_to_u32(&self.inner)),
            _others => VariantValue::Unsupported,
        }
    }
}

impl crate::ToUniquePtr for Variant {
    type CppType = QVariant;

    /// Retrieve the UniquePtr to the Qt QVariant of this Rust Variant
    /// so that this object can be passed back to C++.
    fn to_unique_ptr(self) -> cxx::UniquePtr<QVariant> {
        self.inner
    }
}
