// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// We are only using references to QVariant so it is actually ffi safe as far as we are concerned
#![allow(improper_ctypes)]

use crate::{
    Color, DateTime, QColor, QDate, QDateTime, QPoint, QPointF, QRect, QRectF, QSize, QSizeF,
    QTime, QUrl, ToUniquePtr, Url,
};
use cxx::{memory::UniquePtrTarget, type_id, ExternType};
use std::{
    ffi::c_void,
    marker::{PhantomData, PhantomPinned},
    mem::MaybeUninit,
};

#[repr(u8)]
#[allow(dead_code)]
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

extern "C" {
    #[link_name = "cxxqt1$qvariant$init$from$qvariant"]
    fn qvariant_init_from_qvariant(
        ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>,
        qvariant: &QVariant,
    );
    #[link_name = "cxxqt1$qvariant$init$from$bool"]
    fn qvariant_init_from_bool(ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, b: bool);
    #[link_name = "cxxqt1$qvariant$init$from$f32"]
    fn qvariant_init_from_f32(ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, i: f32);
    #[link_name = "cxxqt1$qvariant$init$from$f64"]
    fn qvariant_init_from_f64(ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, i: f64);
    #[link_name = "cxxqt1$qvariant$init$from$i8"]
    fn qvariant_init_from_i8(ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, i: i8);
    #[link_name = "cxxqt1$qvariant$init$from$i16"]
    fn qvariant_init_from_i16(ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, i: i16);
    #[link_name = "cxxqt1$qvariant$init$from$i32"]
    fn qvariant_init_from_i32(ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, i: i32);
    #[link_name = "cxxqt1$qvariant$init$from$qcolor"]
    fn qvariant_init_from_qcolor(this: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, color: &QColor);
    #[link_name = "cxxqt1$qvariant$init$from$qdate"]
    fn qvariant_init_from_qdate(this: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, date: &QDate);
    #[link_name = "cxxqt1$qvariant$init$from$qdatetime"]
    fn qvariant_init_from_qdatetime(
        this: &mut MaybeUninit<cxx::UniquePtr<QVariant>>,
        date_time: &QDateTime,
    );
    #[link_name = "cxxqt1$qvariant$init$from$qpoint"]
    fn qvariant_init_from_qpoint(this: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, point: &QPoint);
    #[link_name = "cxxqt1$qvariant$init$from$qpointf"]
    fn qvariant_init_from_qpointf(
        this: &mut MaybeUninit<cxx::UniquePtr<QVariant>>,
        pointf: &QPointF,
    );
    #[link_name = "cxxqt1$qvariant$init$from$qrect"]
    fn qvariant_init_from_qrect(this: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, rect: &QRect);
    #[link_name = "cxxqt1$qvariant$init$from$qrectf"]
    fn qvariant_init_from_qrectf(this: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, rectf: &QRectF);
    #[link_name = "cxxqt1$qvariant$init$from$qsize"]
    fn qvariant_init_from_qsize(this: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, size: &QSize);
    #[link_name = "cxxqt1$qvariant$init$from$qsizef"]
    fn qvariant_init_from_qsizef(this: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, sizef: &QSizeF);
    #[link_name = "cxxqt1$qvariant$init$from$qtime"]
    fn qvariant_init_from_qtime(this: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, time: &QTime);
    #[link_name = "cxxqt1$qvariant$init$from$qurl"]
    fn qvariant_init_from_qurl(this: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, url: &QUrl);
    #[link_name = "cxxqt1$qvariant$init$from$str"]
    fn qvariant_init_from_str(ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, s: &str);
    #[link_name = "cxxqt1$qvariant$init$from$u8"]
    fn qvariant_init_from_u8(ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, i: u8);
    #[link_name = "cxxqt1$qvariant$init$from$u16"]
    fn qvariant_init_from_u16(ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, i: u16);
    #[link_name = "cxxqt1$qvariant$init$from$u32"]
    fn qvariant_init_from_u32(ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>, i: u32);
    #[link_name = "cxxqt1$qvariant$get$type"]
    fn qvariant_get_type(this: &QVariant) -> QVariantType;
    #[link_name = "cxxqt1$qvariant$to$bool"]
    fn qvariant_to_bool(this: &QVariant) -> bool;
    #[link_name = "cxxqt1$qvariant$to$f32"]
    fn qvariant_to_f32(this: &QVariant) -> f32;
    #[link_name = "cxxqt1$qvariant$to$f64"]
    fn qvariant_to_f64(this: &QVariant) -> f64;
    #[link_name = "cxxqt1$qvariant$to$i8"]
    fn qvariant_to_i8(this: &QVariant) -> i8;
    #[link_name = "cxxqt1$qvariant$to$i16"]
    fn qvariant_to_i16(this: &QVariant) -> i16;
    #[link_name = "cxxqt1$qvariant$to$i32"]
    fn qvariant_to_i32(this: &QVariant) -> i32;
    // Note that we cannot return QColor as it's opaque so we use the UniquePtr pattern
    #[link_name = "cxxqt1$qvariant$to$qcolor"]
    fn qvariant_to_qcolor(this: &QVariant, ptr: &mut MaybeUninit<cxx::UniquePtr<QColor>>);
    // Note that we cannot return QDateTime as it's opaque so we use the UniquePtr pattern
    #[link_name = "cxxqt1$qvariant$to$qdatetime"]
    fn qvariant_to_qdatetime(this: &QVariant, ptr: &mut MaybeUninit<cxx::UniquePtr<QDateTime>>);
    #[link_name = "cxxqt1$qvariant$to$qdate"]
    fn qvariant_to_qdate(this: &QVariant) -> QDate;
    #[link_name = "cxxqt1$qvariant$to$qpoint"]
    fn qvariant_to_qpoint(this: &QVariant) -> QPoint;
    #[link_name = "cxxqt1$qvariant$to$qpointf"]
    fn qvariant_to_qpointf(this: &QVariant) -> QPointF;
    #[link_name = "cxxqt1$qvariant$to$qrect"]
    fn qvariant_to_qrect(this: &QVariant) -> QRect;
    #[link_name = "cxxqt1$qvariant$to$qrectf"]
    fn qvariant_to_qrectf(this: &QVariant) -> QRectF;
    #[link_name = "cxxqt1$qvariant$to$qsize"]
    fn qvariant_to_qsize(this: &QVariant) -> QSize;
    #[link_name = "cxxqt1$qvariant$to$qsizef"]
    fn qvariant_to_qsizef(this: &QVariant) -> QSizeF;
    #[link_name = "cxxqt1$qvariant$to$qtime"]
    fn qvariant_to_qtime(this: &QVariant) -> QTime;
    // Note that we cannot return QUrl as it's opaque so we use the UniquePtr pattern
    #[link_name = "cxxqt1$qvariant$to$qurl"]
    fn qvariant_to_qurl(this: &QVariant, ptr: &mut MaybeUninit<cxx::UniquePtr<QUrl>>);
    #[link_name = "cxxqt1$qvariant$copy$to$string"]
    fn qvariant_copy_to_string(this: &QVariant, s: &mut String);
    #[link_name = "cxxqt1$qvariant$to$u8"]
    fn qvariant_to_u8(this: &QVariant) -> u8;
    #[link_name = "cxxqt1$qvariant$to$u16"]
    fn qvariant_to_u16(this: &QVariant) -> u16;
    #[link_name = "cxxqt1$qvariant$to$u32"]
    fn qvariant_to_u32(this: &QVariant) -> u32;
}

/// Binding to Qt `QVariant`.
///
/// # Invariants
///
/// As an invariant of this API and the static analysis of the cxx::bridge
/// macro, in Rust code we can never obtain a `QVariant` by value. Qt's QVariant
/// requires a move constructor and may hold internal pointers, which is not
/// compatible with Rust's move behavior. Instead in Rust code we will only ever
/// look at a QVariant through a reference or smart pointer, as in `&QVariant`
/// or `UniquePtr<QVariant>`.
#[repr(C)]
pub struct QVariant {
    _pinned: PhantomData<PhantomPinned>,
}

impl QVariant {
    /// Create a new Rust Variant from this QVariant.
    /// This is a copy operation so any changes will not propagate to the original QVariant.
    pub fn to_rust(&self) -> Variant {
        Variant::from(self)
    }
}

// Safety:
//
// The code in this file ensures that QVariant can only ever be allocated
// on the stack in pinned form which avoids the pitfalls of trying to
// move this type that has a non-trivial move constructor.
unsafe impl ExternType for QVariant {
    type Id = type_id!("QVariant");
    type Kind = cxx::kind::Opaque;
}

extern "C" {
    #[link_name = "cxxqt1$unique_ptr$qvariant$null"]
    fn unique_ptr_qvariant_null(this: *mut MaybeUninit<*mut c_void>);
    #[link_name = "cxxqt1$unique_ptr$qvariant$raw"]
    fn unique_ptr_qvariant_raw(this: *mut MaybeUninit<*mut c_void>, raw: *mut QVariant);
    #[link_name = "cxxqt1$unique_ptr$qvariant$get"]
    fn unique_ptr_qvariant_get(this: *const MaybeUninit<*mut c_void>) -> *const QVariant;
    #[link_name = "cxxqt1$unique_ptr$qvariant$release"]
    fn unique_ptr_qvariant_release(this: *mut MaybeUninit<*mut c_void>) -> *mut QVariant;
    #[link_name = "cxxqt1$unique_ptr$qvariant$drop"]
    fn unique_ptr_qvariant_drop(this: *mut MaybeUninit<*mut c_void>);
}

unsafe impl UniquePtrTarget for QVariant {
    #[doc(hidden)]
    fn __typename(f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("QVariant")
    }

    #[doc(hidden)]
    fn __null() -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unsafe {
            unique_ptr_qvariant_null(&mut repr);
        }
        repr
    }

    #[doc(hidden)]
    unsafe fn __raw(raw: *mut Self) -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unique_ptr_qvariant_raw(&mut repr, raw);
        repr
    }

    #[doc(hidden)]
    unsafe fn __get(repr: MaybeUninit<*mut c_void>) -> *const Self {
        unique_ptr_qvariant_get(&repr)
    }

    #[doc(hidden)]
    unsafe fn __release(mut repr: MaybeUninit<*mut c_void>) -> *mut Self {
        unique_ptr_qvariant_release(&mut repr)
    }

    #[doc(hidden)]
    unsafe fn __drop(mut repr: MaybeUninit<*mut c_void>) {
        unique_ptr_qvariant_drop(&mut repr)
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
pub trait IntoQVariant {
    unsafe fn into_qvariant(self, ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>);
}

macro_rules! into_qvariant {
    ($typeName:ty, $name:ident) => {
        impl IntoQVariant for $typeName {
            unsafe fn into_qvariant(self, ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>) {
                $name(ptr, self);
            }
        }
    };
}

macro_rules! into_qvariant_ref {
    ($typeName:ty, $name:ident) => {
        impl IntoQVariant for $typeName {
            unsafe fn into_qvariant(self, ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>) {
                $name(ptr, &self);
            }
        }
    };
}

macro_rules! into_qvariant_opaque {
    ($typeName:ty, $name:ident) => {
        impl IntoQVariant for $typeName {
            unsafe fn into_qvariant(self, ptr: &mut MaybeUninit<cxx::UniquePtr<QVariant>>) {
                $name(ptr, &self.to_unique_ptr());
            }
        }
    };
}

into_qvariant!(&QVariant, qvariant_init_from_qvariant);
into_qvariant!(bool, qvariant_init_from_bool);
into_qvariant!(f32, qvariant_init_from_f32);
into_qvariant!(f64, qvariant_init_from_f64);
into_qvariant!(i8, qvariant_init_from_i8);
into_qvariant!(i16, qvariant_init_from_i16);
into_qvariant!(i32, qvariant_init_from_i32);
into_qvariant_opaque!(Color, qvariant_init_from_qcolor);
into_qvariant_ref!(QDate, qvariant_init_from_qdate);
into_qvariant_opaque!(DateTime, qvariant_init_from_qdatetime);
into_qvariant_ref!(QPoint, qvariant_init_from_qpoint);
into_qvariant_ref!(QPointF, qvariant_init_from_qpointf);
into_qvariant_ref!(QRect, qvariant_init_from_qrect);
into_qvariant_ref!(QRectF, qvariant_init_from_qrectf);
into_qvariant_ref!(QSize, qvariant_init_from_qsize);
into_qvariant_ref!(QSizeF, qvariant_init_from_qsizef);
into_qvariant_ref!(QTime, qvariant_init_from_qtime);
into_qvariant_opaque!(Url, qvariant_init_from_qurl);
into_qvariant_ref!(String, qvariant_init_from_str);
into_qvariant!(u8, qvariant_init_from_u8);
into_qvariant!(u16, qvariant_init_from_u16);
into_qvariant!(u32, qvariant_init_from_u32);

pub struct Variant {
    pub(crate) inner: cxx::UniquePtr<QVariant>,
}

impl<T> From<T> for Variant
where
    T: IntoQVariant,
{
    fn from(t: T) -> Self {
        Self {
            inner: unsafe {
                let mut ptr = MaybeUninit::<cxx::UniquePtr<QVariant>>::zeroed();
                t.into_qvariant(&mut ptr);
                ptr.assume_init()
            },
        }
    }
}

impl Variant {
    // TODO: add a set_value(&mut self, value: VariantValue);

    pub fn value(&self) -> VariantValue {
        match unsafe { qvariant_get_type(&self.inner) } {
            QVariantType::Unsupported => VariantValue::Unsupported,
            QVariantType::Bool => VariantValue::Bool(unsafe { qvariant_to_bool(&self.inner) }),
            QVariantType::F32 => VariantValue::F32(unsafe { qvariant_to_f32(&self.inner) }),
            QVariantType::F64 => VariantValue::F64(unsafe { qvariant_to_f64(&self.inner) }),
            QVariantType::I8 => VariantValue::I8(unsafe { qvariant_to_i8(&self.inner) }),
            QVariantType::I16 => VariantValue::I16(unsafe { qvariant_to_i16(&self.inner) }),
            QVariantType::I32 => VariantValue::I32(unsafe { qvariant_to_i32(&self.inner) }),
            QVariantType::QColor => VariantValue::QColor(unsafe {
                let mut ptr = MaybeUninit::<cxx::UniquePtr<QColor>>::zeroed();
                qvariant_to_qcolor(&self.inner, &mut ptr);
                Color::from_unique_ptr(ptr.assume_init())
            }),
            QVariantType::QDate => VariantValue::QDate(unsafe { qvariant_to_qdate(&self.inner) }),
            QVariantType::QDateTime => VariantValue::QDateTime(unsafe {
                let mut ptr = MaybeUninit::<cxx::UniquePtr<QDateTime>>::zeroed();
                qvariant_to_qdatetime(&self.inner, &mut ptr);
                DateTime::from_unique_ptr(ptr.assume_init())
            }),
            QVariantType::QPoint => {
                VariantValue::QPoint(unsafe { qvariant_to_qpoint(&self.inner) })
            }
            QVariantType::QPointF => {
                VariantValue::QPointF(unsafe { qvariant_to_qpointf(&self.inner) })
            }
            QVariantType::QRect => VariantValue::QRect(unsafe { qvariant_to_qrect(&self.inner) }),
            QVariantType::QRectF => {
                VariantValue::QRectF(unsafe { qvariant_to_qrectf(&self.inner) })
            }
            QVariantType::QSize => VariantValue::QSize(unsafe { qvariant_to_qsize(&self.inner) }),
            QVariantType::QSizeF => {
                VariantValue::QSizeF(unsafe { qvariant_to_qsizef(&self.inner) })
            }
            QVariantType::QTime => VariantValue::QTime(unsafe { qvariant_to_qtime(&self.inner) }),
            QVariantType::QUrl => VariantValue::QUrl(unsafe {
                let mut ptr = MaybeUninit::<cxx::UniquePtr<QUrl>>::zeroed();
                qvariant_to_qurl(&self.inner, &mut ptr);
                Url::from_unique_ptr(ptr.assume_init())
            }),
            QVariantType::String => {
                let mut s = String::new();
                unsafe { qvariant_copy_to_string(&self.inner, &mut s) };
                VariantValue::String(s)
            }
            QVariantType::U8 => VariantValue::U8(unsafe { qvariant_to_u8(&self.inner) }),
            QVariantType::U16 => VariantValue::U16(unsafe { qvariant_to_u16(&self.inner) }),
            QVariantType::U32 => VariantValue::U32(unsafe { qvariant_to_u32(&self.inner) }),
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

unsafe impl ExternType for Variant {
    type Id = type_id!("CxxQt::Variant");
    type Kind = cxx::kind::Opaque;
}
